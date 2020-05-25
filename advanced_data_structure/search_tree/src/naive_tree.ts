export { NaiveTree, NaiveFactory };
import {
  Internal,
  Leaf,
  Factory,
  Tree,
  connect_by_key,
  find,
  narrow_to_leaf,
  find_interval,
  connect_left,
  connect_right,
  right_rotation,
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

function internal_by_key<K, V>(
  first_child: NaiveNode<K, V>,
  second_child: NaiveNode<K, V>,
  cmp: Comparator<K>
): NaiveInternal<K, V> {
  if (cmp(first_child.key, second_child.key) == Ordering.LT) {
    return new NaiveInternal(second_child.key, first_child, second_child);
  } else if (cmp(second_child.key, first_child.key) == Ordering.LT) {
    return new NaiveInternal(first_child.key, second_child, first_child);
  } else {
    throw new Error("Duplicated key");
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

class NaiveTree<K, V> extends Tree<K, V> {
  cmp: Comparator<K>;
  root: NaiveNode<K, V> | null = null;

  constructor(cmp: Comparator<K>) {
    super();
    this.cmp = cmp;
  }

  find_interval(min: K, max: K): [K, V][] {
    if (this.root == null) {
      return [];
    } else {
      return find_interval(min, max, this.root, this.cmp);
    }
  }

  insert(key: K, value: V) {
    let new_leaf = new NaiveLeaf(key, value);

    if (this.root == null) {
      this.root = new_leaf;
      return;
    }

    let old_leaf = <NaiveLeaf<K, V>>narrow_to_leaf(key, this.root, this.cmp);
    let parent = old_leaf.parent;
    let internal = internal_by_key(new_leaf, old_leaf, this.cmp);
    if (parent == null) {
      this.root = internal;
    } else {
      connect_by_key(parent, internal, this.cmp);
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

    let parent = leaf.parent;
    // tree is a singleton
    if (parent == null) {
      this.root = null;
      return leaf.value;
    }

    let other_child: NaiveNode<K, V>;
    if (this.cmp(key, parent.key) == Ordering.LT) {
      other_child = parent.right_child;
    } else {
      other_child = parent.left_child;
    }

    let grand_parent = parent.parent;
    if (grand_parent == null) {
      other_child.parent = null;
      this.root = other_child;
    } else {
      connect_by_key(grand_parent, other_child, this.cmp);
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
