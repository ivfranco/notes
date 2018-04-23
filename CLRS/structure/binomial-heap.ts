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
    if (this.child && other.child) {
      let bnum1 = this.child.toBNumber();
      let bnum2 = other.child.toBNumber();
      let sum = add(bnum1, bnum2, this.cmp);
      this.child = BHeapNode.fromBNumber(sum);
    }
  }

  public minimum(): BHeapNode<T> | null {
    let cmp = this.cmp;
    if (this.child) {
      let min = this.child;
      for (let c of this.child.siblings()) {
        if (cmp(c.key, min.key)) {
          min = c;
        }
      }
      return min;
    } else {
      return null;
    }
  }

  public extractMin(): BHeapNode<T> | null {
    if (!this.child) {
      return null;
    }

    let min = this.minimum() as BHeapNode<T>;
    this.extractRoot(min);
    return min;
  }

  //  delete a root from the root list
  //  combines its child list with the remaining root list
  private extractRoot(r: BHeapNode<T>) {
    if (r.isSingleton()) {
      this.child = r.child;
      if (this.child) {
        for (let c of this.child.siblings()) {
          c.parent = null;
        }
      }
    } else {
      let right = r.right;
      r.remove();
      let bnum1 = r.child ? r.child.toBNumber() : [];
      let bnum2 = right.toBNumber();
      let sum = add(bnum1, bnum2, this.cmp);
      this.child = BHeapNode.fromBNumber(sum);
    }

    this.n--;
  }

  public decreaseKey(x: BHeapNode<T>, k: T) {
    let cmp = this.cmp;
    if (cmp(x.key, k)) {
      throw Error("Error: new key is greater than current key");
    }

    x.key = k;
    this.fixup(x, false);
  }

  private fixup(x: BHeapNode<T>, isDelete: boolean): BHeapNode<T> {
    let cmp = this.cmp;
    let y = x.parent;
    while (y && (isDelete || cmp(x.key, y.key))) {
      let temp = new BHeapNode(y.key);
      temp.copyFrom(y);
      y.copyFrom(x);
      x.copyFrom(temp);
      y.parent = x;
      if (x.child === x) {
        x.child = y;
      }
      if (this.child === y) {
        this.child = x;
      }
      y = x.parent;
    }

    return x;
  }

  public delete(x: BHeapNode<T>) {
    let root = this.fixup(x, true);
    this.extractRoot(root);
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

  public copyFrom(other: this) {
    if (other.isSingleton()) {
      this.left = this;
      this.right = this;
    } else {
      this.left = other.left;
      this.right = other.right;
    }
    this.parent = other.parent;
    this.child = other.child;
    this.degree = other.degree;
    this.key = other.key;
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
      console.assert(c.parent === this, "parent pointers must be valid");
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

//  combines two root lists of binomial heaps into one, O(lgn) assuming the two heap has n nodes in total
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
