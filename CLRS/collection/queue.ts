export {
  Queue
};

class Queue<T> {
  protected _head: number;
  protected _tail: number;
  protected _size: number;
  protected _arr: T[];
  capacity: number;

  constructor(cap: number) {
    this._head = 0;
    this._tail = 0;
    this._size = 0;
    this.capacity = cap;

    this._arr = new Array(cap);
  }

  isEmpty(): boolean {
    return this._size === 0;
  }

  isFull(): boolean {
    return this._size === this.capacity;
  }

  enqueue(x: T) {
    if (this.isFull()) {
      throw "Error: Queue overflow";
    }

    let Q = this._arr;
    let tail = this._tail;

    Q[this._tail] = x;
    this._tail = this.next(tail);
    this._size++;
  }

  dequeue(): T {
    if (this.isEmpty()) {
      throw "Error: Queue underflow";
    }

    let Q = this._arr;
    let head = this._head;

    let x = Q[head];
    this._head = this.next(head);
    this._size--;
    return x;
  }

  protected next(i: number): number {
    if (i === this.capacity - 1) {
      return 0;
    } else {
      return i + 1;
    }
  }

  report() {
    console.log(`head: ${this._head}, tail: ${this._tail}`);
    console.log(this._arr);
  }
}

class Deque<T> extends Queue<T> {
  prev(i: number): number {
    if (i === 0) {
      return this.capacity - 1;
    } else {
      return i - 1;
    }
  }

  headInsert(x: T) {
    if (this.isFull()) {
      throw "Error: Queue overflow";
    }

    let Q = this._arr;
    let head = this._head;

    head = this.prev(head);
    Q[head] = x;
    this._head = head;
    this._size++;
  }

  headDelete(): T {
    return this.dequeue();
  }

  tailInsert(x: T) {
    this.enqueue(x);
  }

  tailDelete(): T {
    if (this.isEmpty()) {
      throw "Error: Queue underflow";
    }

    let Q = this._arr;
    let tail = this._tail;

    tail = this.prev(tail);
    let x = Q[tail];
    this._tail = tail;
    this._size--;
    return x;
  }
}