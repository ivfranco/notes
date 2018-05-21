export {
  pLuDecomposition,
  pLupDecomposition,
  pLupSolve,
  pInverse,
};

import { multiplyByVector } from "../matrix/lup-decomposition";
import { SWAP } from "../util";
import { pFoldMap } from "./p-loop";
import { Matrix, parallelFor, pMatrixMultiply, pMatrixPairwise, SubMatrix } from "./p-matrix-multiply";

type Vector = number[];

async function initLU(n: number): Promise<[Matrix, Matrix]> {
  let L: Matrix = [];
  let U: Matrix = [];
  await parallelFor(0, n - 1, async i => {
    L[i] = [];
    U[i] = [];
    await parallelFor(0, n - 1, async j => {
      if (i === j) {
        L[i][j] = 1;
      } else if (i < j) {
        L[i][j] = 0;
      } else {
        U[i][j] = 0;
      }
    });
  });

  return [L, U];
}

async function pLuDecomposition(A: Matrix): Promise<[Matrix, Matrix]> {
  let n = A.length;
  let [L, U] = await initLU(n);

  for (let k = 0; k < n; k++) {
    U[k][k] = A[k][k];
    await parallelFor(k + 1, n - 1, async i => {
      L[i][k] = A[i][k] / U[k][k];
      U[k][i] = A[k][i];
    });
    await parallelFor(k + 1, n - 1, async i => {
      await parallelFor(k + 1, n - 1, async j => {
        A[i][j] -= L[i][k] * U[k][j];
      });
    });
  }

  return [L, U];
}

async function maximumIndex(v: Vector, i: number, j: number): Promise<number> {
  let [max, idx] = await pFoldMap(v, i, j, (a, k) => [a, k] as [number, number], (a, b) => {
    if (b[0] > a[0]) {
      return b;
    } else {
      return a;
    }
  });
  return idx;
}

async function pLupDecomposition(A: Matrix): Promise<Vector> {
  let n = A.length;
  let P: Vector = [];
  await parallelFor(0, n - 1, async i => {
    P[i] = i;
  });

  for (let k = 0; k < n; k++) {
    let col: Vector = [];
    await parallelFor(k, n - 1, async i => {
      col[k] = A[i][k];
    });
    let new_k = await maximumIndex(col, k, n - 1);
    SWAP(P, k, new_k);
    SWAP(A, k, new_k);
    await parallelFor(k + 1, n - 1, async i => {
      A[i][k] = A[i][k] / A[k][k];
      await parallelFor(k + 1, n - 1, async j => {
        A[i][j] -= A[i][k] * A[k][j];
      });
    });
  }

  return P;
}

async function pForwardSubst(L: Matrix, b: Vector): Promise<Vector> {
  let n = L.length;
  let x: Vector = [];
  x[0] = b[0];
  for (let i = 1; i < n; i++) {
    let sum = await pFoldMap(L[i], 0, i - 1, (a, j) => a * x[j as number], (lhs, rhs) => lhs + rhs);
    x[i] = b[i] - sum;
  }
  return x;
}

async function pBackwardSubst(U: Matrix, b: Vector): Promise<Vector> {
  let n = U.length;
  let x: Vector = [];
  x[n - 1] = b[n - 1] / U[n - 1][n - 1];
  for (let i = n - 2; i >= 0; i--) {
    let sum = await pFoldMap(U[i], i + 1, n - 1, (a, j) => a * x[j as number], (lhs, rhs) => lhs + rhs);
    x[i] = (b[i] - sum) / U[i][i];
  }

  return x;
}

async function pPermute(P: Vector, b: Vector): Promise<Vector> {
  let n = b.length;
  let permuted: Vector = [];
  await parallelFor(0, n - 1, async i => {
    permuted[i] = b[P[i]];
  });
  return permuted;
}

async function pLupSolve(L: Matrix, U: Matrix, P: Vector, b: Vector): Promise<Vector> {
  let y = await pForwardSubst(L, await pPermute(P, b));
  return pBackwardSubst(U, y);
}

async function pTranspose(A: Matrix): Promise<Matrix> {
  let n = A.length;
  let T: Matrix = [];
  await parallelFor(0, n - 1, async i => {
    T[i] = [];
    await parallelFor(0, n - 1, async j => {
      T[i][j] = A[j][i];
    });
  });
  return T;
}

async function pInverse(A: Matrix): Promise<Matrix> {
  let T = await pTranspose(A);
  let Inv = await pInverseRecursive(await pMatrixMultiply(T, A));
  return pMatrixMultiply(Inv, T);
}

async function copyTo(sub: Matrix, A: Matrix, top: number, left: number) {
  let n = sub.length;
  let m = sub[0].length;
  await parallelFor(0, n - 1, async i => {
    await parallelFor(0, m - 1, async j => {
      A[top + i][left + j] = sub[i][j];
    });
  });
}

async function join(A11: Matrix, A12: Matrix, A21: Matrix, A22: Matrix): Promise<Matrix> {
  let n = A11.length;
  let A: Matrix = [];
  await parallelFor(0, 2 * n - 1, async i => {
    A[i] = [];
  });
  let handles = [
    copyTo(A11, A, 0, 0),
    copyTo(A12, A, 0, n),
    copyTo(A21, A, n, 0),
    copyTo(A22, A, n, n),
  ];
  await Promise.all(handles);
  return A;
}

async function pInverseRecursive(A: Matrix): Promise<Matrix> {
  if (A.length === 1) {
    let a = 1 / A[0][0];
    return [[a]];
  } else {
    let SA = new SubMatrix(A);
    let subs = SA.partition();
    let handles = subs.map(s => s.extract());
    let [B, CT, C, D] = await Promise.all(handles);

    let BInv = await pInverseRecursive(B);
    let W = await pMatrixMultiply(C, BInv);
    let WT = await pTranspose(W);
    let X = await pMatrixMultiply(W, CT);
    let S = await pMatrixPairwise(D, X, (a, b) => a - b);
    let SInv = await pInverseRecursive(S);
    let V = SInv;
    let Y = await pMatrixMultiply(SInv, W);
    let YT = await pTranspose(Y);
    let T = await pMatrixPairwise(YT, YT, (a, b) => -a);
    let U = await pMatrixPairwise(Y, Y, (a, b) => -a);
    let Z = await pMatrixMultiply(WT, Y);
    let R = await pMatrixPairwise(BInv, Z, (a, b) => a + b);

    return join(R, T, U, V);
  }
}
