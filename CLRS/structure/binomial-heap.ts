export {
  BHeap,
};

import { Cmp } from "../util";
import { HeapNode, MergableHeap } from "./fibonacci-heap";

class BHeap<T> implements MergableHeap<T, BHeapNode<T>> {
  public child: BHeapNode<T> | null;
  public n: number;

  constructor() {
    this.child = null;
    this.n = 0;
  }

  protected cmp(a: T, b: T): boolean {
    return a < b;
  }

  public insert(k: T) {
    let x = new BHeapNode(k);
    if (this.child) {
      let bnum = this.child.toBNumber();
      let sum = add(bnum, [x], this.cmp);
      this.child = BHeapNode.fromBNumber(sum);
    } else {
      this.child = x;
    }

    this.n++;
  }

  public union(other: this) {
    throw Error("Not implemented");
  }

  public minimum(): BHeapNode<T> | null {
    throw Error("Not implemented");
  }

  public extractMin(): BHeapNode<T> | null {
    throw Error("Not implemented");
  }

  public decreaseKey(x: BHeapNode<T>, k: T) {
    throw Error("Not implemented");
  }

  public delete(x: BHeapNode<T>) {
    throw Error("Not implemented");
  }

  public show(): string {
    if (this.child) {
      return Array.from(this.child.siblings())
        .map(n => n.show())
        .join("\n");
    } else {
      return "Empty binomial heap";
    }
  }

  public diagnose() {
    let n = 0;
    if (this.child) {
      for (let c of this.child.siblings()) {
        n += c.diagnose(this.cmp);
      }
    }
    console.assert(this.n === n, "n must match the number of nodes");
  }
}

class BHeapNode<T> extends HeapNode<T> {
  public static fromBNumber<T>(bnum: BNumber<T>): BHeapNode<T> {
    let nodes = bnum.filter(n => n != null) as Array<BHeapNode<T>>;
    if (nodes.length === 0) {
      throw Error("Error: Empty node list is not well defined");
    }
    return nodes.reduce((list, node) => {
      list.prepend(node);
      return node;
    }) as BHeapNode<T>;
  }

  public preInsert(other: this) {
    if (this.child) {
      this.child.prepend(other);
    } else {
      this.child = other;
      other.left = other;
      other.right = other;
    }
    this.child = other;
    other.parent = this;
    this.degree++;
  }

  public toBNumber(): BNumber<T> {
    let bnum: BNumber<T> = [];
    for (let s of this.siblings()) {
      bnum[s.degree] = s;
    }
    bnum.forEach(n => {
      if (n) {
        n.left = n;
        n.right = n;
        n.parent = null;
      }
    });
    return bnum;
  }

  public diagnose(cmp: Cmp<T>): number {
    let d = 0;
    let n = 0;

    for (let c of this.children()) {
      d++;
      n += c.diagnose(cmp);
      console.assert(c.degree === this.degree - d, "ith subtree should be a binomial tree of degree d - i");
      console.assert(!cmp(c.key, this.key), "Heap property");
    }

    console.assert(this.degree === d, "d must match the number of children");
    return n + 1;
  }
}

//  an array of binomial trees of distinct degrees
//  A[i] has degree i, A[i] == null indicates no tree in the array has degree i
type BNumber<T> = Array<BHeapNode<T> | null>;

function add<T>(x: BNumber<T>, y: BNumber<T>, cmp: Cmp<T>): BNumber<T> {
  let bnum = [];
  let carry: BHeapNode<T> | null = null;
  for (let i = 0; i < x.length || i < y.length; i++) {
    let xi = x[i];
    let yi = y[i];

    if (xi && yi) {
      bnum[i] = carry;
      carry = link(xi, yi, cmp);
    } else if (!xi && !yi) {
      bnum[i] = carry;
      carry = null;
    } else {
      let node = (xi ? xi : yi) as BHeapNode<T>;
      if (carry) {
        carry = link(node, carry, cmp);
        bnum[i] = null;
      } else {
        bnum[i] = node;
      }
    }
  }

  if (carry) {
    bnum.push(carry);
  }

  return bnum;
}

function link<T>(x: BHeapNode<T>, y: BHeapNode<T>, cmp: Cmp<T>): BHeapNode<T> {
  console.assert(x.degree === y.degree);

  if (cmp(x.key, y.key)) {
    x.preInsert(y);
    return x;
  } else {
    y.preInsert(x);
    return y;
  }
}
