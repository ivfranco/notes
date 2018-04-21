export {
  FHeapNode,
};

//  an empty fibonacci node list is not well defined
//  instead any function that may delete the last node in a node list should check if the list is a singleton
class FHeapNode<T> {
  public key: T;
  public degree: number;
  public mark: boolean;
  public left: this;
  public right: this;
  public child: this | null;

  public static from<T>(I: Iterable<T>): FHeapNode<T> {
    let A = Array.from(I);
    if (A.length === 0) {
      throw Error("Error: Empty fibonacci node list");
    }

    let head = new FHeapNode(A[0]);
    for (let i = 1; i < A.length; i++) {
      let node = new FHeapNode(A[i]);
      head.append(node);
    }

    return head;
  }

  constructor(k: T) {
    this.key = k;
    this.degree = 0;
    this.mark = false;
    this.left = this;
    this.right = this;
    this.child = null;
  }

  public isSingleton(): boolean {
    return this.right === this;
  }

  public *siblings(): IterableIterator<FHeapNode<T>> {
    let start = this;
    let cursor = this;
    do {
      yield cursor;
      cursor = cursor.right;
    } while (cursor !== start);
  }

  public *children(): IterableIterator<FHeapNode<T>> {
    if (this.child) {
      yield* this.child.siblings();
    }
  }

  public append(y: FHeapNode<T>) {
    append(this, y);
  }

  public insert(y: FHeapNode<T>) {
    insert(this, y);
  }

  public concat(y: FHeapNode<T>) {
    concat(this, y);
  }

  //  does not maintain points in parent or fibonacci heap
  public remove() {
    remove(this);
  }
}

function append<T>(x: FHeapNode<T>, y: FHeapNode<T>) {
  let right = x.right;

  x.right = y;
  y.left = x;
  right.left = y;
  y.right = right;
}

function insert<T>(x: FHeapNode<T>, y: FHeapNode<T>) {
  if (x.child === null) {
    y.left = y;
    y.right = y;
    x.child = y;
  } else {
    x.child.append(y);
  }
  x.degree++;
}

function concat<T>(x: FHeapNode<T>, y: FHeapNode<T>) {
  let x_right = x.right;
  let y_left = y.left;

  x_right.left = y_left;
  y_left.right = x_right;
  x.right = y;
  y.left = x;
}

function remove<T>(x: FHeapNode<T>) {
  if (x.right !== x) {
    let left = x.left;
    let right = x.right;

    left.right = right;
    right.left = left;
  }
}
