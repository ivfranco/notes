export {
  MaxDHeap
};

function PARENT(d: number, i: number) {
  return Math.floor((i - 1) / d);
}

function FIRST_CHILD(d: number, i: number) {
  return i * d + 1;
}

// FIRST_CHILD(i) >= n
// i * d + 1 >= n
// i * d >= n - 1
// if d | (n-1), i >= (n-1)/d
// otherwise Math.floor((n-1)/d) < i < Math.ceil((n-1)/d)
function FIRST_LEAF(d: number, n: number) {
  return Math.ceil((n - 1) / d);
}

abstract class DHeap<T> {
  protected _heap_size: number;
  protected _heap_arr: T[];
  protected _d: number;

  constructor(d: number, A: T[]) {
    this._heap_size = A.length;
    this._heap_arr = A;
    this._d = d;

    for (let i = FIRST_LEAF(d, this._heap_size) - 1; i >= 0; i--) {
      this.heapify(i);
    }
  }

  abstract cmp(a: T, b: T): boolean;

  protected inBound(i: number) {
    return i >= 0 && i < this._heap_size;
  }

  protected swap(i: number, j: number) {
    if (!this.inBound(i) || !this.inBound(j)) {
      throw "Error: Out of boundary access";
    }

    let A = this._heap_arr;
    console.log(`swapped A[${i}] = ${A[i]} and A[${j}] = ${A[j]}`);
    let temp = A[i];
    A[i] = A[j];
    A[j] = temp;
  }

  heapify(i: number) {
    let A = this._heap_arr;
    let d = this._d;
    let cmp = this.cmp;

    let largest = i;
    do {
      i = largest;
      let first_child = FIRST_CHILD(d, i);
      let last_child = first_child + d - 1;
      for (let c = first_child; this.inBound(c) && c <= last_child; c++) {
        if (cmp(A[c], A[largest])) {
          largest = c;
        }
      }
      if (i != largest) {
        this.swap(i, largest);
      }
    } while (i != largest);
  }

  arr(): T[] {
    return this._heap_arr;
  }

  root(): T {
    return this._heap_arr[0];
  }

  extractRoot(): T {
    let A = this._heap_arr;
    let n = this._heap_size;

    let root = this.root();
    this.swap(0, n - 1);
    this.decrementSize();
    this.heapify(0);
    return root;
  }

  decrementSize() {
    if (this._heap_size <= 0) {
      throw "Error: Heap underflow";
    }

    this._heap_size--;
  }

  insertKey(key: T) {
    let A = this._heap_arr;
    let n = this._heap_size;

    A[n] = key;
    this._heap_size++;
    this.fix(n);
  }

  adjustKey(i: number, key: T) {
    let A = this._heap_arr;
    let cmp = this.cmp;

    if (cmp(A[i], key)) {
      throw `Error: invalid new key ${key} compared to original A[${i}] = ${A[i]}`;
    }

    A[i] = key;
    this.fix(i);
  }

  protected fix(i: number) {
    let A = this._heap_arr;
    let d = this._d;
    let cmp = this.cmp;

    while (i > 0 && cmp(A[i], A[PARENT(d, i)])) {
      this.swap(i, PARENT(d, i));
      i = PARENT(d, i);
    }
  }

  diagnose() {
    console.log("Self diagnosing...");
    let n = this._heap_size;
    let d = this._d;
    let A = this._heap_arr;
    let cmp = this.cmp;

    for (let i = 0; i < FIRST_LEAF(d, n); i++) {
      let first_child = FIRST_CHILD(d, i);
      let last_child = first_child + d - 1;
      for (let c = first_child; this.inBound(c) && c <= last_child; c++) {
        if (cmp(A[c], A[i])) {
          throw `Error: the position of A[${i}] = ${A[i]} and its child A[${c}] = ${A[c]} are invalid`;
        }
      }
    }

    for (let i = 0; i < n; i++) {
      if (i === undefined) {
        throw `Error: A[${i}] Uninitialized`;
      }
    }

    if (A.length < n) {
      throw "Error: underlying array inconsistent";
    }

    console.log("Self diagnosis successful.");
  }
}

// T: Ord 
class MaxDHeap<T> extends DHeap<T> {
  constructor(d: number, A: T[]) {
    super(d, A);
  }

  cmp(a: T, b: T): boolean {
    return a > b;
  }
}