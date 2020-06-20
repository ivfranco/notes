/// height balanced binary search tree

export { AVLTree, AVLNode, join, split };

import {
  Leaf,
  Internal,
  Tree,
  Factory,
  left_rotation,
  right_rotation,
  connect_left,
  connect_right,
  narrow_to_leaf,
  split_leaf,
  join_leaf,
} from "./lib";
import { Comparator, Ordering, native_comparator } from "./comparator";

type AVLNode<K, V> = AVLLeaf<K, V> | AVLInternal<K, V>;

class AVLLeaf<K, V> implements Leaf<K, V> {
  kind: "Leaf" = "Leaf";
  key: K;
  value: V;
  parent: AVLInternal<K, V> | null = null;

  constructor(key: K, value: V) {
    this.key = key;
    this.value = value;
  }

  get_height(): number {
    return 0;
  }

  recalc_height(): number {
    return this.get_height();
  }

  fix_height() {}
}

class AVLInternal<K, V> implements Internal<K, V> {
  kind: "Internal" = "Internal";
  key: K;
  height!: number;
  parent: AVLInternal<K, V> | null = null;
  left_child!: AVLNode<K, V>;
  right_child!: AVLNode<K, V>;

  constructor(key: K, left_child: AVLNode<K, V>, right_child: AVLNode<K, V>) {
    this.key = key;
    connect_left(this, left_child);
    connect_right(this, right_child);
    this.fix_height();
  }

  get_height(): number {
    return this.height;
  }

  recalc_height(): number {
    return Math.max(this.left_child.get_height(), this.right_child.get_height()) + 1;
  }

  fix_height() {
    this.height = this.recalc_height();
  }
}

// bottom up rebalance of heights
function rebalance<K, V>(node: AVLInternal<K, V> | null) {
  while (node) {
    if (node.left_child.get_height() == node.right_child.get_height() + 2) {
      // size of left child is at least 2, it must be an internal node
      let left_child = <AVLInternal<K, V>>node.left_child;
      let right_child = node.right_child;
      if (left_child.left_child.get_height() == right_child.get_height() + 1) {
        // case 2.1
        right_rotation(node);
        // always fix height of children then parent
        node.right_child.fix_height();
        node.fix_height();
      } else {
        // case 2.2
        console.assert(left_child.left_child.get_height() == right_child.get_height());
        left_rotation(node.left_child);
        right_rotation(node);
        node.left_child.fix_height();
        node.right_child.fix_height();
        node.fix_height();
      }
    } else if (node.left_child.get_height() + 2 == node.right_child.get_height()) {
      // symmetric
      let left_child = node.left_child;
      let right_child = <AVLInternal<K, V>>node.right_child;
      if (right_child.right_child.get_height() == left_child.get_height() + 1) {
        // case 2.3
        left_rotation(node);
        node.left_child.fix_height();
        node.fix_height();
      } else {
        // case 2.4
        console.assert(right_child.right_child.get_height() == left_child.get_height());
        right_rotation(node.right_child);
        left_rotation(node);
        node.left_child.fix_height();
        node.right_child.fix_height();
        node.fix_height();
      }
    } else {
      // case 1, subtree is already in balance
      let new_height = node.recalc_height();
      if (node.height == new_height) {
        return;
      } else {
        node.height = new_height;
      }
    }

    node = node.parent;
  }
}

class AVLTree<K, V> extends Tree<K, V, AVLNode<K, V>> {
  cmp: Comparator<K>;
  root: AVLNode<K, V> | null = null;

  constructor(cmp: Comparator<K>) {
    super();
    this.cmp = cmp;
  }

  is_empty(): boolean {
    return this.root == null;
  }

  insert(key: K, value: V) {
    let new_leaf = new AVLLeaf(key, value);
    if (this.root == null) {
      this.root = new_leaf;
      return;
    }

    let old_leaf = <AVLLeaf<K, V>>narrow_to_leaf(key, this.root, this.cmp);
    if (this.cmp(old_leaf.key, new_leaf.key) == Ordering.EQ) {
      old_leaf.value = value;
      return;
    }

    let internal = split_leaf(old_leaf, new_leaf, this.cmp, new AVLFactory<K, V>());
    if (this.root == old_leaf) {
      this.root = internal;
    }

    rebalance(internal.parent);
  }

  delete(key: K): V | null {
    if (this.root == null) {
      return null;
    }

    let leaf = <AVLLeaf<K, V>>narrow_to_leaf(key, this.root, this.cmp);
    if (this.cmp(leaf.key, key) != Ordering.EQ) {
      return null;
    }

    if (this.root == leaf) {
      this.root = null;
      return null;
    }

    let internal = <AVLNode<K, V>>join_leaf(leaf);
    rebalance(internal.parent);

    return leaf.value;
  }
}

class AVLFactory<K, V> implements Factory<K, V, AVLLeaf<K, V>, AVLInternal<K, V>> {
  create_leaf(key: K, value: V): AVLLeaf<K, V> {
    return new AVLLeaf(key, value);
  }

  create_internal(key: K, left_child: AVLNode<K, V>, right_child: AVLNode<K, V>): AVLInternal<K, V> {
    return new AVLInternal(key, left_child, right_child);
  }
}

function join<K, V>(key: K, left_tree: AVLTree<K, V>, right_tree: AVLTree<K, V>): AVLTree<K, V> {
  // `key` >= all keys in `left_tree`, <= all keys in `right_tree`
  if (left_tree.root == null) {
    return right_tree;
  }

  if (right_tree.root == null) {
    return left_tree;
  }

  let left_root = left_tree.root;
  let right_root = right_tree.root;

  let root = join_nodes(key, left_root, right_root);
  let tree = new AVLTree<K, V>(left_tree.cmp);
  tree.root = root;

  return tree;
}

function join_nodes<K, V>(key: K, left_node: AVLNode<K, V>, right_node: AVLNode<K, V>): AVLNode<K, V> {
  if (Math.abs(left_node.get_height() - right_node.get_height()) <= 1) {
    return new AVLInternal(key, left_node, right_node);
  } else if (left_node.get_height() < right_node.get_height()) {
    // right_node.get_height() >= left_node.get_height() + 2
    let node = right_node;
    let h = left_node.get_height();
    while (node.kind == "Internal" && node.get_height() > h) {
      node = node.left_child;
    }

    // node cannot be root, root has height >= h + 2
    let parent = node.parent!;
    // the three cases can be handled similarly given the current implementation of rebalancing and node constructor
    let internal = new AVLInternal(key, left_node, node);
    connect_left(parent, internal);
    rebalance(parent);

    return right_node;
  } else {
    // left_node.get_height() >= right_node.get_height() + 2
    let node = left_node;
    let h = right_node.get_height();
    while (node.kind == "Internal" && node.get_height() > h) {
      node = node.right_child;
    }

    let parent = node.parent!;
    let internal = new AVLInternal(key, node, right_node);
    connect_right(parent, internal);
    rebalance(parent);

    return left_node;
  }
}

// split a tree into two subtrees, first contains all keys < split_key, second contains all keys >= split_key
function split<K, V>(split_key: K, tree: AVLTree<K, V>): [AVLTree<K, V>, AVLTree<K, V>] {
  let left_tree = new AVLTree<K, V>(tree.cmp);
  let right_tree = new AVLTree<K, V>(tree.cmp);

  if (tree.root) {
    let left_list: Array<[K, AVLNode<K, V>]> = [];
    let right_list: Array<[K, AVLNode<K, V>]> = [];
    let node = tree.root;
    let cmp = tree.cmp;

    while (node.kind == "Internal") {
      // 2 hours of debugging
      // otherwise rebalance in join operations will change the structure of the whole tree
      node.right_child.parent = null;
      node.left_child.parent = null;
      if (cmp(split_key, node.key) == Ordering.LT) {
        right_list.push([node.key, node.right_child]);
        node = node.left_child;
      } else {
        left_list.push([node.key, node.left_child]);
        node = node.right_child;
      }
    }

    if (cmp(split_key, node.key) != Ordering.GT) {
      // the key here is not used
      right_list.push([node.key, node]);
    } else {
      left_list.push([node.key, node]);
    }

    if (left_list.length > 0) {
      let [_, root] = left_list.reduceRight(([_, prev_node], [next_key, next_node]) => [
        next_key,
        // `next_node` is in left subtree of `prev_node.parent`
        join_nodes(next_key, next_node, prev_node),
      ]);
      left_tree.root = root;
    }

    if (right_list.length > 0) {
      let [_, root] = right_list.reduceRight(([_, prev_node], [next_key, next_node]) => [
        next_key,
        // `next_node` is in right subtree of `prev_node.parent`
        join_nodes(next_key, prev_node, next_node),
      ]);
      right_tree.root = root;
    }
  }

  return [left_tree, right_tree];
}
