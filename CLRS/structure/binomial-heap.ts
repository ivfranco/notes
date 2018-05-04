export {
  BHeap,
};

import { Cmp } from "../util";
import { HeapNode, MergableHeap } from "./fibonacci-heap";

class BHeap<K> implements MergableHeap<K, null, BHeapNode<K>> {
  public child: BHeapNode<K> | null;
  public n: number;

  constructor() {
    this.child = null;
    this.n = 0;
  }

  protected cmp(a: K, b: K): boolean {
    return a < b;
  }

  public insert(k: K) {
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

  public minimum(): BHeapNode<K> | null {
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

  public extractMin(): BHeapNode<K> | null {
    if (!this.child) {
      return null;
    }

    let min = this.minimum() as BHeapNode<K>;
    this.extractRoot(min);
    return min;
  }

  //  delete a root from the root list
  //  combines its child list with the remaining root list
  private extractRoot(r: BHeapNode<K>) {
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

  public decreaseKey(x: BHeapNode<K>, k: K) {
    let cmp = this.cmp;
    if (cmp(x.key, k)) {
      throw Error("Error: new key is greater than current key");
    }

    x.key = k;
    this.fixup(x, false);
  }

  private fixup(x: BHeapNode<K>, isDelete: boolean): BHeapNode<K> {
    let cmp = this.cmp;
    let y = x.parent;
    while (y && (isDelete || cmp(x.key, y.key))) {
      let temp = x.key;
      x.key = y.key;
      y.key = temp;
      x = y;
      y = y.parent;
    }

    return x;
  }

  public delete(x: BHeapNode<K>) {
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

class BHeapNode<K> extends HeapNode<K, null> {
  public static fromBNumber<K>(bnum: BNumber<K>): BHeapNode<K> {
    let nodes = bnum.filter(n => n != null) as Array<BHeapNode<K>>;
    if (nodes.length === 0) {
      throw Error("Error: Empty node list is not well defined");
    }
    return nodes.reduce((list, node) => {
      list.prepend(node);
      return node;
    }) as BHeapNode<K>;
  }

  constructor(k: K) {
    super(k, null);
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

  public toBNumber(): BNumber<K> {
    let bnum: BNumber<K> = [];
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

  public diagnose(cmp: Cmp<K>): number {
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
