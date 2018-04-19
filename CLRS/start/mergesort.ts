export {
  gmerge,
};

function merge(A: number[], p: number, q: number, r: number) {
  let L = A.slice(p, q + 1);
  let R = A.slice(q + 1, r + 1);
  L.push(Infinity);
  R.push(Infinity);

  for (let k = p, i = 0, j = 0; k <= r; k++) {
    if (L[i] <= R[j]) {
      A[k] = L[i];
      i++;
    } else {
      A[k] = R[j];
      j++;
    }
  }
  // console.log(A);
}

//  more general merge using no sentinels
//  assumes L and R independent to A
function gmerge<T>(A: T[], L: T[], R: T[], p: number, r: number) {
  let nl = L.length;
  let nr = R.length;
  console.assert(nl + nr === r - p + 1);

  let k = p;
  let i = 0;
  let j = 0;
  for (; k <= r && i < L.length && j < R.length; k++) {
    if (L[i] <= R[j]) {
      A[k] = L[i];
      i++;
    } else {
      A[k] = R[j];
      j++;
    }
  }

  for (; i < L.length; i++ , k++) {
    A[k] = L[i];
  }

  for (; j < R.length; j++ , k++) {
    A[k] = R[j];
  }
}

function mergeCountInversion(A: number[], p: number, q: number, r: number): number {
  let L = A.slice(p, q + 1);
  let R = A.slice(q + 1, r + 1);
  L.push(Infinity);
  R.push(Infinity);

  let inversion = 0;

  // j is the number of elements in R been copied to A
  // if later an element from L is copied to A, then exactly j elements in R is smaller than it
  for (let k = p, i = 0, j = 0; k <= r; k++) {
    if (L[i] <= R[j]) {
      A[k] = L[i];
      i++;
      inversion += j;
    } else {
      A[k] = R[j];
      j++;
    }
  }

  return inversion;
}

function mergeNoSentinel(A: number[], p: number, q: number, r: number) {
  let L = A.slice(p, q + 1);
  let R = A.slice(q + 1, r + 1);

  let k = p;
  let i = 0;
  let j = 0;
  for (; k <= r && i < L.length && j < R.length; k++) {
    if (L[i] <= R[j]) {
      A[k] = L[i];
      i++;
    } else {
      A[k] = R[j];
      j++;
    }
  }

  for (; i < L.length; i++ , k++) {
    A[k] = L[i];
  }

  for (; j < R.length; j++ , k++) {
    A[k] = R[j];
  }
}

export function mergeSort(A: number[], p: number, r: number) {
  if (p < r) {
    let q = Math.floor((p + r) / 2);
    mergeSort(A, p, q);
    mergeSort(A, q + 1, r);
    mergeNoSentinel(A, p, q, r);
  }
}

export function inversionCount(A: number[], p: number, r: number): number {
  if (p < r) {
    let q = Math.floor((p + r) / 2);
    // type 1
    let lc = inversionCount(A, p, q);
    // type 2
    let rc = inversionCount(A, q + 1, r);
    // type 3
    let mc = mergeCountInversion(A, p, q, r);
    return lc + rc + mc;
  } else {
    // permutations of a single element contains no inversion
    return 0;
  }
}
