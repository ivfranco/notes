export {
  pMatrixMultiply,
  pMatrixMultiplyDivide,
  pMatrixMultiplyDivide2,
  pStrassen,
  pTranspose,
  pFloydWarshall,
  parallelFor,
};

type Matrix = number[][];

const MIN_SIZE = 1;

function linearSum(A: Matrix, B: Matrix, i: number, j: number, p: number, q: number): number {
  let s = 0;
  for (let k = p; k <= q; k++) {
    s += A[i][k] * B[k][j];
  }
  return s;
}

async function parallelFor(low: number, high: number, f: (i: number) => Promise<void>) {
  if (high - low + 1 <= MIN_SIZE) {
    for (let i = low; i <= high; i++) {
      await f(i);
    }
  } else {
    let mid = Math.floor((low + high) / 2);
    let low_p = parallelFor(low, mid, f);
    await parallelFor(mid + 1, high, f);
    await low_p;
  }
}

async function parallelSum(A: Matrix, B: Matrix, i: number, j: number, p: number, q: number): Promise<number> {
  if (q - p <= MIN_SIZE) {
    return linearSum(A, B, i, j, p, q);
  } else {
    let mid = Math.floor((p + q) / 2);
    let left_p = parallelSum(A, B, i, j, p, mid);
    let right_sum = await parallelSum(A, B, i, j, mid + 1, q);
    let left_sum = await left_p;
    return left_sum + right_sum;
  }
}

async function pMatrixMultiply(A: Matrix, B: Matrix): Promise<Matrix> {
  let p = A.length;
  let q = B.length;
  let r = B[0].length;
  let C: Matrix = [];
  await parallelFor(0, p - 1, async i => {
    C[i] = [];
    await parallelFor(0, r - 1, async j => {
      C[i][j] = await parallelSum(A, B, i, j, 0, q - 1);
    });
  });
  return C;
}

type Quad<A> = [A, A, A, A];

class SubMatrix {
  public orig: Matrix;
  public top: number;
  public left: number;
  public right: number;
  public bottom: number;

  constructor(A: Matrix) {
    let n = A.length;
    this.orig = A;
    this.top = 0;
    this.left = 0;
    this.right = n - 1;
    this.bottom = n - 1;
  }

  public dimension(): number {
    return this.right - this.left + 1;
  }

  public get(i: number, j: number): number {
    return this.orig[this.top + i][this.left + j];
  }

  public set(i: number, j: number, v: number) {
    this.orig[this.top + i][this.left + j] = v;
  }

  public shrink(top: number, left: number, right: number, bottom: number): SubMatrix {
    console.assert(this.top <= top);
    console.assert(this.left <= left);
    console.assert(this.right >= right);
    console.assert(this.bottom >= bottom);

    let sub_matrix = new SubMatrix(this.orig);
    sub_matrix.top = top;
    sub_matrix.left = left;
    sub_matrix.right = right;
    sub_matrix.bottom = bottom;
    return sub_matrix;
  }

  //  inplace pairwise operation, returns this sub matrix for chaining
  public async pairwise(B: SubMatrix, op: (a: number, b: number) => number): Promise<this> {
    let n = this.dimension();
    await parallelFor(0, n - 1, async i => {
      await parallelFor(0, n - 1, async j => {
        this.set(i, j, op(this.get(i, j), B.get(i, j)));
      });
    });
    return this;
  }

  public async add(B: SubMatrix): Promise<this> {
    return this.pairwise(B, (a, b) => a + b);
  }

  public async sub(B: SubMatrix): Promise<this> {
    return this.pairwise(B, (a, b) => a - b);
  }

  public partition(): Quad<SubMatrix> {
    let { orig, top, left, right, bottom } = this;
    let horiz_mid = Math.floor((left + right) / 2);
    let verti_mid = Math.floor((top + bottom) / 2);
    let A11 = this.shrink(top, left, horiz_mid, verti_mid);
    let A12 = this.shrink(top, horiz_mid + 1, right, verti_mid);
    let A21 = this.shrink(verti_mid + 1, left, horiz_mid, bottom);
    let A22 = this.shrink(verti_mid + 1, horiz_mid + 1, right, bottom);
    return [A11, A12, A21, A22];
  }
}

async function emptySubMatrix(n: number): Promise<SubMatrix> {
  let A: Matrix = [];
  await parallelFor(0, n - 1, async i => {
    A[i] = [];
    await parallelFor(0, n - 1, async j => {
      A[i][j] = 0;
    });
  });
  return new SubMatrix(A);
}

async function pMatrixMultiplyDivide(A: Matrix, B: Matrix): Promise<Matrix> {
  let SA = new SubMatrix(A);
  let SB = new SubMatrix(B);
  let SC = await emptySubMatrix(A.length);
  await pMatrixMultiplyRecursive(SA, SB, SC);
  return SC.orig;
}

async function pMatrixMultiplyRecursive(A: SubMatrix, B: SubMatrix, C: SubMatrix): Promise<void> {
  if (C.dimension() === 1) {
    C.set(0, 0, A.get(0, 0) * B.get(0, 0));
  } else {
    let n = C.dimension();
    let T = await emptySubMatrix(n);

    let [A11, A12, A21, A22] = A.partition();
    let [B11, B12, B21, B22] = B.partition();
    let [C11, C12, C21, C22] = C.partition();
    let [T11, T12, T21, T22] = T.partition();

    let handles = [
      pMatrixMultiplyRecursive(A11, B11, C11),
      pMatrixMultiplyRecursive(A11, B12, C12),
      pMatrixMultiplyRecursive(A21, B11, C21),
      pMatrixMultiplyRecursive(A21, B12, C22),
      pMatrixMultiplyRecursive(A12, B21, T11),
      pMatrixMultiplyRecursive(A12, B22, T12),
      pMatrixMultiplyRecursive(A22, B21, T21),
      pMatrixMultiplyRecursive(A22, B22, T22),
    ];
    await Promise.all(handles);

    await parallelFor(0, n - 1, async i => {
      await parallelFor(0, n - 1, async j => {
        C.set(i, j, C.get(i, j) + T.get(i, j));
      });
    });
  }
}

async function pMatrixMultiplyDivide2(A: Matrix, B: Matrix): Promise<Matrix> {
  let SA = new SubMatrix(A);
  let SB = new SubMatrix(B);
  let SC = await emptySubMatrix(A.length);
  await pMatrixMultiplyAndAdd(SA, SB, SC);
  return SC.orig;
}

async function pMatrixMultiplyAndAdd(A: SubMatrix, B: SubMatrix, C: SubMatrix): Promise<void> {
  if (C.dimension() === 1) {
    C.set(0, 0, C.get(0, 0) + A.get(0, 0) * B.get(0, 0));
  } else {
    let n = C.dimension();
    let [A11, A12, A21, A22] = A.partition();
    let [B11, B12, B21, B22] = B.partition();
    let [C11, C12, C21, C22] = C.partition();

    let handles = [
      pMatrixMultiplyAndAdd(A11, B11, C11),
      pMatrixMultiplyAndAdd(A11, B12, C12),
      pMatrixMultiplyAndAdd(A21, B11, C21),
      pMatrixMultiplyAndAdd(A21, B12, C22),
    ];
    await Promise.all(handles);
    handles = [
      pMatrixMultiplyAndAdd(A12, B21, C11),
      pMatrixMultiplyAndAdd(A12, B22, C12),
      pMatrixMultiplyAndAdd(A22, B21, C21),
      pMatrixMultiplyAndAdd(A22, B22, C22),
    ];
    await Promise.all(handles);
  }
}

async function pStrassen(A: Matrix, B: Matrix): Promise<Matrix> {
  let SA = new SubMatrix(A);
  let SB = new SubMatrix(B);
  let SC = await emptySubMatrix(A.length);
  await pStrassenRecursive(SA, SB, SC);
  return SC.orig;
}

async function pStrassenRecursive(A: SubMatrix, B: SubMatrix, C: SubMatrix): Promise<void> {
  if (C.dimension() === 1) {
    C.set(0, 0, A.get(0, 0) * B.get(0, 0));
  } else {
    let half = C.dimension() / 2;
    let [A11, A12, A21, A22] = A.partition();
    let [B11, B12, B21, B22] = B.partition();
    let [C11, C12, C21, C22] = C.partition();

    let S_handles = [
      emptySubMatrix(half).then(m => m.add(B12)).then(m => m.sub(B22)),
      emptySubMatrix(half).then(m => m.add(A11)).then(m => m.add(A12)),
      emptySubMatrix(half).then(m => m.add(A21)).then(m => m.add(A22)),
      emptySubMatrix(half).then(m => m.add(B21)).then(m => m.sub(B11)),
      emptySubMatrix(half).then(m => m.add(A11)).then(m => m.add(A22)),
      emptySubMatrix(half).then(m => m.add(B11)).then(m => m.add(B22)),
      emptySubMatrix(half).then(m => m.add(A12)).then(m => m.sub(A22)),
      emptySubMatrix(half).then(m => m.add(B21)).then(m => m.add(B22)),
      emptySubMatrix(half).then(m => m.add(A11)).then(m => m.sub(A21)),
      emptySubMatrix(half).then(m => m.add(B11)).then(m => m.add(B12)),
    ];
    let [S1, S2, S3, S4, S5, S6, S7, S8, S9, S10] = await Promise.all(S_handles);

    let P_handles: Array<Promise<SubMatrix>> = [];
    for (let i = 0; i < 7; i++) {
      P_handles[i] = emptySubMatrix(half);
    }
    let [P1, P2, P3, P4, P5, P6, P7] = await Promise.all(P_handles);

    let recur_handles = [
      pStrassenRecursive(A11, S1, P1),
      pStrassenRecursive(S2, B22, P2),
      pStrassenRecursive(S3, B11, P3),
      pStrassenRecursive(A22, S4, P4),
      pStrassenRecursive(S5, S6, P5),
      pStrassenRecursive(S7, S8, P6),
      pStrassenRecursive(S9, S10, P7),
    ];
    await Promise.all(recur_handles);

    await C11
      .add(P5)
      .then(c => c.add(P4))
      .then(c => c.sub(P2))
      .then(c => c.add(P6));
    await C12
      .add(P1)
      .then(c => c.add(P2));
    await C21
      .add(P3)
      .then(c => c.add(P4));
    await C22
      .add(P5)
      .then(c => c.add(P1))
      .then(c => c.sub(P3))
      .then(c => c.sub(P7));
  }
}

async function pTranspose(A: Matrix): Promise<void> {
  let SA = new SubMatrix(A);
  await pTransposeRecursive(SA);
}

async function pTransposeRecursive(A: SubMatrix): Promise<void> {
  if (A.dimension() > 1) {
    let half = A.dimension() / 2;
    let [A11, A12, A21, A22] = A.partition();
    let handles = [
      pTransposeRecursive(A11),
      pTransposeRecursive(A12),
      pTransposeRecursive(A21),
      pTransposeRecursive(A22),
    ];
    await Promise.all(handles);

    await parallelFor(0, half - 1, async i => {
      await parallelFor(0, half - 1, async j => {
        let temp = A12.get(i, j);
        A12.set(i, j, A21.get(i, j));
        A21.set(i, j, temp);
      });
    });
  }
}

//  T1 = O(V^2), T∞ = O(lgV)
async function copy(A: Matrix, B: Matrix) {
  let n = B.length;
  let m = B[0].length;

  await parallelFor(0, n - 1, async i => {
    A[i] = [];
    await parallelFor(0, m - 1, async j => {
      A[i][j] = B[i][j];
    });
  });
}

async function pFloydWarshall(W: Matrix): Promise<Matrix> {
  let n = W.length;
  let D: Matrix = [];
  await copy(D, W);
  //  transform W to the form used in the book
  parallelFor(0, n - 1, async i => {
    D[i][i] = 0;
  });

  for (let k = 0; k < n; k++) {
    parallelFor(0, n - 1, async i => {
      await parallelFor(0, n - 1, async j => {
        D[i][j] = Math.min(D[i][j], D[i][k] + D[k][j]);
      });
    });
  }

  return D;
}
