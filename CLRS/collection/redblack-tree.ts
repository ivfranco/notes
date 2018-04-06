import { SearchTreeNode, treeInsert } from "./tree";

enum Color {
  BLACK,
  RED
}

const BLACK = Color.BLACK;
const RED = Color.RED;

class RBTree<T> {
  root: RBNode<T> | null;

  constructor() {
    this.root = null;
  }

  private leftRotate(x: RBNode<T>) {
    if (x.right === null) {
      throw "Error: left rotate with no right child";
    }

    let y = x.right;
    //  each key in left subtree of y is greater than x
    //  left subtree of y should be right subtree of x
    x.right = y.left;
    if (y.left !== null) {
      y.left.parent = x;
    }
    //  put y in the position of x
    y.parent = x.parent;
    //  fix the pointer of the parent of x, if exist
    if (x.parent === null) {
      this.root = y;
    } else if (x === x.parent.left) {
      x.parent.left = y;
    } else {
      x.parent.right = y;
    }
    //  fix the edge between x and y
    y.left = x;
    x.parent = y;
  }

  private rightRotate(y: RBNode<T>) {
    if (y.left === null) {
      throw "Error: right rotate with no left child";
    }

    let x = y.left;
    //  each key in right subtree of x is smaller than y
    //  left subtree of y should be right subtree of x
    y.left = x.right;
    if (x.right !== null) {
      x.right.parent = y;
    }
    x.parent = y.parent;
    if (y.parent === null) {
      this.root = x;
    } else if (y === y.parent.left) {
      y.parent.left = x;
    } else {
      y.parent.right = x;
    }
    x.right = y;
    y.parent = x;
  }

  insert(k: T) {
    let root = this.root;
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

  insertFixup(z: RBNode<T>) {

  }

  show(): string {
    if (this.root === null) {
      return "Empty tree";
    } else {
      return this.root.show();
    }
  }
}

class RBNode<T> extends SearchTreeNode<T> {
  key: T;
  color: Color;
  parent: RBNode<T> | null;
  left: RBNode<T> | null;
  right: RBNode<T> | null;

  constructor(k: T, color: Color) {
    super();
    this.key = k;
    this.color = color;
    this.parent = null;
    this.left = null;
    this.right = null;
  }

  nodeStringify(): string {
    let color_bit = isBlack(this) ? "b" : "r";
    return this.key.toString() + color_bit;
  }
}

function isBlack<T>(x: RBNode<T> | null): boolean {
  if (x === null) {
    return true;
  } else {
    return x.color === BLACK;
  }
}

function isRed<T>(x: RBNode<T> | null): boolean {
  return !isBlack(x);
}