export {
  ARBTree,
  ARBNode,
  OSTree,
  josephus,
};

import { AbstractRBTree, Color, RBNode } from "./redblack-tree";
import { treeInsert } from "./tree";

abstract class ARBTree<T, F, N extends ARBNode<T, F>> extends AbstractRBTree<T, N> {
  public leftRotate(x: N) {
    super.leftRotate(x);
    const y = x.parent as N;
    //  order of x and y here is crucial
    //  augment must be fixed from children to parent
    x.fixAugment();
    y.fixAugment();
  }

  public rightRotate(y: N) {
    super.rightRotate(y);
    const x = y.parent as N;
    //  order of x and y here is crucial
    //  augment must be fixed from children to parent
    y.fixAugment();
    x.fixAugment();
  }

  public insert(k: T) {
    const root = this.root;
    // initially black, the proper color for root
    const z = this.factory(k);

    if (root === null) {
      // for empty tree, no fixup is necessary, z will be the new root
      this.root = z;
    } else {
      treeInsert(z, root, this.le);
      z.color = Color.RED;
      fixAncestors(z);
      this.insertFixup(z);
    }
  }

  public delete(z: N) {
    const [y_original_color, x, p] = this.preDelete(z);
    if (x) {
      fixAncestors(x);
    } else if (p) {
      fixAncestors(p);
      // otherwise the tree is empty
    }
    if (y_original_color === Color.BLACK) {
      this.deleteFixup(x, p);
    }
  }
}

abstract class ARBNode<T, F> extends RBNode<T> {
  public f: F;
  constructor(k: T) {
    super(k);
    this.f = this.calcAugment();
  }

  // this function should only refer information in current node and its children
  protected abstract calcAugment(): F;

  public fixAugment() {
    this.f = this.calcAugment();
  }
}

function fixAncestors<T, F, N extends ARBNode<T, F>>(z: N) {
  let x: N | null = z;
  while (x !== null) {
    x.fixAugment();
    x = x.parent;
  }
}

class OSTree<T> extends ARBTree<T, number, OSNode<T>> {
  public factory(k: T): OSNode<T> {
    return new OSNode(k);
  }

  protected le(a: T, b: T): boolean {
    return a <= b;
  }

  protected eq(a: T, b: T): boolean {
    return a === b;
  }

  public size(): number {
    if (this.root) {
      return this.root.size();
    } else {
      return 0;
    }
  }

  public select(i: number): OSNode<T> {
    if (!this.root || i < 0 || i > this.root.size()) {
      throw new Error("Error: Out of boundary access");
    }
    //  if the order i falls within the boundary, the node must exist

    let x = this.root;
    let r = size(x.left) + 1;
    while (i !== r) {
      if (i < r) {
        //  must exist, cannot fail
        x = x.left as OSNode<T>;
      } else {
        //  must exist, cannot fail
        x = x.right as OSNode<T>;
        i = i - r;
      }
      r = size(x.left) + 1;
    }

    return x;
  }

  public diagose() {
    if (this.root) {
      this.root.diagnose();
    }
  }
}

class OSNode<T> extends ARBNode<T, number> {
  public nodeStringify(): string {
    return `${this.key}| ${this.size()}`;
  }

  protected calcAugment(): number {
    const fl = size(this.left);
    const fr = size(this.right);

    return fl + fr + 1;
  }

  public size(): number {
    return this.f;
  }

  public rank(): number {
    let r = size(this.left) + 1;
    let y = this;

    while (y.parent !== null) {
      if (y === y.parent.right) {
        r += size(y.parent.left) + 1;
      }
      y = y.parent;
    }

    return r;
  }

  public sizeTest(): number {
    const size_left = this.left ? this.left.sizeTest() : 0;
    const size_right = this.right ? this.right.sizeTest() : 0;

    const size_total = size_left + size_right + 1;
    console.assert(this.size() === size_total, "Calculated size must equal stored size");
    return size_total;
  }

  public diagnose() {
    this.sizeTest();
  }
}

function size<T>(x: OSNode<T> | null): number {
  if (x) {
    return x.size();
  } else {
    return 0;
  }
}

function keyRank<T>(k: T, x: OSNode<T>): number {
  if (k < x.key) {
    if (x.left === null) {
      return 1;
    } else {
      return keyRank(k, x.left);
    }
  } else {
    const r = size(x.left) + 1;
    if (k === x.key) {
      return r;
    } else if (x.right === null) {
      return r + 1;
    } else {
      return r + keyRank(k, x.right);
    }
  }
}

function josephus(n: number, m: number): number[] {
  let os: OSTree<number> = new OSTree();

  for (let i = 1; i <= n; i++) {
    os.insert(i);
  }

  let permutation = [];
  //  rank, starting from 0
  let r = 0;
  while (!os.isEmpty()) {
    r = (r + m - 1) % os.size();
    //  OSTree assumes rank starting from 1
    let z = os.select(r + 1);
    permutation.push(z.key);
    os.delete(z);
  }

  return permutation;
}
