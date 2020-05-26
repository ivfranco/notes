export { AVLTree, AVLNode };

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
import { Comparator, Ordering } from "./comparator";

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
