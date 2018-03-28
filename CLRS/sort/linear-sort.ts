export {
  countingSort,
  radixSort,
  bucketSort,
  inplaceCountingSort,
  stringSort
};

import { insertionSort } from "../start/insertion-sort";
import { SWAP } from "../util";


// C[i] stores the number of elements in A less or equal to i
function counting(A: number[], k: number): number[] {
  let C: number[] = new Array(k + 1).fill(0);
  A.forEach((a) => C[a]++);

  C.reduce((acc, c_val, c_idx, C) => {
    C[c_idx] = acc + c_val;
    return acc + c_val;
  });

  return C;
}

// k: the maximum of elements in A
// there may be easier way to obtain k than iterating over the whole array A
// so it's required as a parameter instead of computed from A
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
  // B.forEach((bucket, i) => console.log(`bucket ${i}: `, bucket));
  B.forEach(insertionSort);
  // B.forEach((bucket, i) => console.log(`bucket ${i} (sorted): `, bucket));
  return B.reduce((concated, bucket) => concated.concat(bucket));
}

function inplaceCountingSort(A: number[], k: number) {
  let n = A.length;
  let C = counting(A, k);
  // a copy of original counting results
  // C_max[i] contains the maximum index an element = i is stored
  // won't be modified
  let C_max = C.slice();

  for (let i = n - 1; i >= 0; i--) {
    // A[i] already swapped to the right position in previous iterations
    // this iteration can be skipped
    if (i > C[A[i]] - 1 && i <= C_max[A[i]] - 1) {
      continue;
    }
    // by moving A[i] to the right position in the array, A[C[A[i]] - 1], element previously there is overwritten
    // instead of overwritting the element, it's swapped with A[i], then it's either in the right position or not
    // if now both elements are in the right position, the inner loop terminates
    // otherwise the new A[i] again swapped to the right position, the loop continues
    while (C[A[i]] - 1 !== i) {
      // each iteration of the inner loop fixes the position of at least one element
      // hence the inner loop at most can run for n iterations in total
      // then the array is sorted, every element in the right position
      let cnt = C[A[i]];
      C[A[i]]--;
      SWAP(A, cnt - 1, i);
    }
  }
}

function stringSort(A: string[]): string[] {
  let max_len = A.map(s => s.length).reduce((a, b) => Math.max(a, b));
  let L: string[][] = new Array(max_len);
  for (let i = 0; i <= max_len; i++) {
    L[i] = [];
  }
  for (let s of A) {
    L[s.length].push(s);
  }

  const CODE_A = "a".charCodeAt(0);
  let S: string[] = [];
  for (let i = max_len; i > 0; i--) {
    // concatenation of javascript arrays are not O(1)
    // only a demonstration of correctness
    S = L[i].concat(S);
    S = countingSort(S, 25, s => s.charCodeAt(i - 1) - CODE_A);
  }

  return S;
}