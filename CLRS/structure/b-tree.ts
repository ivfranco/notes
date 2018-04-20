export {
  BTree,
  BTreeNode,
};

import { isSorted } from "../util";

class BTree<T> {
  public root: BTreeNode<T>;
  protected t: number;

  constructor(t: number) {
    console.assert(t >= 2);
    this.root = new BTreeNode();
    this.t = t;
  }

  protected lt(a: T, b: T): boolean {
    return a < b;
  }

  protected eq(a: T, b: T): boolean {
    return a === b;
  }

  public isEmpty(): boolean {
    return this.root.n === 0;
  }

  public search(k: T): [BTreeNode<T>, number] | null {
    let x = this.root;
    let lt = this.lt;
    let eq = this.eq;

    while (x) {
      let i = glt(x.key, x.n, k, lt) + 1;
      if (i < x.n && eq(k, x.key[i])) {
        return [x, i];
      } else {
        x = x.c[i];
      }
    }

    return null;
  }

  public insert(k: T) {
    let r = this.root;
    let t = this.t;

    if (r.n === 2 * t - 1) {
      let s: BTreeNode<T> = new BTreeNode();
      this.root = s;
      s.leaf = false;
      s.c[0] = r;
      // console.log(`About to insert ${k}`);
      // console.log(this.show());
      splitChild(s, 0, t);
    }

    this.insertNonNull(this.root, k);
  }

  private insertNonNull(x: BTreeNode<T>, k: T) {
    let t = this.t;
    let lt = this.lt;

    while (!x.leaf) {
      let i = glt(x.key, x.n, k, lt) + 1;
      if (x.c[i].n === 2 * t - 1) {
        // console.log(`About to insert ${k}`);
        // console.log(this.show());
        splitChild(x, i, t);
        if (k > x.key[i]) {
          i++;
        }
      }
      x = x.c[i];
    }

    let i = x.n - 1;
    while (i >= 0 && k < x.key[i]) {
      x.key[i + 1] = x.key[i];
      i--;
    }
    x.key[i + 1] = k;
    x.n++;
  }

  public delete(k: T) {
    if (this.root.n > 0) {
      this.deleteAt(this.root, k);
    }
  }

  private deleteAt(x: BTreeNode<T>, k: T) {
    if (x.leaf) {
      this.deleteFromLeaf(x, k);
    } else {
      let lt = this.lt;
      let eq = this.eq;

      let i = glt(x.key, x.n, k, lt) + 1;
      if (i < x.n && eq(x.key[i], k)) {
        //  either x.key[i] == k
        this.deleteCase2(x, k, i);
      } else {
        //  or k belongs to x.c[i]
        this.deleteCase3(x, k, i);
      }
    }
  }

  private deleteFromLeaf(x: BTreeNode<T>, k: T) {
    let eq = this.eq;
    let lt = this.lt;
    let i = glt(x.key, x.n, k, lt) + 1;
    if (i < x.n && eq(x.key[i], k)) {
      x.key.splice(i, 1);
      x.n--;
    } else {
      console.error("Deletion: Key not present");
    }
  }

  private deleteCase2(x: BTreeNode<T>, k: T, i: number) {
    let y = x.c[i];
    let z = x.c[i + 1];
    let t = this.t;

    if (y.n >= t) {
      let pred = this.deleteMaximum(y);
      x.key[i] = pred;
    } else if (z.n >= t) {
      let succ = this.deleteMinimum(z);
      x.key[i] = succ;
    } else {
      merge(y, k, z);
      x.key.splice(i, 1);
      x.c.splice(i + 1, 1);
      x.n--;
      this.fixRoot();
      this.deleteAt(y, k);
    }
  }

  private deleteMaximum(x: BTreeNode<T>): T {
    if (x.leaf) {
      x.n--;
      return x.key.pop() as T;
    } else {
      let y = x.c[x.n];
      if (y.n < this.t) {
        y = this.extendChild(x, x.n);
      }
      return this.deleteMaximum(y);
    }
  }

  private deleteMinimum(x: BTreeNode<T>): T {
    if (x.leaf) {
      x.n--;
      let k = x.key[0];
      x.key.splice(0, 1);
      return k;
    } else {
      let y = x.c[0];
      if (y.n < this.t) {
        y = this.extendChild(x, 0);
      }
      return this.deleteMinimum(y);
    }
  }

  private deleteCase3(x: BTreeNode<T>, k: T, i: number) {
    if (x.c[i].n >= this.t) {
      this.deleteAt(x.c[i], k);
    } else {
      let y = this.extendChild(x, i);
      this.deleteAt(y, k);
    }
  }

  private extendChild(p: BTreeNode<T>, i: number): BTreeNode<T> {
    let y = p.c[i];
    let x = i === 0 ? null : p.c[i - 1];
    let z = i === p.n ? null : p.c[i + 1];
    let t = this.t;

    if (x && x.n >= t) {
      y.key.unshift(p.key[i - 1]);
      y.c.unshift(x.c[x.n]);
      y.n++;
      p.key[i - 1] = x.key[x.n - 1];
      x.fit(x.n - 1);
      return y;
    } else if (z && z.n >= t) {
      y.key.push(p.key[i]);
      y.c.push(z.c[0]);
      y.n++;
      p.key[i] = z.key[0];
      z.key.splice(0, 1);
      z.c.splice(0, 1);
      z.n--;
      return y;
    } else if (x) {
      merge(x, p.key[i - 1], y);
      p.key.splice(i - 1, 1);
      p.c.splice(i, 1);
      p.n--;
      this.fixRoot();
      return x;
    } else if (z) {
      merge(y, p.key[i], z);
      p.key.splice(i, 1);
      p.c.splice(i + 1, 1);
      p.n--;
      this.fixRoot();
      return y;
    } else {
      throw Error("Unreachable");
    }
  }

  private fixRoot() {
    if (this.root.n === 0) {
      let child = this.root.c[0];
      this.root = child;
    }
  }

  public show(): string {
    if (this.root.n > 0) {
      return this.root.show();
    } else {
      return "Empty BTree";
    }
  }

  public diagnose() {
    if (!this.isEmpty()) {
      this.root.diagnose(this.t, true);
    }
  }
}

class BTreeNode<T> {
  public key: T[];
  public c: this[];
  public leaf: boolean;
  public n: number;

  constructor() {
    this.key = [];
    this.c = [];
    this.leaf = true;
    this.n = 0;
  }

  //  shrink c and key with a new smaller n
  public fit(n: number) {
    console.assert(this.n >= n);
    let c = this.c;
    let key = this.key;

    if (c.length > n + 1) {
      c.length = n + 1;
    }
    if (key.length > n) {
      key.length = n;
    }

    this.n = n;
  }

  private keyStringify(i: number): string {
    return "" + this.key[i];
  }

  private toLines(): string[] {
    function prepend(sub: string[], top: boolean, bottom: boolean): string[] {
      let mid = Math.floor(sub.length / 2);
      return sub.map((line, i) => {
        if (i === mid) {
          if (top) {
            return "┌─" + line;
          } else if (bottom) {
            return "└─" + line;
          } else {
            return "├─" + line;
          }
        } else if (i < mid && top || i > mid && bottom) {
          return "  " + line;
        } else {
          return "│ " + line;
        }
      });
    }

    let n = this.n;
    let lines: string[] = [];

    let subs: string[][] = new Array(n + 1);
    if (this.leaf) {
      // for (let i = 0; i <= n; i++) {
      //   if (i === n || i === 0) {
      //     subs[i] = ["  "];
      //   } else {
      //     subs[i] = ["│ "];
      //   }
      // }
      subs.fill([]);
    } else {
      for (let i = 0; i <= n; i++) {
        if (i === n) {
          subs[i] = prepend(this.c[i].toLines(), true, false);
        } else if (i === 0) {
          subs[i] = prepend(this.c[i].toLines(), false, true);
        } else {
          subs[i] = prepend(this.c[i].toLines(), false, false);
        }
      }
    }

    lines.push(...subs[n]);
    for (let i = n - 1; i >= 0; i--) {
      let key = this.keyStringify(i);
      if (lines.length === 0 && i === 0) {
        lines.push("─" + key);
      } else if (lines.length === 0) {
        lines.push("┌" + key);
      } else if (i === 0 && subs[0].length === 0) {
        lines.push("└" + key);
      } else {
        lines.push("├" + key);
      }

      lines.push(...subs[i]);
    }

    return lines;
  }

  public show(): string {
    return this.toLines().join("\n");
  }

  public diagnose(t: number, isRoot: boolean) {
    let key = this.key;
    let n = this.n;
    let c = this.c;

    console.assert(isSorted(key.slice(0, n)), "Key must be sorted");
    console.assert(key.length === n, "n should be the number of keys");
    console.assert(n <= 2 * t - 1, "n <= 2t - 1");

    if (!isRoot) {
      console.assert(n >= t - 1, "n >= t - 1 for non-root nodes");
    }

    if (!this.leaf) {
      console.assert(c.length === n + 1, "internal node should have n + 1 children");
      for (let child of c) {
        child.diagnose(t, false);
      }
    }
  }
}

function splitChild<T>(x: BTreeNode<T>, i: number, t: number) {
  let z: BTreeNode<T> = new BTreeNode();
  let y = x.c[i];
  //  y must be full
  console.assert(y.n === 2 * t - 1);

  z.leaf = y.leaf;
  z.n = t - 1;
  for (let j = 0; j < t - 1; j++) {
    z.key[j] = y.key[j + t];
  }
  if (!y.leaf) {
    for (let j = 0; j < t; j++) {
      z.c[j] = y.c[j + t];
    }
  }

  for (let j = x.n; j >= i + 1; j--) {
    x.c[j + 1] = x.c[j];
    x.key[j] = x.key[j - 1];
  }
  x.c[i + 1] = z;
  x.key[i] = y.key[t - 1];
  x.n++;
  y.fit(t - 1);
}

type Cmp<T> = (a: T, b: T) => boolean;

//  greatest less than, linear
function glt<T>(A: T[], n: number, k: T, lt: Cmp<T>): number {
  let i = 0;
  while (lt(A[i], k) && i < n) {
    i++;
  }
  return i - 1;
}

//  merge k and y into x
//  x and y must both be leaves or not
//  assumes x.n == t - 1 && y.n == t - 1
function merge<T>(x: BTreeNode<T>, k: T, y: BTreeNode<T>) {
  console.assert(x.leaf === y.leaf, "x and y must be both leaves or not");

  x.key[x.n] = k;
  for (let i = 0; i < y.n; i++) {
    x.key[x.n + i + 1] = y.key[i];
  }
  if (!x.leaf) {
    for (let i = 0; i <= y.n; i++) {
      x.c[x.n + i + 1] = y.c[i];
    }
  }
  x.n += y.n + 1;
}
