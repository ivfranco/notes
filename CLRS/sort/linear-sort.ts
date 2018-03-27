export {
  countingSort,
  radixSort,
  bucketSort
};

import { insertionSort } from "../start/insertion-sort";

// k: the maximum of elements in A
// there may be easier way to obtain k than iterating over the whole array A
// so it's required as a parameter instead of computed from A

function counting(A: number[], k: number): number[] {
  let C: number[] = new Array(k + 1).fill(0);
  A.forEach((a) => C[a]++);

  C.reduce((acc, c_val, c_idx, C) => {
    C[c_idx] = acc + c_val;
    return acc + c_val;
  });

  return C;
}

function countingSort<T>(A: T[], k: number, f: (a: T) => number): T[] {
  let n = A.length;
  let B = new Array(n).fill(null);

  let C = counting(A.map(a => f(a)), k);

  for (let i = n - 1; i >= 0; i--) {
    B[C[f(A[i])] - 1] = A[i];
    // console.log(C);
    // console.log(B);
    C[f(A[i])]--;
  }

  return B;
}

function countingRange(A: number[], queries: [number, number][]): number[] {
  let C = counting(A, A.reduce((a, b) => Math.max(a, b)));
  return queries.map(([a, b]) => {
    if (a == 0) {
      return C[b];
    } else {
      return C[b] - C[a - 1];
    }
  });
}

// d: the number of digits in each element
// r: the radix
// extract: extracts ith least significant digit from the value a of type T, which is in range [0, r-1]
function radixSort<T>(A: T[], d: number, r: number, extract: (a: T, i: number) => number): T[] {
  for (let i = 1; i <= d; i++) {
    A = countingSort(A, r - 1, a => extract(a, i));
    console.log(A);
  }
  return A;
}

function bucketSort(A: number[]): number[] {
  let n = A.length;
  let B: number[][] = new Array(n);
  for (let i = 0; i < n; i++) {
    B[i] = [];
  }
  A.forEach(a => B[Math.floor(n * a)].push(a));
  B.forEach((bucket, i) => console.log(`bucket ${i}: `, bucket));
  B.forEach(insertionSort);
  B.forEach((bucket, i) => console.log(`bucket ${i} (sorted): `, bucket));
  return B.reduce((concated, bucket) => concated.concat(bucket));
}