/// weight balanced binary search tree

export { BBNode, BBTree, ALPHA };

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

// anything in [2/7, 1 - 1/√2]
const ALPHA: number = 0.288;
// anything <= α^2 - 2α + 1/2, bigger value decreases the necessary number of rebalancing operations
const EPSILON: number = 0.005;

type BBNode<K, V> = BBLeaf<K, V> | BBInternal<K, V>;

class BBLeaf<K, V> implements Leaf<K, V> {
  kind: "Leaf" = "Leaf";
  key: K;
  value: V;
  parent: BBInternal<K, V> | null = null;

  constructor(key: K, value: V) {
    this.key = key;
    this.value = value;
  }

  get_weight() {
    return 1;
  }

  recalc_weight() {
    return this.get_weight();
  }

  fix_weight() {}
}

class BBInternal<K, V> implements Internal<K, V> {
  kind: "Internal" = "Internal";
  key: K;
  weight!: number;
  parent: BBInternal<K, V> | null = null;
  left_child!: BBNode<K, V>;
  right_child!: BBNode<K, V>;

  constructor(key: K, left_child: BBNode<K, V>, right_child: BBNode<K, V>) {
    this.key = key;
    connect_left(this, left_child);
    connect_right(this, right_child);
    this.fix_weight();
  }

  get_weight() {
    return this.weight;
  }

  recalc_weight() {
    return this.left_child.get_weight() + this.right_child.get_weight();
  }

  fix_weight() {
    this.weight = this.recalc_weight();
  }
}

function rebalance<K, V>(node: BBInternal<K, V> | null) {
  while (node) {
    // unlike heights, rotation will not change the weight of the parent node
    // the weight of the parent node can be fixed first
    node.fix_weight();

    if (node.right_child.get_weight() < ALPHA * node.get_weight()) {
      // right child has weight at least 1
      // αw > 1, by our choice of α = 0.288, left_child.weight > (1 - α)w > αw > 1, left child must be internal node
      let left_child = <BBInternal<K, V>>node.left_child;
      if (left_child.left_child.get_weight() > (ALPHA + EPSILON) * node.get_weight()) {
        // case 2.1
        right_rotation(node);
        node.right_child.fix_weight();
      } else {
        // case 2.2
        left_rotation(node.left_child);
        right_rotation(node);
        node.left_child.fix_weight();
        node.right_child.fix_weight();
        // rotation leaves attributes of `node` other than children and parents untouched
        // weight of `node` is still correct after rotations
      }
    } else if (node.left_child.get_weight() < ALPHA * node.get_weight()) {
      // symmetric
      let right_child = <BBInternal<K, V>>node.right_child;
      if (right_child.right_child.get_weight() > (ALPHA + EPSILON) * node.get_weight()) {
        // case 3.1
        left_rotation(node);
        node.left_child.fix_weight();
      } else {
        // case 3.2
        right_rotation(node.right_child);
        left_rotation(node);
        node.left_child.fix_weight();
        node.right_child.fix_weight();
      }
    }

    node = node.parent;
  }
}

class BBTree<K, V> extends Tree<K, V, BBNode<K, V>> {
  cmp: Comparator<K>;
  root: BBNode<K, V> | null = null;

  constructor(cmp: Comparator<K>) {
    super();
    this.cmp = cmp;
  }

  insert(key: K, value: V) {
    let new_leaf = new BBLeaf(key, value);
    if (this.root == null) {
      this.root = new_leaf;
      return;
    }

    let old_leaf = <BBLeaf<K, V>>narrow_to_leaf(key, this.root, this.cmp);
    if (this.cmp(old_leaf.key, new_leaf.key) == Ordering.EQ) {
      old_leaf.value = value;
      return;
    }

    let internal = split_leaf(old_leaf, new_leaf, this.cmp, new BBFactory<K, V>());
    if (this.root == old_leaf) {
      this.root = internal;
    }

    // `internal` is necessarily balanced, its left and right children are leaves
    // the first possibly imbalance node is its parent
    rebalance(internal.parent);
  }

  delete(key: K): V | null {
    if (this.root == null) {
      return null;
    }

    let leaf = <BBLeaf<K, V>>narrow_to_leaf(key, this.root, this.cmp);
    if (this.cmp(leaf.key, key) != Ordering.EQ) {
      return null;
    }

    if (this.root == leaf) {
      this.root = null;
      return null;
    }

    let internal = <BBNode<K, V>>join_leaf(leaf);
    rebalance(internal.parent);

    return leaf.value;
  }
}

class BBFactory<K, V> implements Factory<K, V, BBLeaf<K, V>, BBInternal<K, V>> {
  create_leaf(key: K, value: V): BBLeaf<K, V> {
    return new BBLeaf(key, value);
  }

  create_internal(key: K, left_child: BBNode<K, V>, right_child: BBNode<K, V>): BBInternal<K, V> {
    return new BBInternal(key, left_child, right_child);
  }
}
