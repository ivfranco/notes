export function randomAB(a: number, b: number) {
  if (a > b) {
    throw "Error: lower bound greater than higher bound";
  }
  return Math.floor(Math.random() * ((b - a) + 1)) + a;
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