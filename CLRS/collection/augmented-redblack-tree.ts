export {
  OSTree
};

import { AbstractRBTree, RBNode, Color } from "./redblack-tree";
import { treeInsert } from "./tree";

abstract class ARBTree<T, F, N extends ARBNode<T, F>> extends AbstractRBTree<T, N> {
  leftRotate(x: N) {
    super.leftRotate(x);
    let y = <N>x.parent;
    //  order of x and y here is crucial
    //  augment must be fixed from children to parent
    x.fixAugment();
    y.fixAugment();
  }

  rightRotate(y: N) {
    super.rightRotate(y);
    let x = <N>y.parent;
    //  order of x and y here is crucial
    //  augment must be fixed from children to parent
    y.fixAugment();
    x.fixAugment();
  }

  insert(k: T) {
    let root = this.root;
    // initially black, the proper color for root
    let z = this.factory(k, Color.BLACK);

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

  delete(z: N) {
    let [y_original_color, x, p] = this.preDelete(z);
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
  f: F;
  constructor(k: T, c: Color) {
    super(k, c);
    this.f = this.calcAugment();
  }

  abstract calcAugment(): F;

  fixAugment() {
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
  factory(k: T, c: Color): OSNode<T> {
    return new OSNode(k, c);
  }

  le(a: T, b: T): boolean {
    return a <= b;
  }

  select(i: number): OSNode<T> {
    if (!this.root || i < 0 || i > this.root.size()) {
      throw "Error: Out of boundary access";
    }
    //  if the order i falls within the boundary, the node must exist

    let x = this.root;
    let r = size(x.left) + 1;
    while (i !== r) {
      if (i < r) {
        //  must exist, cannot fail
        x = <OSNode<T>>x.left;
      } else {
        //  must exist, cannot fail
        x = <OSNode<T>>x.right;
        i = i - r;
      }
      r = size(x.left) + 1;
    }

    return x;
  }

  diagose() {
    if (this.root) {
      this.root.diagnose();
    }
  }
}

class OSNode<T> extends ARBNode<T, number> {
  nodeStringify(): string {
    return `${this.key}| ${this.size()}`;
  }

  calcAugment(): number {
    let fl = size(this.left);
    let fr = size(this.right);

    return fl + fr + 1;
  }

  size(): number {
    return this.f;
  }

  rank(): number {
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

  sizeTest(): number {
    let size_left = this.left ? this.left.sizeTest() : 0;
    let size_right = this.right ? this.right.sizeTest() : 0;

    let size = size_left + size_right + 1;
    console.assert(this.size() === size, "Calculated size must equal stored size");
    return size;
  }

  diagnose() {
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
    let r = size(x.left) + 1;
    if (k === x.key) {
      return r;
    } else if (x.right === null) {
      return r + 1;
    } else {
      return r + keyRank(k, x.right);
    }
  }
}