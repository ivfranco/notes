import { mergeSort, inversionCount } from "./mergesort"

function main() {
  problem_2_4();
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

main();
