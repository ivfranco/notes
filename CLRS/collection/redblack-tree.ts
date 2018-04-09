export {
  AbstractRBTree,
  Color,
  RBTree,
  RBNode
};

import {
  SearchTree,
  SearchTreeNode,
  treeInsert,
  treeMinimum
} from "./tree";

enum Color {
  BLACK,
  RED
}

const BLACK = Color.BLACK;
const RED = Color.RED;

abstract class AbstractRBTree<T, N extends RBNode<T>> extends SearchTree<T, N> {
  root: N | null;

  constructor() {
    super();

    this.root = null;
  }

  protected abstract le(a: T, b: T): boolean;
  protected abstract factory(k: T, c: Color): N;

  insert(k: T) {
    let root = this.root;
    // initially black, the proper color for root
    let z = this.factory(k, BLACK);

    if (root === null) {
      // for empty tree, no fixup is necessary, z will be the new root
      this.root = z;
    } else {
      treeInsert(z, root, this.le);
      z.color = RED;
      this.insertFixup(z);
    }
  }

  protected insertFixup(z: N) {
    while (isRed(z.parent) && z.parent.parent) {
      let p = z.parent;
      let pp = z.parent.parent;
      if (p === pp.left) {
        let y = pp.right;
        if (isRed(y)) {
          p.color = BLACK;
          y.color = BLACK;
          pp.color = RED;
          z = pp;
        } else {
          if (z === p.right) {
            z = p;
            this.leftRotate(z);
          }
          p = <N>z.parent;
          pp = <N>p.parent;
          p.color = BLACK;
          pp.color = RED;
          this.rightRotate(pp);
        }
      } else {
        let y = pp.left;
        if (isRed(y)) {
          p.color = BLACK;
          y.color = BLACK;
          pp.color = RED;
          z = pp;
        } else {
          if (z === p.left) {
            z = p;
            this.rightRotate(z);
          }
          p = <N>z.parent;
          pp = <N>p.parent;
          p.color = BLACK;
          pp.color = RED;
          this.leftRotate(pp);
        }
      }
    }
    if (this.root) {
      this.root.color = BLACK;
    }
  }

  delete(z: N) {
    let [y_original_color, x, p] = this.preDelete(z);
    if (y_original_color === BLACK) {
      this.deleteFixup(x, p);
    }
  }

  protected preDelete(z: N): [Color, N | null, N | null] {
    let y = z;
    let y_original_color = y.color;

    let x: N | null;
    let p: N | null;
    if (z.left === null) {
      x = z.right;
      p = this.transplant(z, z.right);
    } else if (z.right === null) {
      x = z.left;
      p = this.transplant(z, z.left);
    } else {
      y = <N>treeMinimum(z.right);
      y_original_color = y.color;
      x = y.right;
      if (y.parent === z) {
        if (x) {
          x.parent = y;
        }
        p = y;
      } else {
        p = this.transplant(y, y.right);
        y.right = z.right;
        y.right.parent = y;
      }
      this.transplant(z, y);
      y.left = z.left;
      y.left.parent = y;
      y.color = z.color;
    }

    return [y_original_color, x, p];
  }

  protected deleteFixup(x: N | null, p: N | null) {
    while (x !== this.root && isBlack(x) && p) {
      //  if x is not root, parent of x exists
      if (x === p.left) {
        //  as x is double-black, w cannot be T.nil
        //  otherwise the black height won't equal
        let w = <N>p.right;
        if (isRed(w)) {
          w.color = BLACK;
          p.color = RED;
          this.leftRotate(p);
          //  w is red, x.parent must be black
          //  as x is double-black, none of w's children can be T.nil
          //  otherwise the black height won't equal
          w = <N>p.right;
        }
        if (isBlack(w.left) && isBlack(w.right)) {
          w.color = RED;
          x = p;
          p = p.parent;
        } else {
          if (isRed(w.left)) {
            w.left.color = BLACK;
            w.color = RED;
            this.rightRotate(w);
            //  w.left is red and rotated to p.right, so p.right cannot be T.nil
            w = <N>p.right;
          }
          w.color = p.color;
          p.color = BLACK;
          //  w.right must be red, so w.right cannot be T.nil
          //  conditional here only to bypass the type system
          if (w.right) {
            w.right.color = BLACK;
          }
          this.leftRotate(p);
          x = this.root;
        }
      } else {
        //  symmetric
        let w = <N>p.left;
        if (isRed(w)) {
          w.color = BLACK;
          p.color = RED;
          this.rightRotate(p);
          w = <N>p.left;
        }
        if (isBlack(w.left) && isBlack(w.right)) {
          w.color = RED;
          x = p;
          p = p.parent;
        } else {
          if (isRed(w.right)) {
            w.right.color = BLACK;
            w.color = RED;
            this.leftRotate(w);
            w = <N>p.left;
          }
          w.color = p.color;
          p.color = BLACK;
          if (w.left) {
            w.left.color = BLACK;
          }
          this.rightRotate(p);
          x = this.root;
        }
      }
    }
    if (x) {
      x.color = BLACK;
    }
  }
}

class RBTree<T> extends AbstractRBTree<T, RBNode<T>> {
  le(a: T, b: T): boolean {
    return a <= b;
  }

  factory(k: T, c: Color): RBNode<T> {
    return new RBNode(k, c);
  }

  constructor() {
    super();
  }
}

class RBNode<T> extends SearchTreeNode<T> {
  key: T;
  color: Color;
  parent: this | null;
  left: this | null;
  right: this | null;

  constructor(k: T, color: Color) {
    super();
    this.key = k;
    this.color = color;
    this.parent = null;
    this.left = null;
    this.right = null;
  }

  protected nodeStringify(): string {
    let color_bit = isBlack(this) ? "b" : "r";
    return `${this.key}${color_bit}`;
  }

  protected bhTest(): number {
    let bh_left = this.left ? this.left.bhTest() : 1;
    let bh_right = this.right ? this.right.bhTest() : 1;

    console.assert(bh_left === bh_right, `Black height must equal`);
    if (this.color === BLACK) {
      return bh_left + bh_right + 1;
    } else {
      return bh_left + bh_right;
    }
  }

  diagnose() {
    // property 1 is guarenteed by the type system

    // property 2
    if (this.parent === null) {
      console.assert(this.color === BLACK, "Root must be black");
    }

    // property 3 is implicit, isBlack(null) == true

    // property 4
    if (this.color === RED) {
      console.assert(isBlack(this.left) && isBlack(this.right), "Red node must have black children");
    }

    // property 5
    this.bhTest();
  }
}

function isBlack<T, N extends RBNode<T>>(x: N | null): boolean {
  if (x === null) {
    return true;
  } else {
    return x.color === BLACK;
  }
}

function isRed<T, N extends RBNode<T>>(x: N | null): x is N {
  return !isBlack(x);
}
