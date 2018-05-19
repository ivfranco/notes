export {
  randomizedSelect2,
  select,
  median,
};

import { insertionSortSlice } from "../start/insertion-sort";
import { partition, randomPivoter } from "./quicksort";
// T: Ord
function randomizedSelect<T>(A: T[], p: number, r: number, i: number): T {
  if (p === r) {
    return A[p];
  } else {
    let q = partition(A, p, r, randomPivoter);
    // after partition, A[q] then is the (q - p + 1)th smallest element in A[p .. r]
    let k = q - p + 1;
    if (i === k) {
      return A[q];
    } else if (i < k) {
      return randomizedSelect(A, p, q - 1, i);
    } else {
      return randomizedSelect(A, q + 1, r, i - k);
    }
  }
}

function randomizedSelect2<T>(A: T[], p: number, r: number, i: number): T {
  while (p < r) {
    let q = partition(A, p, r, randomPivoter);
    // after partition, A[q] then is the (q - p + 1)th smallest element in A[p .. r]
    let k = q - p + 1;
    if (i === k) {
      return A[q];
    } else if (i < k) {
      // return randomizedSelect(A, p, q - 1, i);
      r = q - 1;
    } else {
      // return randomizedSelect(A, q + 1, r, i - k);
      p = q + 1;
      i = i - k;
    }
  }
  return A[p];
}

function mmPivoter<T>(A: T[], p: number, r: number): number {
  let medians = [];
  for (let i = p; i <= r - 4; i += 5) {
    insertionSortSlice(A, i, i + 4);
    medians.push(A[i + 2]);
  }
  if (i < r) {
    insertionSortSlice(A, i, r);
    medians.push(A[Math.floor((i + r) / 2)]);
  }
  let x = select(medians, 0, medians.length - 1, Math.floor(medians.length / 2));
  for (let i = p; i <= r; i++) {
    if (A[i] === x) {
      return i;
    }
  }

  throw new Error("Error: supposed median of medians not present in the array");
}

function select<T>(A: T[], p: number, r: number, i: number): T {
  if (p === r) {
    return A[p];
  } else {
    let q = partition(A, p, r, mmPivoter);
    let k = q - p + 1;
    if (i === k) {
      return A[q];
    } else if (i < k) {
      return select(A, p, q - 1, i);
    } else {
      return select(A, q + 1, r, i - k);
    }
  }
}

function median<T>(A: T[]): T {
  let r = A.length - 1;
  let m = Math.floor(r / 2);

  return select(A, 0, r, m);
}
