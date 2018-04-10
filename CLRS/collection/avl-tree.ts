export {
  AVLTree,
};

import { SearchTree, SearchTreeNode } from "./tree";

class AVLTree<T> extends SearchTree<T, AVLTreeNode<T>> {
  public root: AVLTreeNode<T> | null;

  constructor() {
    super();
    this.root = null;
  }

  private balance(x: AVLTreeNode<T>) {
    if (h(x.right) >= h(x.left) + 2) {
      let y = x.right as AVLTreeNode<T>;
      if (h(y.left) <= h(y.right)) {
        this.leftRotate(x);
        fixHeight(x);
        fixHeight(y);
      } else {
        let z = y.left as AVLTreeNode<T>;
        this.rightRotate(y);
        this.leftRotate(x);
        fixHeight(x);
        fixHeight(y);
        fixHeight(z);
      }
    } else if (h(x.left) >= h(x.right) + 2) {
      // symmetric
      let y = x.left as AVLTreeNode<T>;
      if (h(y.right) <= h(y.left)) {
        this.rightRotate(x);
        fixHeight(x);
        fixHeight(y);
      } else {
        let z = y.right as AVLTreeNode<T>;
        this.leftRotate(y);
        this.rightRotate(x);
        fixHeight(x);
        fixHeight(y);
        fixHeight(z);
      }
    }
  }

  private insertAt(z: AVLTreeNode<T>, x: AVLTreeNode<T>) {
    if (z.key <= x.key) {
      if (x.left === null) {
        x.left = z;
        z.parent = x;
      } else {
        this.insertAt(z, x.left);
      }
    } else {
      if (x.right === null) {
        x.right = z;
        z.parent = x;
      } else {
        this.insertAt(z, x.right);
      }
    }

    fixHeight(x);
    this.balance(x);
  }

  public insert(k: T) {
    let z = new AVLTreeNode(k);
    if (this.root) {
      this.insertAt(z, this.root);
    } else {
      this.root = z;
    }
  }

  public delete(z: AVLTreeNode<T>) {
    throw new Error("Error: Not implemented");
  }
}

class AVLTreeNode<T> extends SearchTreeNode<T> {
  public key: T;
  public parent: this | null;
  public left: this | null;
  public right: this | null;
  public h: number;

  public nodeStringify(): string {
    return `${this.key}, ${this.h}`;
  }

  constructor(k: T) {
    super();
    this.key = k;
    this.parent = null;
    this.left = null;
    this.right = null;
    this.h = 0;
  }
}

function h<T>(x: AVLTreeNode<T> | null): number {
  if (x === null) {
    return -1;
  } else {
    return x.h;
  }
}

function recalcHeight<T>(x: AVLTreeNode<T>): number {
  return 1 + Math.max(h(x.left), h(x.right));
}

function fixHeight<T>(x: AVLTreeNode<T>) {
  x.h = recalcHeight(x);
}
