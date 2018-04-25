export {
  DSTreeNode,
};

class DSTreeNode<T> implements Iterable<DSTreeNode<T>> {
  public readonly key: T;
  private parent: this;
  private next: this;
  private rank: number;

  constructor(k: T) {
    this.key = k;
    this.parent = this;
    this.next = this;
    this.rank = 0;
  }

  private link(y: this) {
    let x = this;
    if (x.rank > y.rank) {
      y.parent = x;
    } else {
      x.parent = y;
      if (x.rank === y.rank) {
        y.rank++;
      }
    }

    let temp = y.next;
    y.next = x.next;
    x.next = temp;
  }

  public findSet(): this {
    let ancestors = [this];
    let cursor = this;
    while (cursor.parent !== cursor) {
      cursor = cursor.parent;
      ancestors.push(cursor);
    }
    for (let x of ancestors) {
      x.parent = cursor;
    }

    return this.parent;
  }

  public *[Symbol.iterator](): IterableIterator<DSTreeNode<T>> {
    yield this;
    let cursor = this.next;
    while (cursor !== this) {
      yield cursor;
      cursor = cursor.next;
    }
  }

  public show(): string {
    let keys = Array.from(this.findSet()).map(e => e.key);
    return `{${keys.join(", ")}}`;
  }

  public union(y: this) {
    this.findSet().link(y.findSet());
  }
}
