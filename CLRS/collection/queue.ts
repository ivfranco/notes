export {
  Queue,
};

class Queue<T> {
  protected _head: number;
  protected _tail: number;
  protected _size: number;
  protected _arr: T[];
  public capacity: number;

  constructor(cap: number) {
    this._head = 0;
    this._tail = 0;
    this._size = 0;
    this.capacity = cap;

    this._arr = new Array(cap);
  }

  public isEmpty(): boolean {
    return this._size === 0;
  }

  public isFull(): boolean {
    return this._size === this.capacity;
  }

  public enqueue(x: T) {
    if (this.isFull()) {
      throw new Error("Error: Queue overflow");
    }

    let Q = this._arr;
    let tail = this._tail;

    Q[this._tail] = x;
    this._tail = this.next(tail);
    this._size++;
  }

  public dequeue(): T {
    if (this.isEmpty()) {
      throw new Error("Error: Queue underflow");
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

  public report() {
    console.log(`head: ${this._head}, tail: ${this._tail}`);
    console.log(this._arr);
  }
}

class Deque<T> extends Queue<T> {
  public prev(i: number): number {
    if (i === 0) {
      return this.capacity - 1;
    } else {
      return i - 1;
    }
  }

  public headInsert(x: T) {
    if (this.isFull()) {
      throw new Error("Error: Queue overflow");
    }

    let Q = this._arr;
    let head = this._head;

    head = this.prev(head);
    Q[head] = x;
    this._head = head;
    this._size++;
  }

  public headDelete(): T {
    return this.dequeue();
  }

  public tailInsert(x: T) {
    this.enqueue(x);
  }

  public tailDelete(): T {
    if (this.isEmpty()) {
      throw new Error("Error: Queue underflow");
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
