export {
  quicksort,
  hoarePartition
};

import { swap, randomAB, swapReport } from "../util";

// a function choosing a pivot from an array slice
type Pivoter<T> = (A: T[], p: number, r: number) => number;

function lastPivoter<T>(A: T[], p: number, r: number): number {
  return r;
}

function randomPivoter<T>(A: T[], p: number, r: number): number {
  return randomAB(p, r);
}

// T: Ord
function partition<T>(A: T[], p: number, r: number, pivoter: Pivoter<T>): number {
  let pivot = pivoter(A, p, r);
  swap(A, pivot, r);
  let x = A[r];
  let i = p - 1;
  for (let j = p; j <= r - 1; j++) {
    if (A[j] <= x) {
      i++;
      swap(A, i, j);
    }
  }
  swap(A, i + 1, r);
  let equal = true;
  for (let j = p; j <= r; j++) {
    equal = equal && A[j] == x;
  }
  if (equal) {
    return Math.floor((p + r) / 2);
  } else {
    return i + 1;
  }
}

// T: Ord
function quicksort<T>(A: T[], p: number, r: number) {
  if (p < r) {
    let q = partition(A, p, r, lastPivoter);
    quicksort(A, p, q - 1);
    quicksort(A, q + 1, r);
  }
}

function randomizedQuicksort<T>(A: T[], p: number, r: number) {
  if (p < r) {
    let q = partition(A, p, r, randomPivoter);
    quicksort(A, p, q - 1);
    quicksort(A, q + 1, r);
  }
}

function hoarePartition<T>(A: T[], p: number, r: number): number {
  let x = A[p];
  let i = p - 1;
  let j = r + 1;
  while (true) {
    do {
      j--;
    } while (A[j] > x);
    do {
      i++;
    } while (A[i] < x);
    if (i < j) {
      swapReport(A, i, j);
    } else {
      return j;
    }
  }
}