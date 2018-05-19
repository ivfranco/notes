import { floydWarshall, fromWeightedGraph } from "../graph/all-pair-shortest-path";
import { WeightedGraph } from "../graph/weighted-graph";
import { matrixMultiplication } from "../start/matrix-mul";
import { isSorted } from "../util";
import { randomAB } from "../util";
import {
  pFloydWarshall,
  pMatrixMultiply,
  pMatrixMultiplyDivide,
  pStrassen,
  pTranspose,
} from "./p-matrix-multiply";
import {
  pMergeSort,
  pPartition,
  pRandomizedSelect,
  pRandomPivoter,
  pSelect,
} from "./p-merge-sort";

async function main() {
  while (1) {
    await problem_27_3_6();
  }
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

async function problem_27_3_3() {
  let n = randomAB(1, 15);
  let p = randomAB(0, n - 1);
  let r = randomAB(p, n - 1);
  console.log(`n = ${n}, p = ${p}, r = ${r}`);
  let A: number[] = [];
  for (let i = 0; i < n; i++) {
    A.push(randomAB(0, 100));
  }
  let T: number[] = [];
  let last_le = await pPartition(A, p, r, pRandomPivoter, T);
  let pivot = T[last_le];
  console.log(T, last_le);
  for (let i = 0; i < last_le; i++) {
    console.assert(T[i] <= pivot, "Items before the return value should be less than or equal to the pivot");
  }
  for (let i = last_le + 1; i < T.length; i++) {
    console.assert(T[i] > pivot, "Items after the return value should be greater than the pivot");
  }
  let B = A.slice(p, r + 1);
  console.assert(B.length === T.length, "Partitioned array should have the same length to the source");
  B.sort((a, b) => a - b);
  T.sort((a, b) => a - b);
  for (let i = 0; i < B.length; i++) {
    console.assert(B[i] === T[i], "Partitioned array should have the same items to the source");
  }
}

async function problem_27_3_5() {
  let A = [13, 19, 9, 5, 12, 8, 7, 4, 11, 2, 6, 21];
  let B = A.slice();
  B.sort((a, b) => a - b);
  let i = randomAB(1, A.length);
  let selected = await pRandomizedSelect(A, 0, A.length - 1, i);
  console.log(B);
  console.log(`${i}th smallest item: ${selected}`);
  console.assert(selected === B[i - 1]);
}

async function problem_27_3_6() {
  let A = [13, 19, 9, 5, 12, 8, 7, 4, 11, 2, 6, 21];
  let B = A.slice();
  B.sort((a, b) => a - b);
  let i = randomAB(1, A.length);
  let selected = await pSelect(A, 0, A.length - 1, i);
  console.log(B);
  console.log(`${i}th smallest item: ${selected}`);
  console.assert(selected === B[i - 1]);
}

main();
