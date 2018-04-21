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
      this.root = splitRoot(r, t);
    }

    this.insertNonNull(this.root, k);
  }

  private insertNonNull(x: BTreeNode<T>, k: T) {
    let t = this.t;
    let lt = this.lt;

    while (!x.leaf) {
      let i = glt(x.key, x.n, k, lt) + 1;
      if (x.c[i].n === 2 * t - 1) {
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
        this.deleteFromInternal(x, k, i);
      } else {
        //  or k belongs to x.c[i]
        this.deleteFromChild(x, k, i);
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

  private deleteFromInternal(x: BTreeNode<T>, k: T, i: number) {
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

  private deleteFromChild(x: BTreeNode<T>, k: T, i: number) {
    let y = x.c[i];
    if (x.c[i].n < this.t) {
      y = this.extendChild(x, i);
    }
    this.deleteAt(y, k);
  }

  private extendChild(p: BTreeNode<T>, i: number): BTreeNode<T> {
    let y = p.c[i];
    //  left sibling of y if there is one
    let x = i === 0 ? null : p.c[i - 1];
    //  right sibling of y if there is one
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

  public split(k: T): [BTree<T>, BTree<T>] {
    let t = this.t;
    let [l, g] = split(this.root, k, t, this.lt, this.eq);
    let LT: BTree<T> = new BTree(t);
    LT.root = l;
    let GT: BTree<T> = new BTree(t);
    GT.root = g;
    return [LT, GT];
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
  public height: number;

  constructor() {
    this.key = [];
    this.c = [];
    this.leaf = true;
    this.n = 0;
    this.height = 0;
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
        console.assert(child.height === this.height - 1, "each child should have height one lower than parent");
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
  z.height = y.height;
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

function splitRoot<T>(r: BTreeNode<T>, t: number): BTreeNode<T> {
  let s: BTreeNode<T> = new BTreeNode();
  s.leaf = false;
  s.c[0] = r;
  s.height = r.height + 1;
  splitChild(s, 0, t);
  return s;
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
//  x and y must both be leaves or both not
//  assumes x.n == t - 1 && y.n == t - 1
function merge<T>(x: BTreeNode<T>, k: T, y: BTreeNode<T>) {
  console.assert(x.leaf === y.leaf, "x and y must be both leaves or both not");

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

function join<T>(x: BTreeNode<T>, k: T, y: BTreeNode<T>, t: number): BTreeNode<T> {
  if (x.n === 0 || y.n === 0) {
    //  if one tree is empty, simply insert k into another
    let btree: BTree<T> = new BTree(t);
    btree.root = x.n === 0 ? y : x;
    btree.insert(k);
    return btree.root;
  } else if (x.height === y.height) {
    //  if height equals, both x and y inserted as children of a new node with sole key k
    let r: BTreeNode<T> = new BTreeNode();
    r.key = [k];
    r.c = [x, y];
    r.n = 1;
    r.leaf = false;
    r.height = x.height + 1;
    return r;
  } else if (x.height > y.height) {
    //  if x higher than y, y is inserted as a child of the right-most node in x with height y.height + 1 alongside k
    if (x.n >= 2 * t - 1) {
      x = splitRoot(x, t);
    }
    let r = x;
    while (x.height > y.height + 1) {
      if (x.c[x.n].n >= 2 * t - 1) {
        splitChild(x, x.n, t);
      }
      x = x.c[x.n];
    }
    x.key.push(k);
    x.c.push(y);
    x.n++;
    return r;
  } else {
    //  if y higher than x, x is inserted as a child of the left-most node in y with height x.height + 1 alongside k
    if (y.n >= 2 * t - 1) {
      y = splitRoot(y, t);
    }
    let r = y;
    while (y.height > x.height + 1) {
      if (y.c[0].n >= 2 * t - 1) {
        splitChild(y, 0, t);
      }
      y = y.c[0];
    }
    y.key.unshift(k);
    y.c.unshift(x);
    y.n++;
    return r;
  }
}

//  only defined on 2-3-4 trees
function split<T>(x: BTreeNode<T>, k: T, t: number, lt: Cmp<T>, eq: Cmp<T>): [BTreeNode<T>, BTreeNode<T>] {
  let i = glt(x.key, x.n, k, lt) + 1;
  if (i < x.n && eq(x.key[i], k)) {
    return splitAroundKey(x, i);
  } else {
    let [l_curr, g_curr] = splitAroundChild(x, i);
    let [l, g] = split(x.c[i], k, t, lt, eq);

    if (i - 1 >= 0) {
      l = join(l_curr, x.key[i - 1], l, t);
    }

    if (i < x.n) {
      g = join(g, x.key[i], g_curr, t);
    }

    return [l, g];
  }
}

function splitAroundKey<T>(x: BTreeNode<T>, i: number): [BTreeNode<T>, BTreeNode<T>] {
  let y: BTreeNode<T> = new BTreeNode();
  let z: BTreeNode<T> = new BTreeNode();

  y.key = x.key.slice(0, i);
  y.c = x.c.slice(0, i + 1);
  y.n = i;
  y.leaf = x.leaf;
  y.height = x.height;
  y = foldEmpty(y);

  z.key = x.key.slice(i + 1);
  z.c = x.c.slice(i + 1);
  z.n = x.n - i - 1;
  z.leaf = x.leaf;
  z.height = x.height;
  z = foldEmpty(z);

  return [y, z];
}

function splitAroundChild<T>(x: BTreeNode<T>, i: number): [BTreeNode<T>, BTreeNode<T>] {
  let y: BTreeNode<T> = new BTreeNode();
  let z: BTreeNode<T> = new BTreeNode();

  y.key = x.key.slice(0, i - 1);
  y.c = x.c.slice(0, i);
  y.n = Math.max(i - 1, 0);
  y.leaf = x.leaf;
  y.height = x.height;
  y = foldEmpty(y);

  z.key = x.key.slice(i + 1);
  z.c = x.c.slice(i + 1);
  z.n = Math.max(x.n - i - 1, 0);
  z.leaf = x.leaf;
  z.height = x.height;
  z = foldEmpty(z);

  return [y, z];
}

function foldEmpty<T>(x: BTreeNode<T>): BTreeNode<T> {
  if (x.n === 0 && x.c.length === 1) {
    return x.c[0];
  } else {
    return x;
  }
}
