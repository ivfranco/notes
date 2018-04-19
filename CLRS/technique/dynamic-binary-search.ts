export {
  DynamicSortedArrays,
};

import { gmerge } from "../start/mergesort";
import { binarySearch, isSorted } from "../util";

class DynamicSortedArrays<T> {
  private arrays: T[][];
  private n: number;

  constructor(k: number) {
    this.arrays = [];
    for (let i = 0; i < k; i++) {
      this.arrays[i] = [];
    }
    this.n = 0;
  }

  public insert(k: T) {
    let AS = this.arrays;
    let merged = [k];
    let i = 0;
    for (; AS[i].length !== 0; i++) {
      gmerge(merged, AS[i], merged.slice(), 0, 2 ** (i + 1) - 1);
      AS[i] = [];
    }
    AS[i] = merged;
    this.n++;
  }

  public search(k: T): T | null {
    for (let A of this.arrays) {
      let idx = binarySearch(k, A);
      if (idx !== null) {
        return A[idx];
      }
    }
    return null;
  }

  /* tslint:disable no-bitwise */
  public diagnose() {
    let AS = this.arrays;
    let n = this.n;
    let size_list = [];
    for (let i = 0; i < AS.length; i++) {
      console.assert(isSorted(AS[i]));
      let mask = 1 << i;
      let size = (n & mask) === 0 ? 0 : 2 ** i;
      console.assert(AS[i].length === size);
      size_list.push(size);
    }
    console.assert(size_list.reduce((a, b) => a + b) === n);
  }
}
