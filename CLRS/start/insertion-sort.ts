export function insertionSort<T>(arr: T[]) {
  for (let j = 1; j < arr.length; j++) {
    let key = arr[j];
    let i = j - 1;
    while (i >= 0 && arr[i] > key) {
      arr[i + 1] = arr[i];
      // console.log(arr);
      i--;
    }
    arr[i + 1] = key;
  }
}