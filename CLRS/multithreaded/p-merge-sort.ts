export {
  pMergeSort,
};

import { parallelFor } from "./p-matrix-multiply";

function binarySearch<T>(x: T, T: T[], p: number, r: number): number {
  let low = p;
  let high = Math.max(p, r + 1);

  while (low < high) {
    let mid = Math.floor((low + high) / 2);
    if (x <= T[mid]) {
      high = mid;
    } else {
      low = mid + 1;
    }
  }

  return high;
}

async function pMerge<T>(T: T[], p1: number, r1: number, p2: number, r2: number, A: T[], p3: number): Promise<void> {
  let n1 = r1 - p1 + 1;
  let n2 = r2 - p2 + 1;
  if (n1 < n2) {
    return pMerge(T, p2, r2, p1, r1, A, p3);
  }

  if (n1 === 0) {
    return;
  } else {
    let q1 = Math.floor((p1 + r1) / 2);
    let q2 = binarySearch(T[q1], T, p2, r2);
    let q3 = p3 + (q1 - p1) + (q2 - p2);
    A[q3] = T[q1];
    let handle = pMerge(T, p1, q1 - 1, p2, q2 - 1, A, p3);
    await pMerge(T, q1 + 1, r1, q2, r2, A, q3 + 1);
    await handle;
  }
}

async function pMergeSort<T>(A: T[], p: number, r: number, B: T[], s: number) {
  let n = r - p + 1;
  if (n === 1) {
    B[s] = A[p];
  } else {
    let C: T[] = [];
    let q = Math.floor((p + r) / 2);
    let t = q - p;
    let handle = pMergeSort(A, p, q, C, 0);
    await pMergeSort(A, q + 1, r, C, t + 1);
    await handle;
    await pMerge(C, 0, t, t + 1, n - 1, B, s);
  }
}
