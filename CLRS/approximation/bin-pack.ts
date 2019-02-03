import { MaxPriorityQueue } from "../sort/heap";

export function approxBinPack(S: number[]): number {
  let heap: MaxPriorityQueue<number> = new MaxPriorityQueue([]);

  for (let s of S) {
    if (heap.isEmpty() || heap.root() < s) {
      heap.insertKey(1 - s);
    } else {
      let bin = heap.extractRoot();
      heap.insertKey(bin - s);
    }
  }

  return heap.size();
}