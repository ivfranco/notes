export {
  AbstractRBTree,
  Color,
  RBTree,
  RBNode,
};

import {
  SearchTree,
  SearchTreeNode,
  treeInsert,
  treeMinimum,
  treePredecessor,
  treeSearch,
  treeSuccessor,
} from "./tree";

enum Color {
  BLACK,
  RED,
}

const BLACK = Color.BLACK;
const RED = Color.RED;

abstract class AbstractRBTree<T, N extends RBNode<T>> extends SearchTree<T, N> {
  public root: N | null;

  protected abstract le(a: T, b: T): boolean;
  protected abstract eq(a: T, b: T): boolean;
  protected abstract factory(k: T): N;

  constructor() {
    super();

    this.root = null;
  }

  public search(k: T): N | null {
    if (this.root) {
      return treeSearch(k, this.root, this.eq, this.le) as N | null;
    } else {
      return null;
    }
  }

  public predecessor(node: N): N | null {
    return treePredecessor(node);
  }

  public successor(node: N): N | null {
    return treeSuccessor(node);
  }

  public insert(k: T): N {
    const root = this.root;
    // initially black, the proper color for root
    const z = this.factory(k);

    if (root === null) {
      // for empty tree, no fixup is necessary, z will be the new root
      this.root = z;
    } else {
      treeInsert(z, root, this.le);
      z.color = RED;
      this.insertFixup(z);
    }

    return z;
  }

  protected insertFixup(z: N) {
    while (isRed(z.parent) && z.parent.parent) {
      let p = z.parent;
      let pp = z.parent.parent;
      if (p === pp.left) {
        const y = pp.right;
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
          p = z.parent as N;
          pp = p.parent as N;
          p.color = BLACK;
          pp.color = RED;
          this.rightRotate(pp);
        }
      } else {
        const y = pp.left;
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
          p = z.parent as N;
          pp = p.parent as N;
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

  public delete(z: N) {
    const [y_original_color, x, p] = this.preDelete(z);
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
      y = treeMinimum(z.right) as N;
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
        let w = p.right as N;
        if (isRed(w)) {
          w.color = BLACK;
          p.color = RED;
          this.leftRotate(p);
          //  w is red, x.parent must be black
          //  as x is double-black, none of w's children can be T.nil
          //  otherwise the black height won't equal
          w = p.right as N;
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
            w = p.right as N;
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
        let w = p.left as N;
        if (isRed(w)) {
          w.color = BLACK;
          p.color = RED;
          this.rightRotate(p);
          w = p.left as N;
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
            w = p.left as N;
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
  public le(a: T, b: T): boolean {
    return a <= b;
  }

  public eq(a: T, b: T): boolean {
    return a === b;
  }

  public factory(k: T): RBNode<T> {
    return new RBNode(k);
  }

  constructor() {
    super();
  }
}

class RBNode<T> extends SearchTreeNode<T> {
  public key: T;
  public color: Color;
  public parent: this | null;
  public left: this | null;
  public right: this | null;

  constructor(k: T) {
    super();
    this.key = k;
    //  all RBNode initially black
    this.color = BLACK;
    this.parent = null;
    this.left = null;
    this.right = null;
  }

  protected nodeStringify(): string {
    const color_bit = isBlack(this) ? "b" : "r";
    return `${this.key}${color_bit}`;
  }

  protected bhTest(): number {
    const bh_left = this.left ? this.left.bhTest() : 1;
    const bh_right = this.right ? this.right.bhTest() : 1;

    console.assert(bh_left === bh_right, `Black height must equal`);
    if (this.color === BLACK) {
      return bh_left + bh_right + 1;
    } else {
      return bh_left + bh_right;
    }
  }

  public diagnose() {
    // property 1 is guaranteed by the type system

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
  return x === null || x.color === BLACK;
}

function isRed<T, N extends RBNode<T>>(x: N | null): x is N {
  return !isBlack(x);
}
