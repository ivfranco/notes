export {
  Tree,
  TreeNode,
  printTree,
  printTreeStack,
  printTreeConstant,
  randomTree,
  treeParent
};

import { randomAB, shuffle } from "../util";


abstract class SearchTreeNode<T> {
  abstract key: T;
  abstract parent: SearchTreeNode<T> | null;
  abstract left: SearchTreeNode<T> | null;
  abstract right: SearchTreeNode<T> | null;

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

    let lines = [...right_lines, this.key.toString(), ...left_lines];

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

class Tree<T> {
  root: TreeNode<T> | null;

  constructor() {
    this.root = null;
  }

  isEmpty() {
    return this.root === null;
  }

  search(k: T): TreeNode<T> | null {
    if (this.root === null) {
      return null;
    } else {
      return treeSearch(k, this.root);
    }
  }

  insert(k: T) {
    if (this.root === null) {
      this.root = new TreeNode(k);
    } else {
      treeInsert(k, this.root);
    }
  }

  private transplant(u: TreeNode<T>, v: TreeNode<T> | null) {
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
  parent: TreeNode<T> | null;
  left: TreeNode<T> | null;
  right: TreeNode<T> | null;

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
}

function printTree<T>(node: TreeNode<T>) {
  console.log(node.key);
  if (node.left !== null) {
    printTree(node.left);
  }
  if (node.right !== null) {
    printTree(node.right);
  }
}

function printTreeStack<T>(node: TreeNode<T>) {
  let stack: TreeNode<T>[] = [];

  while (true) {
    console.log(node.key);
    if (node.right !== null) {
      stack.push(node.right);
    }
    if (node.left !== null) {
      node = node.left;
    } else if (stack.length !== 0) {
      node = <TreeNode<T>>stack.pop();
    } else {
      return;
    }
  }
}

function treeSearch<T>(k: T, node: TreeNode<T>): TreeNode<T> | null {
  let cursor: TreeNode<T> | null = node;
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

function treeInsert<T>(k: T, node: TreeNode<T>) {
  let p: TreeNode<T> = node;
  let x: TreeNode<T> | null = node;

  while (x !== null) {
    p = x;
    if (k <= p.key) {
      x = p.left;
    } else {
      x = p.right;
    }
  }

  if (k <= p.key) {
    p.left = new TreeNode(k);
    p.left.parent = p;
  } else {
    p.right = new TreeNode(k);
    p.right.parent = p;
  }
}

function randomTree<T>(A: T[]): TreeNode<T> {
  shuffle(A);
  let node = new TreeNode(A[0]);
  for (let i = 1; i < A.length; i++) {
    treeInsert(A[i], node);
  }
  return node;
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

function treeMinimum<T>(node: TreeNode<T>): TreeNode<T> {
  if (node.left === null) {
    return node;
  } else {
    return treeMinimum(node.left);
  }
}

function treeMaximum<T>(node: TreeNode<T>): TreeNode<T> {
  if (node.right === null) {
    return node;
  } else {
    return treeMaximum(node.right);
  }
}

function treeSuccessor<T>(node: TreeNode<T>): TreeNode<T> | null {
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

function treePredecessor<T>(node: TreeNode<T>): TreeNode<T> | null {
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