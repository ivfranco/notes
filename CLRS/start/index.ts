import { mergeSort, inversionCount } from "./mergesort";
import {
  findMaximumSubarray,
  findMaximumSubarrayBrute,
  findMaximumSubarrayMix,
  findMaximumSubArrayLinear
} from "./max-subarray";
import { matrixMultiplication, strassen } from "./matrix-mul";
import { Chip, GoodChip, BadChip, testChips } from "./chip-test";
import { randomAB } from "../util"

function main() {
  problem_4_5();
}

function insertionSort<T>(arr: T[]) {
  for (let j = 1; j < arr.length; j++) {
    let key = arr[j];
    let i = j - 1;
    while (i >= 0 && arr[i] < key) {
      arr[i + 1] = arr[i];
      // console.log(arr);
      i--;
    }
    arr[i + 1] = key;
  }
}

function problem_2_1_2() {
  let arr = [31, 41, 59, 26, 41, 58];
  insertionSort(arr);
  console.log(arr);
}

function linearSearch<T>(v: T, arr: T[]): number | null {
  for (let i = 0; i < arr.length; i++) {
    if (v == arr[i]) {
      return i;
    }
  }
  return null;
}

function addBinary(lhs: number[], rhs: number[]): number[] {
  let ret = [];
  let carry = 0;
  for (let i = 0; i < lhs.length; i++) {
    let sum = lhs[i] + rhs[i] + carry;
    ret[i] = sum & 0x1;
    carry = sum >> 1;
  }
  ret.push(carry);
  return ret;
}

function findMinimum(from: number, to: number, arr: number[]): number {
  let min = arr[from];
  let idx = from;
  for (let i = from; i <= to; i++) {
    if (arr[i] < min) {
      min = arr[i];
      idx = i;
    }
  }
  return idx;
}

function selectionSort(arr: number[]) {
  for (let i = 0; i < arr.length - 1; i++) {
    let min_idx = findMinimum(i, arr.length - 1, arr);
    let temp = arr[i];
    arr[i] = arr[min_idx];
    arr[min_idx] = temp;
  }
}

function problem_2_2_2() {
  let arr = [31, 41, 59, 26, 41, 58];
  selectionSort(arr);
  console.log(arr);
}

function problem_2_3_1() {
  let A = [3, 41, 52, 26, 38, 57, 9, 49];
  mergeSort(A, 0, A.length - 1);
  console.log(A);
}

function binaryPosition<T>(v: T, A: T[], p: number, q: number): number {
  if (p < q) {
    let mid = Math.floor((p + q) / 2);
    if (v == A[mid]) {
      return mid;
    } else if (v > A[mid]) {
      return binaryPosition(v, A, mid + 1, q);
    } else {
      return binaryPosition(v, A, p, mid);
    }
  } else {
    return p;
  }
}

function binarySearch<T>(v: T, A: T[]): number | null {
  let idx = binaryPosition(v, A, 0, A.length - 1);
  if (A[idx] == v) {
    return idx;
  } else {
    return null;
  }
}

function problem_2_3_5() {
  let A = [3, 41, 52, 26, 38, 57, 9, 49];
  mergeSort(A, 0, A.length - 1);
  console.log(A);
  console.log("index of 9:  ", binarySearch(9, A));
  console.log("index of 30: ", binarySearch(30, A));
}

// assume elements in A are distinct (A is a set)
function pairSum(x: number, A: number[]): [number, number] | null {
  // avoid modifying the original array
  let C = A.slice();
  mergeSort(C, 0, C.length - 1);
  for (let i = 0; i < C.length; i++) {
    let idx = binarySearch(x - C[i], C);
    if (idx != null && idx != i) {
      return [C[i], C[idx]];
    }
  }
  return null;
}

function problem_2_3_7() {
  let A = [3, 41, 52, 26, 38, 57, 9, 49];
  console.log(pairSum(29, A));
  console.log(pairSum(30, A));
}

function naivePolynomial(x: number, coffs: number[]) {
  let sum = 0;
  for (let i = 0; i < coffs.length; i++) {
    sum += coffs[i] * Math.pow(x, i);
  }
}

function problem_2_4() {
  let A = [2, 3, 8, 6, 1];
  console.log(inversionCount(A, 0, A.length - 1));
}

function reportTime(f: () => void): number {
  let now = Date.now();
  f();
  let then = Date.now();
  return then - now;
}

function benchSubarray(size: number) {
  let testSet = new Array(size);
  for (let i = 0; i < size; i++) {
    testSet[i] = Math.random() - 0.5;
  }

  let t_recur = reportTime(() => {
    findMaximumSubarray(testSet, 0, size - 1);
  });
  let t_brute = reportTime(() => {
    findMaximumSubarrayBrute(testSet, 0, size - 1);
  });
  let t_mix = reportTime(() => {
    findMaximumSubarrayMix(testSet, 0, size - 1);
  });
  let t_linear = reportTime(() => {
    findMaximumSubArrayLinear(testSet, 0, size - 1);
  });

  console.log("Test size:       ", size)
  console.log("Recurrance time: ", t_recur);
  console.log("Bruteforce time: ", t_brute);
  console.log("Mix time:        ", t_mix);
  console.log("Linear time:     ", t_linear);
}

function problem_4_1_3() {
  for (let size = 1; size <= 100000; size <<= 1) {
    benchSubarray(size);
    console.log("");
  }
}

function problem_4_1_5() {
  let A = new Array(100);
  for (let i = 0; i < 100; i++) {
    A[i] = Math.random() - 0.5;
  }
  console.log(findMaximumSubArrayLinear(A, 0, A.length - 1));
  console.log(findMaximumSubarrayBrute(A, 0, A.length - 1));
}

function problem_4_2_1() {
  let A = [
    [1, 3],
    [7, 5]
  ];
  let B = [
    [6, 8],
    [4, 2]
  ];
  console.log(matrixMultiplication(A, B));
  console.log(strassen(A, B));
}

function complexMultiply(a: number, b: number, c: number, d: number): [number, number] {
  let p1 = (a + b) * (c + d);
  let p2 = a * c;
  let p3 = b * d;
  return [p2 - p3, p1 - p2 - p3];
}

function problem_4_2_7() {
  let a = Math.random() - 0.5;
  let b = Math.random() - 0.5;
  let c = Math.random() - 0.5;
  let d = Math.random() - 0.5;
  console.log([a * c - b * d, a * d + b * c]);
  console.log(complexMultiply(a, b, c, d));
}

function shuffle<T>(arr: T[]) {
  for (let i = arr.length - 1; i > 0; i--) {
    let j = Math.floor(Math.random() * (i + 1));
    let temp = arr[j];
    arr[j] = arr[i];
    arr[i] = temp;
  }
}

function problem_4_5(): boolean {
  let n = Math.floor(Math.random() * 100 + 1);
  let nBad = Math.floor(n / 3);
  let nGood = n - nBad;
  let cs: Chip[] = [];
  for (let i = 0; i < nBad; i++) {
    cs.push(new BadChip());
  }
  for (let i = 0; i < nGood; i++) {
    cs.push(new GoodChip());
  }

  shuffle(cs);
  let allGood = testChips(cs).every((c) => c.isGood());
  console.log(allGood);
  return allGood;
}

// T: Eq
function randomSearch<T>(x: T, A: T[]): number | null {
  let n = A.length;
  // count of array elements not compared to x yet
  let cnt = n;
  // slots[idx] = true if A[idx] has yet been compared to x (and not equal)
  let slots = new Array(n).fill(true);

  while (cnt > 0) {
    let idx = randomAB(0, n - 1);
    // the exact index is generated before, skip this iteration
    if (slots[idx] == false) {
      continue;
    }
    // equal, search succeeded
    if (x == A[idx]) {
      return idx;
    }
    // not equal, and the index is not seen before, decrement the count, fill the slot
    if (slots[idx] == true) {
      slots[idx] = false;
      cnt--;
    }
  }

  // cnt == 0, all elements of A compared without success, search failed
  return null;
}

main();