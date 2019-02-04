import { PriorityQueue } from "../sort/heap";

class MinPriorityQueue extends PriorityQueue<number> {
  cmp(a: number, b: number): boolean {
    return a < b;
  }
}

export function parallelScheduling(m: number, jobs: number[]): number {
  let machines: number[] = [];
  for (let i = 0; i < m; i++) {
    machines.push(0);
  }
  let heap = new MinPriorityQueue(machines);

  for (let job of jobs) {
    let machine = heap.extractRoot();
    heap.insertKey(machine + job);
  }

  let comps = heap.arr().slice(0, heap.size());
  return comps.reduce((a, b) => Math.max(a, b));
}