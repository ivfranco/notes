import { MaxHeap, heapSort } from "./heap";

function main() {
  problem_6_4_1();
}

function problem_6_3_1() {
  let A = [5, 3, 17, 10, 84, 19, 6, 22, 9];
  let max_heap = new MaxHeap(A);
}

function problem_6_4_1() {
  let A = [5, 13, 2, 25, 7, 17, 20, 8, 4];
  heapSort(A);
  console.log(A);
}

main();