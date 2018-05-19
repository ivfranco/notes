import { id, isSorted, randomAB, randomStr, shuffle } from "../util";
import { MaxDHeap } from "./d-heap";
import {
  FIFOQueue,
  heapSort,
  MaxHeap,
  MaxPriorityQueue,
  mergeArrays,
  Stack,
} from "./heap";
import {
  bucketSort,
  countingSort,
  inplaceCountingSort,
  radixSort,
  stringSort,
} from "./linear-sort";
import { randomizedSelect2 } from "./order";
import {
  fuzzysort,
  hoarePartition,
  hoareQuicksort,
  isFuzzySorted,
  quicksort,
  quicksort2,
  tailRecursiveQuicksort,
} from "./quicksort";
import { YoungTableau } from "./young-tableau";

function main() {
  for (let i = 0; i < 100; i++) {
    if (!problem_9_2_3()) {
      throw new Error("Fail");
    }
  }
  console.log("Pass");
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

function problem_6_5_1() {
  let A = [15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1];
  let max_pq = new MaxPriorityQueue(A);
  console.log(`Maximum key is ${max_pq.extractRoot()}`);
}

function problem_6_5_2() {
  let A = [15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1];
  let max_pq = new MaxPriorityQueue(A);
  console.log("Inserting 10...");
  max_pq.insertKey(10);
  max_pq.diagnose();
}

function problem_6_5_7() {
  let n = randomAB(1, 100);
  let A = [];
  for (let i = 0; i < n; i++) {
    A.push(Math.random());
  }
  let queue = new FIFOQueue();
  let stack = new Stack();
  for (let r of A) {
    queue.insert(r);
    stack.insert(r);
  }
  for (let i = 0; i < n; i++) {
    if (A[i] !== queue.extract()) {
      console.error(`wrong queue behavior on A[${i}]`);
    }
    if (A[n - 1 - i] !== stack.extract()) {
      console.error(`wrong stack behavior on A[${i}]`);
    }
  }
  if (!queue.isEmpty()) {
    console.error("unmatched queue insert and extract");
  }
  if (!stack.isEmpty()) {
    console.error("unmatched stack insert and extract");
  }
  console.log("Test end");
}

function problem_6_5_8() {
  let n = randomAB(1, 100);
  let A = [];
  for (let i = 0; i < n; i++) {
    A.push(Math.random());
  }
  let max_heap = new MaxPriorityQueue(A);

  console.log("Expected: out of boundery error");
  try {
    max_heap.deleteKey(n);
  } catch (e) {
    console.log(e);
  }

  while (n > 0) {
    max_heap.deleteKey(randomAB(0, n - 1));
    n--;
  }

  if (!max_heap.isEmpty()) {
    console.error("wrong _heap_size");
  }

  console.log("Test end");
}

function problem_6_5_9() {
  let n = randomAB(1, 100);
  let A: number[][] = [];
  let cnt = 0;
  for (let i = 0; i < n; i++) {
    let rs = [];
    let m = randomAB(0, 100);
    for (let j = 0; j < m; j++) {
      rs.push(Math.random());
    }
    cnt += m;
    rs.sort((a, b) => a - b);
    A.push(rs);
  }

  let sorted = mergeArrays(A);
  if (isSorted(sorted)) {
    console.log("Sorted");
  } else {
    console.error("Not sorted");
  }

  console.log(`Expected length: ${cnt}`);
  console.log(`Actual length: ${sorted.length}`);
}

function problem_6_1() {
  let O = [1, 2, 3, 4, 5];
  let A = O.slice();
  let B = A.slice();

  new MaxHeap(A);
  let max_pq: MaxPriorityQueue<number> = new MaxPriorityQueue([]);
  for (let b of B) {
    max_pq.insertKey(b);
  }

  console.log("Original input:  ", O);
  console.log("BUILD-MAX-HEAP:  ", A);
  console.log("Repeated insert: ", max_pq.arr());
}

function problem_6_2() {
  let A = [15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1];
  let d = randomAB(2, 5);
  let max_dheap = new MaxDHeap(d, A);

  console.log("Initial test:");
  console.log(max_dheap.arr());
  max_dheap.diagnose();

  console.log("\nInsert test:");
  max_dheap.insertKey(10);
  console.log(max_dheap.arr());
  max_dheap.diagnose();

  console.log("\nIncrease test:");
  max_dheap.adjustKey(5, 20);
  console.log(max_dheap.arr());
  max_dheap.diagnose();
}

function problem_6_3() {
  let n = randomAB(1, 10);
  let tableau = new YoungTableau(n, n);
  for (let i = 0; i < n ** 2; i++) {
    tableau.insert(Math.random());
  }
  tableau.diagnose();

  let sorted = [];
  while (!tableau.isEmpty()) {
    sorted.push(tableau.extractMin());
  }
  console.log("Is sorted:       ", isSorted(sorted));
  console.log("Expected length: ", n ** 2);
  console.log("Actual length:   ", sorted.length);
}

function problem_6_3_f(): boolean {
  let m = randomAB(1, 5);
  let n = randomAB(1, 5);
  let key = randomAB(1, 100);
  let tableau = new YoungTableau(m, n);

  let inserted = false;
  if (Math.random() >= 0.5) {
    tableau.insert(key);
    inserted = true;
  }

  console.log("Key = ", key);

  let filling = randomAB(0, m * n - 1);
  for (let i = 0; i < filling; i++) {
    let r = randomAB(1, 100);
    if (r !== key) {
      tableau.insert(r);
    }
  }

  tableau.diagnose();
  console.log("Expected answer: ", inserted);
  console.log("Search result:   ", tableau.find(key));
  return (tableau.find(key) !== null) === inserted;
}

function problem_7_1_1() {
  let A = [13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
  quicksort(A, 0, A.length - 1);
  console.log(A);
}

function problem_7_1() {
  let A = [13, 19, 9, 5, 12, 8, 7, 4, 11, 2, 6, 21];
  console.log("Return value: ", hoarePartition(A, 0, A.length - 1));

  hoareQuicksort(A, 0, A.length - 1);
  console.log(A);
}

function problem_7_2() {
  let n = randomAB(1, 10000);
  let A = [];
  for (let i = 0; i < n; i++) {
    A.push(randomAB(1, 100));
  }
  quicksort2(A, 0, A.length - 1);
  return isSorted(A);
}

function problem_7_4() {
  let n = randomAB(1, 10000);
  let A = [];
  for (let i = 0; i < n; i++) {
    A.push(randomAB(1, 100));
  }
  tailRecursiveQuicksort(A, 0, A.length - 1);
  return isSorted(A);
}

function problem_7_6() {
  let n = randomAB(1, 10000);
  let A: Array<[number, number]> = [];
  for (let i = 0; i < n; i++) {
    let a = randomAB(0, 100);
    let b = a + randomAB(0, 100);
    A.push([a, b]);
  }
  fuzzysort(A, 0, A.length - 1);
  return [isFuzzySorted(A), A];
}

function problem_8_2_1() {
  let A = [6, 0, 2, 0, 1, 3, 4, 6, 1, 3, 2];
  console.log(isSorted(countingSort(A, 6, id)));
}

function problem_8_3_1() {
  const CODE_A = "A".charCodeAt(0);
  function extract(w: string, i: number): number {
    return w.charCodeAt(w.length - i) - CODE_A;
  }
  let A = ["COW", "DOG", "SEA", "RUG", "ROW", "MOB", "BOX", "TAB", "BAR", "EAR",
    "TAR", "DIG", "BIG", "TEA", "NOW", "FOX"];
  console.log(isSorted(radixSort(A, 3, 26, extract)));
}

function problem_8_4_1() {
  let A = [.79, .13, .16, .64, .39, .20, .89, .53, .71, .42];
  console.log(bucketSort(A));
}

function problem_8_2(): boolean {
  let n = randomAB(1, 10000);
  let A = [];
  for (let i = 0; i < n; i++) {
    A[i] = randomAB(0, 20);
  }
  inplaceCountingSort(A, 20);
  return isSorted(A);
}

function problem_8_3(): boolean {
  let n = randomAB(1, 10);
  let A = [];
  for (let i = 0; i < n; i++) {
    A.push(randomStr(randomAB(0, 10)));
  }
  let sorted = stringSort(A);
  return isSorted(sorted);
}

function problem_9_2_3(): boolean {
  let n = randomAB(1, 10000);
  let A = [];
  for (let i = 0; i < n; i++) {
    A[i] = i + 1;
  }
  shuffle(A);
  let m = randomAB(1, n);
  return randomizedSelect2(A, 0, A.length - 1, m) === m;
}

main();
