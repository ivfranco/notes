export function insertionSort<T>(A: T[]) {
  insertionSortSlice(A, 0, A.length - 1);
}

export function insertionSortSlice<T>(A: T[], p: number, r: number) {
  for (let j = p; j <= r; j++) {
    let key = A[j];
    let i = j - 1;
    while (i >= 0 && A[i] > key) {
      A[i + 1] = A[i];
      // console.log(arr);
      i--;
    }
    A[i + 1] = key;
  }
}