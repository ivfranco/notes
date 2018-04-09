export {
  Tree,
  TreeNode,
  SearchTree,
  BinaryTreeNode,
  SearchTreeNode,
  printTree,
  printTreeStack,
  printTreeConstant,
  randomTree,
  treeParent,
  treeInsert,
  treeMinimum
};

import { randomAB, shuffle } from "../util";

type Cmp<T> = (a: T, b: T) => boolean;

abstract class SearchTree<T, N extends SearchTreeNode<T>> {
  abstract root: N | null;

  //  replace u by v, fix the link between u.parent and v
  //  the link between v and children of u is not fixed
  //  parent of u is returned to mimic the behavior of text-book RB-TRANSPLANT which sets parent of T.nil

  protected transplant(u: N, v: N | null): N | null {
    if (u.parent === null) {
      this.root = v;
    } else if (u === u.parent.left) {
      u.parent.left = v;
    } else {
      u.parent.right = v;
    }

    if (v !== null) {
      v.parent = u.parent;
    }

    return u.parent;
  }

  protected leftRotate(x: N) {
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

  protected rightRotate(y: N) {
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

  abstract insert(k: T): void;
  abstract delete(z: N): void;

  search(k: T): N | null {
    if (this.root === null) {
      return null;
    } else {
      return treeSearch(k, this.root);
    }
  }

  show(): string {
    if (this.root === null) {
      return "Empty tree";
    } else {
      return this.root.show();
    }
  }

  height(): number | null {
    if (this.root) {
      return this.root.height();
    } else {
      return null;
    }
  }

  *[Symbol.iterator](): IterableIterator<T> {
    if (this.root) {
      yield* this.root.inorder();
    }
  }
}

abstract class BinaryTreeNode<T> {
  abstract key: T;
  abstract left: this | null;
  abstract right: this | null;

  protected abstract nodeStringify(): string;

  private toLines(): [number, string[]] {
    let [left_top, left_lines] = this.left === null ? [0, []] : this.left.toLines();
    let [right_top, right_lines] = this.right === null ? [0, []] : this.right.toLines();

    left_lines = left_lines.map((s, i) => {
      if (i < left_top) {
        return "│ " + s;
      } else if (i === left_top) {
        return "└─" + s;
      } else {
        return "  " + s;
      }
    });

    right_lines = right_lines.map((s, i) => {
      if (i < right_top) {
        return "  " + s;
      } else if (i === right_top) {
        return "┌─" + s;
      } else {
        return "│ " + s;
      }
    });

    let lines = [...right_lines, this.nodeStringify(), ...left_lines];

    return [right_lines.length, lines];
  }

  show(): string {
    return this.toLines()[1].join("\n");
  }

  height(): number {
    let left_height = this.left === null ? -1 : this.left.height();
    let right_height = this.right === null ? -1 : this.right.height();

    return 1 + Math.max(left_height, right_height);
  }

  *preorder(): IterableIterator<T> {
    yield this.key;
    if (this.left !== null) {
      yield* this.left.preorder();
    }
    if (this.right !== null) {
      yield* this.right.preorder();
    }
  }

  *postorder(): IterableIterator<T> {
    if (this.left !== null) {
      yield* this.left.preorder();
    }
    if (this.right !== null) {
      yield* this.right.preorder();
    }
    yield this.key;
  }
}

abstract class SearchTreeNode<T> extends BinaryTreeNode<T> {
  abstract key: T;
  abstract parent: this | null;
  abstract left: this | null;
  abstract right: this | null;

  *inorder(): IterableIterator<T> {
    let cursor: SearchTreeNode<T> | null = this;

    while (cursor !== null) {
      if (cursor.left !== null) {
        cursor = cursor.left;
      } else if (cursor.right !== null) {
        yield cursor.key;
        cursor = cursor.right;
      } else {
        yield cursor.key;
        // go up
        while (cursor.parent !== null && (cursor.parent.right === cursor || cursor.parent.right === null)) {
          cursor = cursor.parent;
          if (cursor.right === null) {
            yield cursor.key;
          }
        }
        if (cursor.parent === null) {
          cursor = null;
        } else {
          yield cursor.parent.key;
          cursor = cursor.parent.right;
        }
      }
    }
  }
}

class Tree<T> extends SearchTree<T, TreeNode<T>> {
  root: TreeNode<T> | null;

  constructor() {
    super();
    this.root = null;
  }

  insert(k: T) {
    let z = new TreeNode(k);
    if (this.root === null) {
      this.root = z;
    } else {
      treeInsert(z, this.root, (a, b) => a <= b);
    }
  }

  delete(z: TreeNode<T>) {
    if (z.left === null) {
      this.transplant(z, z.right);
    } else if (z.right === null) {
      this.transplant(z, z.left);
    } else {
      let y = treeMinimum(z.right);
      if (y.parent !== z) {
        this.transplant(y, y.right);
        y.right = z.right;
        y.right.parent = y;
      }
      this.transplant(z, y);
      y.left = z.left;
      y.left.parent = y;
    }
  }
}

class TreeNode<T> extends SearchTreeNode<T> {
  key: T;
  parent: this | null;
  left: this | null;
  right: this | null;

  constructor(k: T) {
    super();

    this.key = k;
    this.parent = null;
    this.left = null;
    this.right = null;
  }

  isLeaf(): boolean {
    return this.left === null && this.right === null;
  }

  protected nodeStringify(): string {
    return this.key.toString();
  }
}

function printTree<T>(node: SearchTreeNode<T>) {
  console.log(node.key);
  if (node.left !== null) {
    printTree(node.left);
  }
  if (node.right !== null) {
    printTree(node.right);
  }
}

function printTreeStack<T>(node: SearchTreeNode<T>) {
  let stack: SearchTreeNode<T>[] = [];

  while (true) {
    console.log(node.key);
    if (node.right !== null) {
      stack.push(node.right);
    }
    if (node.left !== null) {
      node = node.left;
    } else if (stack.length !== 0) {
      node = <SearchTreeNode<T>>stack.pop();
    } else {
      return;
    }
  }
}

function treeSearch<T, N extends SearchTreeNode<T>>(k: T, node: N): N | null {
  let cursor: N | null = node;
  while (cursor !== null) {
    if (k === cursor.key) {
      return cursor;
    } else if (k < cursor.key) {
      cursor = cursor.left;
    } else {
      cursor = cursor.right;
    }
  }

  return null;
}

function treeInsert<T>(z: SearchTreeNode<T>, node: SearchTreeNode<T>, le: Cmp<T>) {
  let p: SearchTreeNode<T> = node;
  let x: SearchTreeNode<T> | null = node;

  while (x !== null) {
    p = x;
    if (le(z.key, x.key)) {
      x = p.left;
    } else {
      x = p.right;
    }
  }

  z.parent = p;
  if (le(z.key, p.key)) {
    p.left = z;
  } else {
    p.right = z;
  }
}

function randomTree<T, N extends SearchTreeNode<T>, ST extends SearchTree<T, N>>(tree: ST, A: T[]) {
  shuffle(A);
  for (let k of A) {
    tree.insert(k);
  }
}

class SiblingTreeNode<T> {
  key: T;
  parent: SiblingTreeNode<T> | null;
  left_child: SiblingTreeNode<T> | null;
  right_sibling: SiblingTreeNode<T> | null;

  constructor(k: T) {
    this.key = k;
    this.parent = null;
    this.left_child = null;
    this.right_sibling = null;
  }
}

function printSiblingTree<T>(node: SiblingTreeNode<T>) {
  console.log(node.key);
  if (node.left_child !== null) {
    printSiblingTree(node.left_child);
  }
  if (node.right_sibling !== null) {
    printSiblingTree(node.right_sibling);
  }
}

function printTreeConstant<T>(node: TreeNode<T>) {
  function goUp() {
    if (cursor !== null) {
      // the procedure only calls goUp from a leaf 
      // if cursor is left child of its parent and there is a right child, traverse the right child instead
      // otherwise both left and right child are traversed, go one level up in the tree
      while (cursor.parent !== null && (cursor.parent.right === cursor || cursor.parent.right === null)) {
        //  while either:
        //    the procedure is going up from a right child
        //    the procedure is going up from a sole left child 
        //  then go up one more level
        cursor = cursor.parent;
      }
      if (cursor.parent === null) {
        //  reached the root
        cursor = null;
      } else {
        //  cursor is the left child, parent also has a right child
        //  traverse right branch
        cursor = cursor.parent.right;
      }
    }
  }

  let cursor: TreeNode<T> | null = node;
  while (cursor !== null) {
    console.log(cursor.key);
    if (cursor.left !== null) {
      // if there is a left child, traverse the left child
      cursor = cursor.left;
    } else if (cursor.right !== null) {
      // if there is no left child but a right child, traverse the right child
      cursor = cursor.right;
    } else {
      // current node is a leaf
      goUp();
    }
  }
}

function treeMinimum<T, N extends SearchTreeNode<T>>(node: N): N {
  if (node.left === null) {
    return node;
  } else {
    return treeMinimum(node.left);
  }
}

function treeMaximum<T, N extends SearchTreeNode<T>>(node: N): N {
  if (node.right === null) {
    return node;
  } else {
    return treeMaximum(node.right);
  }
}

function treeSuccessor<T, N extends SearchTreeNode<T>>(node: N): N | null {
  let x = node;
  if (x.right !== null) {
    return treeMinimum(x.right);
  } else {
    let y = x.parent;
    while (y !== null && x === y.right) {
      x = y;
      y = y.parent;
    }
    return y;
  }
}

function treePredecessor<T, N extends SearchTreeNode<T>>(node: N): N | null {
  let x = node;
  if (x.left !== null) {
    return treeMaximum(x.left);
  } else {
    let y = x.parent;
    while (y !== null && x === y.left) {
      x = y;
      y = y.parent;
    }
    return y;
  }
}

function treeInsertRecur<T>(k: T, node: TreeNode<T>) {
  if (k <= node.key) {
    if (node.left === null) {
      node.left = new TreeNode(k);
      node.left.parent = node;
    } else {
      treeInsertRecur(k, node.left);
    }
  } else {
    if (node.right === null) {
      node.right = new TreeNode(k);
      node.right.parent = node;
    } else {
      treeInsertRecur(k, node.right);
    }
  }
}

function treeParent<T>(node: TreeNode<T>, T: Tree<T>): TreeNode<T> | null {
  let root = <TreeNode<T>>T.root;
  let max = treeMaximum(node);
  let succ = treeSuccessor(max);

  if (succ !== null && succ.left === node) {
    return succ;
  }
  if (node === root) {
    return null;
  }

  let cursor = succ === null ? root : <TreeNode<T>>succ.left;
  while (cursor.right !== null && cursor.right !== node) {
    cursor = cursor.right;
  }
  return cursor;
}