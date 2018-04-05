export {
  RadixTree
};

class RadixTree {
  root: RadixTreeNode;

  constructor() {
    this.root = new RadixTreeNode(false);
  }

  insert(str: string) {
    insert(str, this.root);
  }

  *preorder(): IterableIterator<string> {
    yield* this.root.preorder([]);
  }
}

class RadixTreeNode {
  left: RadixTreeNode | null;
  right: RadixTreeNode | null;
  is_end: boolean;

  constructor(is_end: boolean) {
    this.left = null;
    this.right = null;
    this.is_end = is_end;
  }

  *preorder(prefix: string[]): IterableIterator<string> {
    if (this.is_end) {
      yield prefix.join("");
    }
    if (this.left !== null) {
      prefix.push("0");
      yield* this.left.preorder(prefix);
      prefix.pop();
    }
    if (this.right !== null) {
      prefix.push("1");
      yield* this.right.preorder(prefix);
      prefix.pop();
    }
  }
}

function insert(str: string, node: RadixTreeNode) {
  for (let c of str) {
    if (c === "0") {
      if (node.left === null) {
        node.left = new RadixTreeNode(false);
      }
      node = node.left;
    } else {
      if (node.right === null) {
        node.right = new RadixTreeNode(false);
      }
      node = node.right;
    }
  }
  node.is_end = true;
}