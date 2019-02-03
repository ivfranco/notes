export {
  MaxHeap,
  PriorityQueue,
  MaxPriorityQueue,
  heapSort,
  FIFOQueue,
  Stack,
  mergeArrays,
};

function PARENT(i: number): number {
  return Math.floor((i + 1) / 2) - 1;
}

function LEFT(i: number): number {
  return 2 * (i + 1) - 1;
}

// the first node with no left (or right) child
// LEFT(i) = 2 * (i+1) - 1 >= n
// 2 * (i+1) >= n+1
// if n+1 is even, i+1 >= (n+1)/2 = [(n+1)/2], i >= [(n+1)/2] - 1
// if n+1 is odd, i+1 > [(n+1)/2] = n/2
// LEFT(n/2 - 1) = n-1, still in the array
function FIRST_LEAF(n: number): number {
  return Math.floor(n / 2);
}

function RIGHT(i: number): number {
  return LEFT(i) + 1;
}

abstract class Heap<T> {
  protected _heap_size: number;
  protected _heap_arr: T[];

  constructor(A: T[]) {
    this._heap_arr = A;
    this._heap_size = A.length;

    for (let i = FIRST_LEAF(this._heap_size) - 1; i >= 0; i--) {
      this.heapify(i);
    }
  }

  // cmp must not refer any fields or methods of the heap
  // it must compute the result solely from functions/variables independent of the heap
  // otherwise due to the weird behavior of `this` in javascript, all let cmp = this.cmp in methods will be invalid
  public abstract cmp(a: T, b: T): boolean;

  protected inBound(i: number) {
    return i >= 0 && i < this._heap_size;
  }

  protected swap(i: number, j: number) {
    if (!this.inBound(i) || !this.inBound(j)) {
      throw new Error("Error: Out of boundary access");
    }

    let A = this._heap_arr;
    // console.log(`swapped A[${i}] = ${A[i]} and A[${j}] = ${A[j]}`);
    let temp = A[i];
    A[i] = A[j];
    A[j] = temp;
  }

  public size(): number {
    return this._heap_size;
  }

  public isEmpty(): boolean {
    return this._heap_size === 0;
  }

  public arr(): T[] {
    return this._heap_arr;
  }

  public heapify(i: number) {
    let largest = i;
    do {
      i = largest;
      let l = LEFT(i);
      let r = RIGHT(i);
      let A = this._heap_arr;

      if (this.inBound(l) && this.cmp(A[l], A[largest])) {
        largest = l;
      }
      if (this.inBound(r) && this.cmp(A[r], A[largest])) {
        largest = r;
      }

      if (i !== largest) {
        this.swap(i, largest);
      }
    } while (i !== largest);
  }

  public decrementSize() {
    if (this._heap_size <= 0) {
      throw new Error("Error: Heap underflow");
    }

    this._heap_size--;
  }

  public diagnose() {
    console.log("Self diagnosing...");
    let n = this._heap_size;
    let A = this._heap_arr;
    let cmp = this.cmp;

    for (let i = 0; i < FIRST_LEAF(n); i++) {
      let l = LEFT(i);
      let r = RIGHT(i);
      if (this.inBound(l) && cmp(A[l], A[i])) {
        throw new Error(`Error: the position of A[${i}] = ${A[i]} and its left child A[${l}] = ${A[l]} are invalid`);
      }
      if (this.inBound(r) && cmp(A[r], A[i])) {
        throw new Error(`Error: the position of A[${i}] = ${A[i]} and its right child A[${r}] = ${A[r]} are invalid`);
      }
    }

    for (let i = 0; i < n; i++) {
      if (i === undefined) {
        throw new Error(`Error: A[${i}] Uninitialized`);
      }
    }

    if (A.length < n) {
      throw new Error("Error: underlying array inconsistent");
    }

    console.log("Self diagnosis successful.");
  }
}

abstract class PriorityQueue<T> extends Heap<T> {
  public root() {
    return this._heap_arr[0];
  }

  public extractRoot(): T {
    if (this._heap_size <= 0) {
      throw new Error("Error: Heap underflow");
    }

    let root = this.root();
    this.swap(0, this._heap_size - 1);
    this.decrementSize();
    this.heapify(0);
    return root;
  }

  protected fix(i: number) {
    let A = this._heap_arr;
    let cmp = this.cmp;
    let key = A[i];
    while (i > 0 && cmp(key, A[PARENT(i)])) {
      A[i] = A[PARENT(i)];
      // console.log(`moved A[${PARENT(i)}] = ${A[PARENT(i)]} to A[${i}]`);
      i = PARENT(i);
    }
    // console.log(`placed key ${key} at A[${i}]`);
    A[i] = key;
  }

  public adjustKey(i: number, key: T) {
    let cmp = this.cmp;
    let A = this._heap_arr;

    if (cmp(A[i], key)) {
      throw new Error(`Error: invalid new key ${key} compared to original A[${i}] = ${A[i]}`);
    }

    A[i] = key;
    this.fix(i);
  }

  public insertKey(key: T) {
    let A = this._heap_arr;

    this._heap_size++;
    let i = this._heap_size - 1;
    A[i] = key;
    this.fix(i);
  }

  public deleteKey(i: number) {
    let A = this._heap_arr;
    let cmp = this.cmp;

    this.swap(i, this._heap_size - 1);
    this.decrementSize();
    if (this._heap_size === 0) {
      return;
    }
    if (this.inBound(PARENT(i)) && cmp(A[i], A[PARENT(i)])) {
      this.fix(i);
    } else {
      this.heapify(i);
    }
  }
}

// T: Ord
class MaxHeap<T> extends Heap<T> {
  constructor(A: T[]) {
    super(A);
  }

  public cmp(a: T, b: T): boolean {
    return a > b;
  }
}

// T: Ord
function heapSort<T>(A: T[]) {
  let max_heap = new MaxHeap(A);
  for (let i = A.length - 1; i >= 1; i--) {
    let temp = A[i];
    A[i] = A[0];
    A[0] = temp;
    max_heap.decrementSize();
    max_heap.heapify(0);
  }
}

// T: Ord
class MaxPriorityQueue<T> extends PriorityQueue<T> {
  constructor(A: T[]) {
    super(A);
  }

  public cmp(a: T, b: T): boolean {
    return a > b;
  }
}

// T: Ord
class MinPriorityQueue<T> extends PriorityQueue<T> {
  constructor(A: T[]) {
    super(A);
  }

  public cmp(a: T, b: T): boolean {
    return a < b;
  }
}

class FIFOQueue<T> extends PriorityQueue<[number, T]> {
  public cnt: number;
  constructor() {
    super([]);
    this.cnt = 0;
  }

  public cmp(a: [number, T], b: [number, T]) {
    let [key_a, ele_a] = a;
    let [key_b, ele_b] = b;
    return key_a < key_b;
  }

  public insert(a: T) {
    this.insertKey([this.cnt, a]);
    this.cnt++;
  }

  public extract(): T {
    this.cnt--;
    return this.extractRoot()[1];
  }
}

class Stack<T> extends PriorityQueue<[number, T]> {
  public cnt: number;
  constructor() {
    super([]);
    this.cnt = 0;
  }

  public cmp(a: [number, T], b: [number, T]) {
    let [key_a, ele_a] = a;
    let [key_b, ele_b] = b;
    return key_a > key_b;
  }

  public insert(a: T) {
    this.insertKey([this.cnt, a]);
    this.cnt++;
  }

  public extract(): T {
    this.cnt--;
    return this.extractRoot()[1];
  }
}

// T: Ord
class MergeQueue<T> extends PriorityQueue<T[]> {
  constructor(A: T[][]) {
    super(A);
  }

  // decide the larger of the value of the last element in arrays (>, strictly greater)
  // an empty array is considered smaller than any non-empty array
  public cmp(a: T[], b: T[]): boolean {
    if (a.length === 0 && b.length === 0) {
      return false;
    }
    if (b.length === 0) {
      return true;
    }
    if (a.length === 0) {
      return false;
    }

    let a_last = a[a.length - 1];
    let b_last = b[b.length - 1];
    return a_last > b_last;
  }

  public extract(): T | null {
    let max_arr = this.root();
    // if the max array is empty, all arrays in the heap is empty
    if (max_arr.length === 0) {
      return null;
    }
    // extract max (last) element from max_arr, the new value of its last element uncertain
    // as max_arr.length != 0, max will always exist
    let max = max_arr.pop() as T;
    // therefore the heap property has to be restored
    this.heapify(0);
    return max;
  }
}

function mergeArrays<T>(A: T[][]) {
  let max_pq = new MergeQueue(A);
  let sorted: T[] = [];
  let max: T | null;
  while ((max = max_pq.extract()) != null) {
    sorted.push(max);
  }
  return sorted.reverse();
}
