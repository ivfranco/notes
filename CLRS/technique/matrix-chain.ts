export {
  matrixChainOrder,
  optimalParens,
};

import { Matrix, matrixMultiplication } from "../start/matrix-mul";

function optimalParens(s: number[][], i: number, j: number): string {
  if (i === j) {
    return `A${i}`;
  } else {
    let lhs = optimalParens(s, i, s[i][j]);
    let rhs = optimalParens(s, s[i][j] + 1, j);
    return `(${lhs}${rhs})`;
  }
}

function matrixChainOrder(p: number[]): [number, number[][]] {
  let n = p.length - 1;
  let m: number[][] = new Array(n);
  let s: number[][] = new Array(n);

  for (let i = 0; i < n; i++) {
    m[i] = [];
    m[i][i] = 0;
    s[i] = [];
  }

  //  l is the chain length
  for (let l = 2; l <= n; l++) {
    //  unlike the text of CLRS, i the index of matrices falls within the range {0 .. n-1} here
    //  p[i] is the number of rows of Ai, p[i+1] is the number of columns
    for (let i = 0; i + l - 1 < n; i++) {
      let j = i + l - 1;
      m[i][j] = +Infinity;
      for (let k = i; k < j; k++) {
        //  the number of rows of Ai..k is p[i]
        //  the number of columns of Ai..k and the number of rows of Ak+1..j is p[k+1]
        //  the number of columns of Ak+1..j is p[j+1]
        let q = m[i][k] + m[k + 1][j] + p[i] * p[k + 1] * p[j + 1];
        if (q < m[i][j]) {
          m[i][j] = q;
          s[i][j] = k;
        }
      }
    }
  }

  return [m[0][n - 1], s];
}

function matrixChainMultiply(A: Matrix[], s: number[][], i: number, j: number): Matrix {
  if (i === j) {
    return A[i];
  } else {
    let lhs = matrixChainMultiply(A, s, i, s[i][j]);
    let rhs = matrixChainMultiply(A, s, s[i][j] + 1, j);
    return matrixMultiplication(lhs, rhs);
  }
}
