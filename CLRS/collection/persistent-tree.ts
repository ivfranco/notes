export {
  PTree,
};

import { BinaryTreeNode } from "./tree";

class PTree<T> {
  public root: PTreeNode<T> | null;

  constructor() {
    this.root = null;
  }

  public insert(k: T): PTree<T> {
    let tree: PTree<T> = new PTree();
    let z = new PTreeNode(k);

    if (this.root === null) {
      tree.root = z;
    } else {
      tree.root = pInsert(z, this.root);
    }

    return tree;
  }

  public show(): string {
    if (this.root === null) {
      return "Empty tree";
    } else {
      return this.root.show();
    }
  }
}
class PTreeNode<T> extends BinaryTreeNode<T> {
  public key: T;
  public left: this | null;
  public right: this | null;

  constructor(k: T) {
    super();
    this.key = k;
    this.left = null;
    this.right = null;
  }

  public nodeStringify(): string {
    return this.key.toString();
  }

  //  a shallow copy of this node
  public copy(): PTreeNode<T> {
    let node = new PTreeNode(this.key);
    node.left = this.left;
    node.right = this.right;
    return node;
  }
}

function pInsert<T>(z: PTreeNode<T>, x: PTreeNode<T>): PTreeNode<T> {
  let copy = x.copy();
  if (z.key <= copy.key) {
    if (copy.left === null) {
      copy.left = z;
    } else {
      copy.left = pInsert(z, copy.left);
    }
  } else {
    if (copy.right === null) {
      copy.right = z;
    } else {
      copy.right = pInsert(z, copy.right);
    }
  }

  return copy;
}
