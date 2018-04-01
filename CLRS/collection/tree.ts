export {
  Tree,
  TreeNode,
  printTree,
  printTreeStack,
  printTreeConstant,
  randomTree
};

import { randomAB } from "../util";

class Tree<T> {
  root: TreeNode<T> | null;

  constructor() {
    this.root = null;
  }
}

class TreeNode<T> {
  key: T;
  parent: TreeNode<T> | null;
  left: TreeNode<T> | null;
  right: TreeNode<T> | null;

  constructor(k: T) {
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

function randomGrow<T>(a: T, node: TreeNode<T>) {
  while (true) {
    if (Math.random() >= 0.5) {
      if (node.left !== null) {
        node = node.left;
      } else {
        node.left = new TreeNode(a);
        node.left.parent = node;
        return;
      }
    } else {
      if (node.right !== null) {
        node = node.right;
      } else {
        node.right = new TreeNode(a);
        node.right.parent = node;
        return;
      }
    }
  }
}

function randomTree(n: number): TreeNode<number> {
  let node = new TreeNode(randomAB(1, 100));
  for (let i = 0; i < n - 1; i++) {
    randomGrow(randomAB(1, 100), node);
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