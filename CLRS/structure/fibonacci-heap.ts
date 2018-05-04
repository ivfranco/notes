export {
  AbstractFHeap,
  FHeap,
  FHeapNode,
  HeapNode,
  MergableHeap,
};

type Cmp<T> = (a: T, b: T) => boolean;

interface MergableHeap<K, V, N extends HeapNode<K, V>> {
  insert(k: K, v: V): void;
  minimum(): N | null;
  union(other: this): void;
  extractMin(): N | null;
  decreaseKey(x: N, k: K): void;
  delete(x: N): void;
}

abstract class AbstractFHeap<K, V, N extends FHeapNode<K, V>> implements MergableHeap<K, V, N> {
  public min: N | null;
  public n: number;

  constructor() {
    this.min = null;
    this.n = 0;
  }

  protected abstract factory(k: K, v: V): N;
  protected abstract cmp(a: K, b: K): boolean;

  public isEmpty(): boolean {
    return this.n === 0;
  }

  public insert(k: K, v: V): N {
    let cmp = this.cmp;

    let x = this.factory(k, v);
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

  public decreaseKey(x: N, k: K) {
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

class FHeap<K, V> extends AbstractFHeap<K, V, FHeapNode<K, V>> {
  protected factory(k: K, v: V): FHeapNode<K, V> {
    return new FHeapNode(k, v);
  }

  protected cmp(a: K, b: K): boolean {
    return a < b;
  }
}

abstract class HeapNode<K, V> {
  public key: K;
  public value: V;
  public degree: number;
  public left: this;
  public right: this;
  public parent: this | null;
  public child: this | null;

  constructor(k: K, v: V) {
    this.key = k;
    this.value = v;
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
    if (this.value) {
      return `${this.key}:${this.value}`;
    } else {
      return `${this.key}`;
    }
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
class FHeapNode<K, V> extends HeapNode<K, V> {
  public mark: boolean;

  public static from<K, V>(I: Iterable<[K, V]>): FHeapNode<K, V> {
    let A = Array.from(I);
    if (A.length === 0) {
      throw Error("Error: Empty fibonacci node list");
    }

    let [k, v] = A[0];
    let head = new FHeapNode<K, V>(k, v);
    for (let i = 1; i < A.length; i++) {
      [k, v] = A[i];
      let node = new FHeapNode<K, V>(k, v);
      head.append(node);
    }

    return head;
  }

  public static fromKey<K>(I: Iterable<K>): FHeapNode<K, null> {
    let A = Array.from(I);
    let B: Array<[K, null]> = A.map(k => [k, null] as [K, null]);
    return FHeapNode.from(B);
  }

  constructor(k: K, v: V) {
    super(k, v);
    this.mark = false;
  }

  public diagnose(cmp: Cmp<K>): number {
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

function append<K, V>(x: HeapNode<K, V>, y: HeapNode<K, V>) {
  let right = x.right;
  x.right = y;
  y.left = x;
  right.left = y;
  y.right = right;
  y.parent = x.parent;
}

function prepend<K, V>(x: HeapNode<K, V>, y: HeapNode<K, V>) {
  let left = x.left;
  x.left = y;
  y.right = x;
  left.right = y;
  y.left = left;
  y.parent = x.parent;
}

function insert<K, V>(x: HeapNode<K, V>, y: HeapNode<K, V>) {
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
function concat<K, V>(x: HeapNode<K, V>, y: HeapNode<K, V>) {
  let x_right = x.right;
  let y_left = y.left;

  x_right.left = y_left;
  y_left.right = x_right;
  x.right = y;
  y.left = x;
}

function remove<K, V>(x: HeapNode<K, V>) {
  if (x.right !== x) {
    let left = x.left;
    let right = x.right;

    left.right = right;
    right.left = left;
  }
}

//  assume x and y being different roots in the same heap
function link<K, V>(x: FHeapNode<K, V>, y: FHeapNode<K, V>) {
  y.remove();
  x.insert(y);
  y.mark = false;
}
