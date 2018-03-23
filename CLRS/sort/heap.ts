export {
  MaxHeap,
  heapSort
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
  private _heap_size: number;
  private _heap_arr: T[];

  constructor(A: T[]) {
    this._heap_arr = A;
    this._heap_size = A.length;

    for (let i = FIRST_LEAF(this._heap_size) - 1; i >= 0; i--) {
      this.maxHeapify(i);
    }
  }

  abstract cmp(a: T, b: T): boolean;

  private inBound(i: number) {
    return i >= 0 && i < this._heap_size;
  }

  private swap(i: number, j: number) {
    if (!this.inBound(i) || !this.inBound(j)) {
      throw "Error: Out of boundary access";
    }

    let A = this._heap_arr;
    console.log(`swapped A[${i}] = ${A[i]} and A[${j}] = ${A[j]}`);
    let temp = A[i];
    A[i] = A[j];
    A[j] = temp;
  }

  maxHeapify(i: number) {
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

      if (i != largest) {
        this.swap(i, largest);
      }
    } while (i != largest);
  }

  decrementSize() {
    if (this._heap_size <= 0) {
      throw "Error: Heap underflow";
    }

    this._heap_size--;
  }
}

// T: Ord
class MaxHeap<T> extends Heap<T> {
  constructor(A: T[]) {
    super(A);
  }

  cmp(a: T, b: T): boolean {
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
    max_heap.maxHeapify(0);
  }
}