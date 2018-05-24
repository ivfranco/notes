export {
  Matrix,
  Vector,
  copyMatrix,
  multiplyByVector,
  lupDecomposition,
  luDecomposition,
  forwardSubst,
  backwardSubst,
  luSolve,
  lupSolve,
  luInverse,
};

import { SWAP } from "../util";

type Vector = number[];
type Matrix = Vector[];

function copyMatrix(A: Matrix): Matrix {
  let n = A.length;
  let m = A[0].length;
  let B: Matrix = [];
  for (let i = 0; i < n; i++) {
    B[i] = [];
    for (let j = 0; j < m; j++) {
      B[i][j] = A[i][j];
    }
  }
  return B;
}

function multiplyByVector(A: Matrix, v: Vector): Vector {
  console.assert(A[0].length === v.length, "Matrix and vector are not compatible");

  let n = A.length;
  let m = v.length;
  let b: Vector = [];
  for (let i = 0; i < n; i++) {
    b[i] = 0;
    for (let j = 0; j < m; j++) {
      b[i] += A[i][j] * v[j];
    }
  }

  return b;
}

function initLU(n: number): [Matrix, Matrix] {
  let L: Matrix = [];
  let U: Matrix = [];
  for (let i = 0; i < n; i++) {
    L[i] = [];
    U[i] = [];
    for (let j = 0; j < n; j++) {
      if (i === j) {
        L[i][j] = 1;
      } else if (i < j) {
        L[i][j] = 0;
      } else {
        U[i][j] = 0;
      }
    }
  }

  return [L, U];
}

function luDecomposition(A: Matrix): [Matrix, Matrix] {
  let n = A.length;
  let [L, U] = initLU(n);
  for (let k = 0; k < n; k++) {
    U[k][k] = A[k][k];
    for (let i = k + 1; i < n; i++) {
      L[i][k] = A[i][k] / U[k][k];
      U[k][i] = A[k][i];
    }
    for (let i = k + 1; i < n; i++) {
      for (let j = k + 1; j < n; j++) {
        A[i][j] -= L[i][k] * U[k][j];
      }
    }
  }
  return [L, U];
}

function lupDecomposition(A: Matrix): Vector {
  let n = A.length;
  let P: Vector = [];
  for (let i = 0; i < n; i++) {
    P[i] = i;
  }
  for (let k = 0; k < n; k++) {
    let max_abs = Math.abs(A[k][k]);
    let new_k = k;
    for (let i = k; i < n; i++) {
      if (Math.abs(A[i][k]) > max_abs) {
        max_abs = Math.abs(A[i][k]);
        new_k = i;
      }
    }
    if (max_abs === 0) {
      throw new Error("Error: Singular matrix");
    }
    SWAP(P, k, new_k);
    SWAP(A, k, new_k);
    for (let i = k + 1; i < n; i++) {
      A[i][k] = A[i][k] / A[k][k];
      for (let j = k + 1; j < n; j++) {
        A[i][j] -= A[i][k] * A[k][j];
      }
    }
  }

  return P;
}

function forwardSubst(L: Matrix, b: Vector): Vector {
  let n = L.length;
  let x: Vector = [];
  for (let i = 0; i < n; i++) {
    let sum = 0;
    for (let j = 0; j < i; j++) {
      sum += L[i][j] * x[j];
    }
    x[i] = b[i] - sum;
  }
  return x;
}

function backwardSubst(U: Matrix, b: Vector): Vector {
  let n = U.length;
  let x: Vector = [];
  for (let i = n - 1; i >= 0; i--) {
    let sum = 0;
    for (let j = i + 1; j < n; j++) {
      sum += U[i][j] * x[j];
    }
    x[i] = (b[i] - sum) / U[i][i];
  }

  return x;
}

function permute(P: Vector, b: Vector): Vector {
  let n = b.length;
  let permuted: Vector = [];
  for (let i = 0; i < n; i++) {
    permuted[i] = b[P[i]];
  }
  return permuted;
}

function luSolve(L: Matrix, U: Matrix, b: Vector): Vector {
  let y = forwardSubst(L, b);
  return backwardSubst(U, y);
}

function lupSolve(L: Matrix, U: Matrix, P: Vector, b: Vector): Vector {
  return luSolve(L, U, permute(P, b));
}

function unitVector(n: number, i: number): Vector {
  let e: Vector = [];
  for (let j = 0; j < n; j++) {
    e[j] = i === j ? 1 : 0;
  }
  return e;
}

function luInverse(L: Matrix, U: Matrix): Matrix {
  let n = L.length;
  let Inv: Matrix = [];
  for (let i = 0; i < n; i++) {
    Inv[i] = [];
  }

  for (let i = 0; i < n; i++) {
    let e = unitVector(n, i);
    let x = luSolve(L, U, e);
    for (let j = 0; j < n; j++) {
      Inv[j][i] = x[j];
    }
  }

  return Inv;
}
