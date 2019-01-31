export type Cmp<T> = (a: T, b: T) => boolean;

export function id<T>(a: T): T {
  return a;
}

export function noop(any?: any) { /* noop */ }

export function randomAB(a: number, b: number) {
  if (a > b) {
    throw new Error("Error: lower bound greater than higher bound");
  }
  return Math.floor(Math.random() * ((b - a) + 1)) + a;
}

export function randomStr(len: number) {
  let chars = [];
  for (let i = 0; i < len; i++) {
    // lower case characters
    chars.push(randomAB(0x61, 0x7a));
  }
  return String.fromCharCode(...chars);
}

export function shuffle<T>(arr: T[]) {
  for (let i = arr.length - 1; i > 0; i--) {
    let j = Math.floor(Math.random() * (i + 1));
    let temp = arr[j];
    arr[j] = arr[i];
    arr[i] = temp;
  }
}

// T: Ord
export function isSorted<T>(A: T[]): boolean {
  for (let i = 0; i < A.length - 2; i++) {
    if (A[i] > A[i + 1]) {
      return false;
    }
  }
  return true;
}

// B: Ord
export function maxOn<A, B>(lhs: A, rhs: A, f: (a: A) => B): A {
  if (f(lhs) < f(rhs)) {
    return rhs;
  } else {
    return lhs;
  }
}

// B: Ord
export function maximumOn<A, B>(arr: A[], f: (a: A) => B): A {
  return arr.reduce((lhs, rhs) => maxOn(lhs, rhs, f));
}

// B: Ord
export function minOn<A, B>(lhs: A, rhs: A, f: (a: A) => B): A {
  if (f(lhs) > f(rhs)) {
    return rhs;
  } else {
    return lhs;
  }
}

// B: Ord
export function minimumOn<A, B>(arr: A[], f: (a: A) => B): A {
  return arr.reduce((lhs, rhs) => minOn(lhs, rhs, f));
}

export function SWAP<T>(A: T[], i: number, j: number) {
  let temp = A[i];
  A[i] = A[j];
  A[j] = temp;
}

export function swapReport<T>(A: T[], i: number, j: number) {
  SWAP(A, i, j);
  console.log(`Swapped A[${i}] = ${A[i]}, A[${j}] = ${A[j]}`);
  console.log(A);
}

function binaryPosition<T>(v: T, A: T[], p: number, q: number): number {
  if (p < q) {
    let mid = Math.floor((p + q) / 2);
    if (v === A[mid]) {
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

export function binarySearch<T>(v: T, A: T[]): number | null {
  let idx = binaryPosition(v, A, 0, A.length - 1);
  if (A[idx] === v) {
    return idx;
  } else {
    return null;
  }
}

export function equals<T>(A: T[], B: T[]): boolean {
  if (A.length !== B.length) {
    return false;
  }
  for (let i = 0; i < A.length; i++) {
    if (A[i] !== B[i]) {
      return false;
    }
  }
  return true;
}
