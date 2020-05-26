export { NaiveTree, NaiveFactory };
import {
  Internal,
  Leaf,
  Factory,
  Tree,
  narrow_to_leaf,
  connect_left,
  connect_right,
  split_leaf,
  join_leaf,
} from "./lib";
import { Comparator, Ordering } from "./comparator";

type NaiveNode<K, V> = NaiveInternal<K, V> | NaiveLeaf<K, V>;

class NaiveInternal<K, V> implements Internal<K, V> {
  kind: "Internal" = "Internal";
  parent: NaiveInternal<K, V> | null = null;
  key: K;
  left_child!: NaiveNode<K, V>;
  right_child!: NaiveNode<K, V>;

  constructor(key: K, left_child: NaiveNode<K, V>, right_child: NaiveNode<K, V>) {
    this.key = key;
    connect_left(this, left_child);
    connect_right(this, right_child);
  }
}

class NaiveLeaf<K, V> implements Leaf<K, V> {
  kind: "Leaf" = "Leaf";
  parent: NaiveInternal<K, V> | null = null;
  key: K;
  value: V;

  constructor(key: K, value: V) {
    this.key = key;
    this.value = value;
  }
}

class NaiveTree<K, V> extends Tree<K, V, NaiveNode<K, V>> {
  cmp: Comparator<K>;
  root: NaiveNode<K, V> | null = null;

  constructor(cmp: Comparator<K>) {
    super();
    this.cmp = cmp;
  }

  insert(key: K, value: V) {
    let new_leaf = new NaiveLeaf(key, value);

    if (this.root == null) {
      this.root = new_leaf;
      return;
    }

    let old_leaf = <NaiveLeaf<K, V>>narrow_to_leaf(key, this.root, this.cmp);
    let internal = split_leaf(old_leaf, new_leaf, this.cmp, new NaiveFactory<K, V>());

    if (this.root == old_leaf) {
      this.root = internal;
    }
  }

  delete(key: K): V | null {
    // tree is empty
    if (this.root == null) {
      return null;
    }

    let leaf = <NaiveLeaf<K, V>>narrow_to_leaf(key, this.root, this.cmp);
    // tree has no corresponding value
    if (this.cmp(leaf.key, key) != Ordering.EQ) {
      return null;
    }

    // tree is a singleton
    if (leaf.parent == null) {
      this.root = null;
      return leaf.value;
    }

    let other_child = join_leaf(leaf);
    if (other_child.parent == null) {
      this.root = other_child;
    }

    return leaf.value;
  }
}

class NaiveFactory<K, V> implements Factory<K, V, NaiveLeaf<K, V>, NaiveInternal<K, V>> {
  create_leaf(key: K, value: V): NaiveLeaf<K, V> {
    return new NaiveLeaf(key, value);
  }

  create_internal(key: K, left_child: NaiveNode<K, V>, right_child: NaiveNode<K, V>): NaiveInternal<K, V> {
    return new NaiveInternal(key, left_child, right_child);
  }
}
