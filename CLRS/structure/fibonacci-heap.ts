export {
  AbstractFHeap,
  FHeap,
  FHeapNode,
  HeapNode,
  MergableHeap,
};

type Cmp<T> = (a: T, b: T) => boolean;

interface MergableHeap<T, N extends HeapNode<T>> {
  insert(k: T): void;
  minimum(): N | null;
  union(other: this): void;
  extractMin(): N | null;
  decreaseKey(x: N, k: T): void;
  delete(x: N): void;
}

abstract class AbstractFHeap<T, N extends FHeapNode<T>> implements MergableHeap<T, N> {
  public min: N | null;
  public n: number;

  constructor() {
    this.min = null;
    this.n = 0;
  }

  protected abstract factory(k: T): N;
  protected abstract cmp(a: T, b: T): boolean;

  public isEmpty(): boolean {
    return this.n === 0;
  }

  public insert(k: T): N {
    let cmp = this.cmp;

    let x = this.factory(k);
    if (this.min === null) {
      this.min = x;
    } else {
      this.min.append(x);
      if (cmp(x.key, this.min.key)) {
        this.min = x;
      }
    }

    this.n++;
    return x;
  }

  public minimum(): N | null {
    return this.min;
  }

  public decreaseKey(x: N, k: T) {
    let cmp = this.cmp;
    if (cmp(x.key, k)) {
      throw Error("Error: new key is greater than current key");
    }
    x.key = k;
    let y = x.parent;
    if (y && cmp(x.key, y.key)) {
      this.cut(x, y);
      this.cascadingCut(y);
    }
    if (!this.min || cmp(x.key, this.min.key)) {
      this.min = x;
    }
  }

  private cut(x: N, y: N) {
    if (x.isSingleton()) {
      y.child = null;
    } else {
      if (y.child === x) {
        y.child = x.right;
      }
      x.remove();
    }
    y.degree--;
    x.parent = null;
    x.mark = false;
    (this.min as N).append(x);
  }

  private cascadingCut(y: N) {
    let z = y.parent;
    if (z) {
      if (!y.mark) {
        y.mark = true;
      } else {
        this.cut(y, z);
        this.cascadingCut(z);
      }
    }
  }

  public delete(x: N) {
    //  basically inlined decreaseKey, avoided resorting to -Infinity
    let y = x.parent;
    if (y) {
      this.cut(x, y);
      this.cascadingCut(y);
    }
    this.min = x;
    this.extractMin();
  }

  public union(H2: this) {
    let cmp = this.cmp;
    let H1 = this;

    if (H1.min && H2.min) {
      H1.min.concat(H2.min);
    }

    if (H1.min === null || H2.min && cmp(H2.min.key, H1.min.key)) {
      H1.min = H2.min;
    }

    H1.n += H2.n;
  }

  public extractMin(): N | null {
    let z = this.min;
    if (z) {
      let c = Array.from(z.children());
      for (let x of c) {
        z.append(x);
        x.parent = null;
      }

      if (z.isSingleton()) {
        this.min = null;
      } else {
        this.min = z.right;
        z.remove();
        this.consolidate(this.min);
      }

      this.n--;
    }

    return z;
  }

  private consolidate(min: N) {
    let cmp = this.cmp;
    let A: Array<N | null> = [];
    let W = Array.from(min.siblings());

    for (let x of W) {
      let d = x.degree;
      while (A[d] != null) {
        let y = A[d] as N;
        if (cmp(y.key, x.key)) {
          let temp = x;
          x = y;
          y = temp;
        }
        link(x, y);
        A[d] = null;
        d++;
      }
      A[d] = x;
    }

    this.min = null;

    for (let w of A) {
      if (!this.min || w && cmp(w.key, this.min.key)) {
        this.min = w;
      }
    }
  }

  public show(): string {
    if (this.min) {
      let A = Array.from(this.min.siblings());
      return A.map(x => x.show()).join("\n");
    } else {
      return "Empty fibonacci heap";
    }
  }

  public diagnose() {
    let cmp = this.cmp;
    let n = 0;
    let A = [];
    if (this.min) {
      for (let r of this.min.siblings()) {
        n += r.diagnose(cmp);
        console.assert(!cmp(r.key, this.min.key), "this.min should be the minimum node");
      }
    }
    console.assert(this.n === n, "n must match the number of nodes");
  }
}

class FHeap<T> extends AbstractFHeap<T, FHeapNode<T>> {
  protected factory(k: T): FHeapNode<T> {
    return new FHeapNode(k);
  }

  protected cmp(a: T, b: T): boolean {
    return a < b;
  }
}

abstract class HeapNode<T> {
  public key: T;
  public degree: number;
  public left: this;
  public right: this;
  public parent: this | null;
  public child: this | null;

  constructor(k: T) {
    this.key = k;
    this.degree = 0;
    this.left = this;
    this.right = this;
    this.parent = null;
    this.child = null;
  }

  public isSingleton(): boolean {
    return this.right === this;
  }

  public *siblings(): IterableIterator<this> {
    let start = this;
    let cursor = this;
    do {
      yield cursor;
      cursor = cursor.right;
    } while (cursor !== start);
  }

  public *children(): IterableIterator<this> {
    if (this.child) {
      yield* this.child.siblings();
    }
  }

  public append(y: this) {
    append(this, y);
  }

  public prepend(y: this) {
    prepend(this, y);
  }

  public insert(y: this) {
    insert(this, y);
  }

  public concat(y: this) {
    concat(this, y);
  }

  //  does not maintain pointers in parent or fibonacci heap
  public remove() {
    remove(this);
  }

  protected nodeStringify(): string {
    return "" + this.key;
  }

  private toLines(): string[] {
    let lines: string[] = [];
    for (let c of this.children()) {
      lines.push(...c.toLines());
    }

    let mid = Math.floor(lines.length / 2);

    lines = lines.map((l, i) => {
      let prefix = "";
      if (lines.length === 1) {
        prefix = "──";
      } else if (i === 0 && i === mid) {
        prefix = "┬─";
      } else if (i === lines.length - 1 && i === mid) {
        prefix = "┴─";
      } else if (i === mid) {
        prefix = "┤ ";
      } else if (i === 0) {
        prefix = "┌─";
      } else if (i === lines.length - 1) {
        prefix = "└─";
      } else {
        prefix = "│ ";
      }
      return prefix + l;
    });

    if (lines.length === 0) {
      lines = [""];
    }

    let str = this.nodeStringify();
    let len = str.length;
    let space = "";
    while (len > 0) {
      space += " ";
      len--;
    }

    return lines.map((l, i) => {
      if (i === mid) {
        return str + l;
      } else {
        return space + l;
      }
    });
  }

  public show(): string {
    return this.toLines().join("\n");
  }
}

//  an empty fibonacci node list is not well defined
//  instead any function that may delete the last node in a node list should check if the list is a singleton
class FHeapNode<T> extends HeapNode<T> {
  public mark: boolean;

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
    super(k);
    this.mark = false;
  }

  public diagnose(cmp: Cmp<T>): number {
    let d = 0;
    let n = 0;
    for (let c of this.children()) {
      console.assert(!cmp(c.key, this.key), "heap property");
      console.assert(c.parent === this, "parent pointer must be valid");
      n += c.diagnose(cmp);
      d++;
    }
    console.assert(this.degree === d, "degree field must match the number of children");
    return n + 1;
  }
}

function append<T, N extends HeapNode<T>>(x: N, y: N) {
  let right = x.right;
  x.right = y;
  y.left = x;
  right.left = y;
  y.right = right;
  y.parent = x.parent;
}

function prepend<T, N extends HeapNode<T>>(x: N, y: N) {
  let left = x.left;
  x.left = y;
  y.right = x;
  left.right = y;
  y.left = left;
  y.parent = x.parent;
}

function insert<T, N extends HeapNode<T>>(x: N, y: N) {
  if (x.child === null) {
    y.left = y;
    y.right = y;
    x.child = y;
  } else {
    append(x.child, y);
  }
  y.parent = x;
  x.degree++;
}

//  parent pointers in y is not fixed
function concat<T, N extends HeapNode<T>>(x: N, y: N) {
  let x_right = x.right;
  let y_left = y.left;

  x_right.left = y_left;
  y_left.right = x_right;
  x.right = y;
  y.left = x;
}

function remove<T, N extends HeapNode<T>>(x: N) {
  if (x.right !== x) {
    let left = x.left;
    let right = x.right;

    left.right = right;
    right.left = left;
  }
}

//  assume x and y being different roots in the same heap
function link<T>(x: FHeapNode<T>, y: FHeapNode<T>) {
  y.remove();
  x.insert(y);
  y.mark = false;
}
