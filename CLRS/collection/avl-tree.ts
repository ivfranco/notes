export {
  AVLTree
};

import { SearchTreeNode, SearchTree } from "./tree";

class AVLTree<T> extends SearchTree<T, AVLTreeNode<T>> {
  root: AVLTreeNode<T> | null;

  constructor() {
    super();
    this.root = null;
  }

  private balance(x: AVLTreeNode<T>) {
    if (h(x.right) >= h(x.left) + 2) {
      let y = <AVLTreeNode<T>>x.right;
      if (h(y.left) <= h(y.right)) {
        this.leftRotate(x);
        fixHeight(x);
        fixHeight(y);
      } else {
        let z = <AVLTreeNode<T>>y.left;
        this.rightRotate(y);
        this.leftRotate(z);
        fixHeight(x);
        fixHeight(y);
        fixHeight(z);
      }
    } else if (h(x.left) >= h(x.right) + 2) {
      let y = <AVLTreeNode<T>>x.left;
      if (h(y.right) <= h(y.left)) {
        this.rightRotate(x);
        fixHeight(x);
        fixHeight(y);
      } else {
        let z = <AVLTreeNode<T>>y.right;
        this.leftRotate(y);
        this.rightRotate(z);
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
  }

  insert(k: T) {
    let z = new AVLTreeNode(k);
    if (this.root) {
      this.insertAt(z, this.root);
    } else {
      this.root = z;
    }
  }

  delete(z: AVLTreeNode<T>) {
    throw "Error: Not implemented"
  }
}

class AVLTreeNode<T> extends SearchTreeNode<T> {
  key: T;
  parent: this | null;
  left: this | null;
  right: this | null;
  h: number;

  nodeStringify(): string {
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