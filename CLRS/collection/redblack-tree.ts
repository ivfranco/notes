export {
  RBTree,
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

class RBTree<T> extends SearchTree<T, RBNode<T>> {
  root: RBNode<T> | null;

  constructor() {
    super();

    this.root = null;
  }

  insert(k: T) {
    let root = this.root;
    // initially black, the proper color for root
    let z = new RBNode(k, BLACK);

    if (root === null) {
      // for empty tree, no fixup is necessary, z will be the new root
      this.root = z;
    } else {
      treeInsert(z, root);
      z.color = RED;
      this.insertFixup(z);
    }
  }

  private insertFixup(z: RBNode<T>) {
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
          p = <RBNode<T>>z.parent;
          pp = <RBNode<T>>p.parent;
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
          p = <RBNode<T>>z.parent;
          pp = <RBNode<T>>p.parent;
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

  delete(z: RBNode<T>) {
    let y = z;
    let y_original_color = y.color;

    let x: RBNode<T> | null;
    let p: RBNode<T> | null;
    if (z.left === null) {
      x = z.right;
      p = this.transplant(z, z.right);
    } else if (z.right === null) {
      x = z.left;
      p = this.transplant(z, z.left);
    } else {
      y = treeMinimum(z.right);
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

    if (y_original_color === BLACK) {
      this.deleteFixup(x, p);
    }
  }

  private deleteFixup(x: RBNode<T> | null, p: RBNode<T> | null) {
    while (x !== this.root && isBlack(x) && p) {
      //  if x is not root, parent of x exists
      if (x === p.left) {
        //  as x is double-black, w cannot be T.nil
        //  otherwise the black height won't equal
        let w = <RBNode<T>>p.right;
        if (isRed(w)) {
          w.color = BLACK;
          p.color = RED;
          this.leftRotate(p);
          //  w is red, x.parent must be black
          //  as x is double-black, none of w's children can be T.nil
          //  otherwise the black height won't equal
          w = <RBNode<T>>p.right;
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
            w = <RBNode<T>>p.right;
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
        let w = <RBNode<T>>p.left;
        if (isRed(w)) {
          w.color = BLACK;
          p.color = RED;
          this.rightRotate(p);
          w = <RBNode<T>>p.left;
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
            w = <RBNode<T>>p.left;
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
}

function isBlack<T>(x: RBNode<T> | null): boolean {
  if (x === null) {
    return true;
  } else {
    return x.color === BLACK;
  }
}

function isRed<T>(x: RBNode<T> | null): x is RBNode<T> {
  return !isBlack(x);
}
