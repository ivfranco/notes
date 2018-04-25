export {
  DSNode,
};

class DSList<T> implements Iterable<DSNode<T>> {
  public head: DSNode<T>;
  public tail: DSNode<T>;
  public n: number;

  constructor(x: DSNode<T>) {
    this.head = x;
    this.tail = x;
    this.n = 1;
  }

  public union(other: DSList<T>) {
    for (let x of other) {
      x.set = this;
    }
    this.tail.next = other.head;
    this.tail = other.tail;
    this.n += other.n;
  }

  public *[Symbol.iterator](): IterableIterator<DSNode<T>> {
    for (let cursor: DSNode<T> | null = this.head; cursor !== null; cursor = cursor.next) {
      yield cursor;
    }
  }

  public show(): string {
    let keys = Array.from(this).map(n => n.key);
    return `{${keys.join(", ")}}`;
  }
}

class DSNode<T> {
  public readonly key: T;
  public set: DSList<T>;
  public next: this | null;

  constructor(k: T) {
    this.key = k;
    this.set = new DSList(this);
    this.next = null;
  }

  public findSet(): DSNode<T> {
    return this.set.head;
  }

  public union(g: DSNode<T>) {
    if (this.set.n >= g.set.n) {
      this.set.union(g.set);
    } else {
      g.set.union(this.set);
    }
  }
}
