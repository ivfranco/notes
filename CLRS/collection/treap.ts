export {
  Treap,
};

import { SearchTree, SearchTreeNode, treeInsert } from "./tree";

class Treap<T> extends SearchTree<T, TreapNode<T>> {
  public root: TreapNode<T> | null;

  constructor() {
    super();
    this.root = null;
  }

  private insertFixup(z: TreapNode<T>) {
    while (z.parent && z.parent.priority > z.priority) {
      if (z === z.parent.left) {
        this.rightRotate(z.parent);
      } else {
        this.leftRotate(z.parent);
      }
    }
  }

  public insert(k: T) {
    let z = new TreapNode(k, Math.random());
    if (this.root) {
      treeInsert(z, this.root, (a, b) => a <= b);
      this.insertFixup(z);
    } else {
      this.root = z;
    }
  }

  public delete(z: TreapNode<T>) {
    throw new Error("Error: Not implemented");
  }

  public diagnose() {
    if (this.root) {
      this.root.diagnose();
    }
  }
}

class TreapNode<T> extends SearchTreeNode<T> {
  public key: T;
  public priority: number;
  public parent: this | null;
  public left: this | null;
  public right: this | null;

  constructor(k: T, p: number) {
    super();
    this.key = k;
    this.priority = p;
    this.parent = null;
    this.left = null;
    this.right = null;
  }

  public nodeStringify(): string {
    let p = Math.floor(this.priority * 100);
    return `${this.key}: ${p}`;
  }

  public diagnose() {
    if (this.left) {
      console.assert(this.key >= this.left.key);
      console.assert(this.priority <= this.left.priority);
      this.left.diagnose();
    }
    if (this.right) {
      console.assert(this.key <= this.right.key);
      console.assert(this.priority <= this.right.priority);
      this.right.diagnose();
    }
  }
}
