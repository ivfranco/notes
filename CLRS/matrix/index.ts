import { matrixMultiplication } from "../start/matrix-mul";
import { leastSquareFit } from "./least-squares-approximation";
import {
  copyMatrix,
  forwardSubst,
  luDecomposition,
  lupDecomposition,
  lupSolve,
  multiplyByVector,
} from "./lup-decomposition";

function main() {
  problem_28_3_6();
}

function problem_28_1_1() {
  let A = [
    [1, 0, 0],
    [4, 1, 0],
    [-6, 5, 1],
  ];

  let b = [3, 14, -7];
  let x = forwardSubst(A, b);
  console.log(x);
  console.log("Given vector:");
  console.log(b);
  console.log("Calculated vector:");
  console.log(multiplyByVector(A, x));
}

function problem_28_1_2() {
  let A = [
    [4, -5, 6],
    [8, -6, 7],
    [12, -7, 12],
  ];
  console.log("Given matrix:");
  console.log(A);

  let [L, U] = luDecomposition(A);
  console.log("Calculated matrix:");
  console.log(matrixMultiplication(L, U));
}

function problem_28_1_3() {
  let A = [
    [1, 5, 4],
    [2, 0, 3],
    [5, 8, 2],
  ];
  let B = copyMatrix(A);
  let b = [12, 9, 5];

  let P = lupDecomposition(A);
  let x = lupSolve(A, A, P, b);
  console.log("Given vector:");
  console.log(b);
  console.log("Calculated vector:");
  console.log(multiplyByVector(B, x));
}

function leastSquareFitTest() {
  let points = [
    [-1, 2],
    [1, 1],
    [2, 1],
    [3, 0],
    [5, 3],
  ].map(([x, y]) => {
    return { x, y };
  });

  let f = (x: number, n: number) => x ** n;
  console.log(leastSquareFit(points, 3, f));
}

function problem_28_3_6() {
  let points = [
    [1, 1],
    [2, 1],
    [3, 3],
    [4, 8],
  ].map(([x, y]) => {
    return { x, y };
  });

  let f = (x: number, n: number) => {
    switch (n) {
      case 0:
        return 1;
      case 1:
        return x * Math.log2(x);
      case 2:
        return Math.exp(x);
    }
    throw Error("Error: Unreachable");
  };
  console.log(leastSquareFit(points, 3, f));
}

main();
