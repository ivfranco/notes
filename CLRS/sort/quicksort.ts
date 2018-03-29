export {
  quicksort,
  hoarePartition,
  hoareQuicksort,
  quicksort2,
  tailRecursiveQuicksort,
  fuzzysort,
  isFuzzySorted,
  partition,
  randomPivoter
};

import { SWAP, randomAB, swapReport } from "../util";

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
  SWAP(A, pivot, r);
  let x = A[r];
  let i = p - 1;
  for (let j = p; j <= r - 1; j++) {
    if (A[j] <= x) {
      i++;
      SWAP(A, i, j);
    }
  }
  SWAP(A, i + 1, r);
  // let equal = true;
  // for (let j = p; j <= r; j++) {
  //   equal = equal && A[j] == x;
  // }
  // if (equal) {
  //   return Math.floor((p + r) / 2);
  // }
  return i + 1;
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

// T: Ord
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
      SWAP(A, i, j);
    } else {
      return j;
    }
  }
}

function hoareQuicksort<T>(A: T[], p: number, r: number) {
  if (p < r) {
    let q = hoarePartition(A, p, r);
    hoareQuicksort(A, p, q);
    hoareQuicksort(A, q + 1, r);
  }
}

// T: Ord
function partition2<T>(A: T[], p: number, r: number, pivoter: Pivoter<T>): [number, number] {
  let pivot = pivoter(A, p, r);
  SWAP(A, pivot, r);
  let x = A[r];
  let i = p - 1, j = p - 1;
  for (let k = p; k <= r - 1; k++) {
    if (A[k] < x) {
      i++;
      j++;
      SWAP(A, i, j);
      if (j !== k) {
        SWAP(A, i, k);
      }
    } else if (A[k] === x) {
      j++;
      SWAP(A, j, k);
    }
  }
  SWAP(A, j + 1, r);
  return [i + 1, j + 1];
}

function quicksort2<T>(A: T[], p: number, r: number) {
  if (p < r) {
    let [q, t] = partition2(A, p, r, lastPivoter);
    quicksort2(A, p, q - 1);
    quicksort2(A, t + 1, r);
  }
}

function tailRecursiveQuicksort<T>(A: T[], p: number, r: number) {
  while (p < r) {
    let q = partition(A, p, r, lastPivoter);
    if (q <= (r + p) / 2) {
      tailRecursiveQuicksort(A, p, q - 1);
      p = q + 1;
    } else {
      tailRecursiveQuicksort(A, q + 1, r);
      r = q - 1;
    }
  }
}

type Interval = [number, number];

function overlapping([a1, b1]: Interval, [a2, b2]: Interval): boolean {
  return b1 >= a2 && a1 <= b2;
}

// i1 contains i2
function contain([a1, b1]: Interval, [a2, b2]: Interval): boolean {
  return a1 <= a2 && b1 >= b2;
}

function intersection(i1: Interval, i2: Interval): Interval | null {
  if (!overlapping(i1, i2)) {
    return null;
  } else {
    let [a1, b1] = i1;
    let [a2, b2] = i2;
    return [Math.max(a1, a2), Math.min(b1, b2)];
  }
}

function fuzzyPartition(A: Interval[], p: number, r: number, pivoter: Pivoter<Interval>): [number, number] {
  let pivot = pivoter(A, p, r);
  SWAP(A, pivot, r);
  let x = A[r];
  // first pass, compute an approximately minimal intersection of intervals overlap with the pivot
  for (let k = p; k <= r - 1; k++) {
    let insec = intersection(x, A[k]);
    if (insec !== null) {
      x = insec;
    }
  }
  // second pass, partition the array
  let i = p - 1, j = p - 1;
  let [ax, bx] = x;
  for (let k = p; k <= r - 1; k++) {
    let [a, b] = A[k];
    if (contain(A[k], x)) {
      j++;
      SWAP(A, j, k);
    } else if (a < ax) {
      i++;
      j++;
      SWAP(A, i, j);
      if (j !== k) {
        SWAP(A, i, k);
      }
    }
  }
  SWAP(A, j + 1, r);
  return [i + 1, j + 1];
}

function fuzzysort(A: Interval[], p: number, r: number) {
  if (p < r) {
    let [q, t] = fuzzyPartition(A, p, r, randomPivoter);
    fuzzysort(A, p, q - 1);
    fuzzysort(A, t + 1, r);
  }
}

function isFuzzySorted(A: Interval[]): boolean {
  let [amin, bmin] = A[0];
  for (let [a, b] of A) {
    if (amin > b) {
      return false;
    } else {
      amin = Math.min(amin, a);
    }
  }
  return true;
}