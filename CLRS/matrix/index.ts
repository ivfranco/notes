import { matrixMultiplication } from "../start/matrix-mul";
import {
  copyMatrix,
  forwardSubst,
  luDecomposition,
  lupDecomposition,
  lupSolve,
  multiplyByVector,
} from "./lup-decomposition";

function main() {
  problem_28_1_3();
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

main();
