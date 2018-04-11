export {
  Matrix,
  matrixMultiplication,
  strassen,
};

type Elem = number;
type Matrix = Elem[][];
type Quad<A> = [A, A, A, A];

function matrixMultiplication(A: Matrix, B: Matrix): Matrix {
  let n = A.length;
  //  used to be
  //    let C = new Array(n).fill([]);
  //  the argument of .fill is evaluated only once
  //  therefore all rows contain the reference to the SAME array
  //  should be careful about the difference between reference and value types in javascript/typescript
  let C = new Array(n);
  for (let i = 0; i < n; i++) {
    C[i] = [];
    for (let j = 0; j < n; j++) {
      let sum = 0;
      for (let k = 0; k < n; k++) {
        sum += A[i][k] * B[k][j];
      }
      C[i][j] = sum;
    }
  }
  return C;
}

function matrixPointwise(A: Matrix, B: Matrix, f: (lhs: Elem, rhs: Elem) => Elem): Matrix {
  let n = A.length;
  let C = emptyMatrix(n);
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
      C[i][j] = f(A[i][j], B[i][j]);
    }
  }
  return C;
}

function mAdd(A: Matrix, B: Matrix): Matrix {
  return matrixPointwise(A, B, (a, b) => a + b);
}

function mSub(A: Matrix, B: Matrix): Matrix {
  return matrixPointwise(A, B, (a, b) => a - b);
}

function emptyMatrix(n: number): Matrix {
  let A = new Array(n);
  for (let i = 0; i < n; i++) {
    A[i] = [];
  }
  return A;
}

// not O(1), but won't affect the overall complexity
function divide(A: Matrix): Quad<Matrix> {
  let n = A.length;
  let half = n / 2;
  let A11 = emptyMatrix(half);
  let A12 = emptyMatrix(half);
  let A21 = emptyMatrix(half);
  let A22 = emptyMatrix(half);
  for (let i = 0; i < half; i++) {
    for (let j = 0; j < half; j++) {
      A11[i][j] = A[i][j];
      A12[i][j] = A[i][j + half];
      A21[i][j] = A[i + half][j];
      A22[i][j] = A[i + half][j + half];
    }
  }
  return [A11, A12, A21, A22];
}

function conquer(quad: Quad<Matrix>): Matrix {
  let [A11, A12, A21, A22] = quad;
  let half = A11.length;
  let n = half * 2;
  let A = emptyMatrix(n);

  for (let i = 0; i < half; i++) {
    for (let j = 0; j < half; j++) {
      A[i][j] = A11[i][j];
      A[i][j + half] = A12[i][j];
      A[i + half][j] = A21[i][j];
      A[i + half][j + half] = A22[i][j];
    }
  }

  return A;
}
function strassen(A: Matrix, B: Matrix): Matrix {
  let n = A.length;
  if (n === 1) {
    let C = emptyMatrix(1);
    C[0][0] = A[0][0] * B[0][0];
    return C;
  } else {
    let [A11, A12, A21, A22] = divide(A);
    let [B11, B12, B21, B22] = divide(B);
    let S1 = mSub(B12, B22);
    let S2 = mAdd(A11, A12);
    let S3 = mAdd(A21, A22);
    let S4 = mSub(B21, B11);
    let S5 = mAdd(A11, A22);
    let S6 = mAdd(B11, B22);
    let S7 = mSub(A12, A22);
    let S8 = mAdd(B21, B22);
    let S9 = mSub(A11, A21);
    let S10 = mAdd(B11, B12);
    let P1 = strassen(A11, S1);
    let P2 = strassen(S2, B22);
    let P3 = strassen(S3, B11);
    let P4 = strassen(A22, S4);
    let P5 = strassen(S5, S6);
    let P6 = strassen(S7, S8);
    let P7 = strassen(S9, S10);
    let C11 = mAdd(mSub(mAdd(P5, P4), P2), P6);
    let C12 = mAdd(P1, P2);
    let C21 = mAdd(P3, P4);
    let C22 = mSub(mSub(mAdd(P5, P1), P3), P7);
    return conquer([C11, C12, C21, C22]);
  }
}
