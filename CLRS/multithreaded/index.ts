import { floydWarshall, fromWeightedGraph } from "../graph/all-pair-shortest-path";
import { WeightedGraph } from "../graph/weighted-graph";
import { matrixMultiplication } from "../start/matrix-mul";
import { isSorted } from "../util";
import {
  pFloydWarshall,
  pMatrixMultiply,
  pMatrixMultiplyDivide,
  pStrassen,
  pTranspose,
} from "./p-matrix-multiply";
import { pMergeSort } from "./p-merge-sort";

async function main() {
  await pMergeSortTest();
}

async function matrixTest() {
  let A = [
    [1, 3],
    [7, 5],
  ];
  let B = [
    [6, 8],
    [4, 2],
  ];

  console.log("Serial implementation:");
  console.log(matrixMultiplication(A, B));
  console.log("P-SQUARE-MATRIX-MULTIPLY:");
  let C = await pMatrixMultiply(A, B);
  console.log(C);
  console.log("P-MATRIX-MULTIPLY-RECURSIVE:");
  C = await pMatrixMultiplyDivide(A, B);
  console.log(C);
  console.log("P-STRASSEN-RECURSIVE:");
  C = await pStrassen(A, B);
  console.log(C);
}

async function problem_27_2_5() {
  let A = [
    [1, 2, 3, 4],
    [1, 2, 3, 4],
    [1, 2, 3, 4],
    [1, 2, 3, 4],
  ];

  await pTranspose(A);
  console.log(A);
}

async function problem_27_2_6() {
  let G = WeightedGraph.fromDirected(
    "1 2 3 4 5 6",
    [
      "1 5 -1",
      "2 1 1", "2 4 2",
      "3 2 2", "3 6 -8",
      "4 1 -4", "4 5 3",
      "5 2 7",
      "6 2 5", "6 3 10",
    ],
  );

  let W = fromWeightedGraph(G);

  console.log(floydWarshall(W)[0]);
  console.log(await pFloydWarshall(W));
}

async function pMergeSortTest() {
  let A = [];
  let n = 1000;
  for (let i = 0; i < n; i++) {
    A.push(Math.random());
  }

  let B: number[] = [];
  await pMergeSort(A, 0, n - 1, B, 0);
  A.sort((a, b) => a - b);
  for (let i = 0; i < n; i++) {
    console.assert(A[i] === B[i]);
  }
}

main();
