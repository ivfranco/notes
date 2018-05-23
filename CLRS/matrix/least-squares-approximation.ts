export {
  leastSquareFit,
};

import { matrixMultiplication } from "../start/matrix-mul";
import { luDecomposition, luSolve, Matrix, multiplyByVector, Vector } from "./lup-decomposition";

function transpose(A: Matrix): Matrix {
  let n = A.length;
  let m = A[0].length;
  let T: Matrix = [];
  for (let i = 0; i < m; i++) {
    T[i] = [];
    for (let j = 0; j < n; j++) {
      T[i][j] = A[j][i];
    }
  }
  return T;
}

interface Point {
  x: number;
  y: number;
}

function leastSquareFit(points: Point[], n: number, f: (x: number, n: number) => number): Vector {
  let m = points.length;
  let y: Vector = points.map(p => p.y);
  let A: Matrix = [];
  for (let i = 0; i < m; i++) {
    A[i] = [];
    for (let j = 0; j < n; j++) {
      A[i][j] = f(points[i].x, j);
    }
  }

  let T = transpose(A);
  let b = multiplyByVector(T, y);
  let S = matrixMultiplication(T, A);
  let [L, U] = luDecomposition(S);
  return luSolve(L, U, b);
}
