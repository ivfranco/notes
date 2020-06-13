/// red-black tree with top-down rebalancing
export { RBNode, Color, RBTree };

import {
  Leaf,
  Internal,
  Tree,
  Factory,
  left_rotation,
  right_rotation,
  connect_left,
  connect_right,
  split_leaf,
  join_leaf,
} from "./lib";
import { Comparator, Ordering } from "./comparator";

enum Color {
  RED,
  BLACK,
}

type RBNode<K, V> = RBLeaf<K, V> | RBInternal<K, V>;

class RBLeaf<K, V> implements Leaf<K, V> {
  kind: "Leaf" = "Leaf";
  key: K;
  value: V;
  color: Color;
  parent: RBInternal<K, V> | null = null;

  constructor(key: K, value: V, color: Color) {
    this.key = key;
    this.value = value;
    this.color = color;
  }
}

class RBInternal<K, V> implements Internal<K, V> {
  kind: "Internal" = "Internal";
  key: K;
  color: Color;
  parent: RBInternal<K, V> | null = null;
  left_child!: RBNode<K, V>;
  right_child!: RBNode<K, V>;

  constructor(key: K, left_child: RBNode<K, V>, right_child: RBNode<K, V>, color: Color) {
    this.key = key;
    connect_left(this, left_child);
    connect_right(this, right_child);
    this.color = color;
  }
}

class RBTree<K, V> extends Tree<K, V, RBNode<K, V>> {
  cmp: Comparator<K>;
  root: RBNode<K, V> | null = null;

  constructor(cmp: Comparator<K>) {
    super();
    this.cmp = cmp;
  }

  insert(key: K, value: V) {
    let new_leaf = new RBLeaf(key, value, Color.RED);

    // tree is empty
    if (this.root == null) {
      this.root = new_leaf;
      return;
    }

    // `upper` is the black node immediately above `current`
    // `upper` is either the parent or the grandparent of `current`
    let upper: RBInternal<K, V> | null = null;
    let current: RBNode<K, V> = this.root;

    while (current.kind == "Internal") {
      let next: RBNode<K, V>;
      if (this.cmp(key, current.key) == Ordering.LT) {
        next = current.left_child;
      } else {
        next = current.right_child;
      }

      if (current.color == Color.BLACK) {
        if (current.left_child.color == Color.BLACK || current.right_child.color == Color.BLACK) {
          // case 1.1, no rebalancing
        } else {
          if (upper == null) {
            current.left_child.color = Color.BLACK;
            current.right_child.color = Color.BLACK;
          } else if (this.cmp(current.key, upper.key) == Ordering.LT) {
            // `current` is in the left subtree of `upper`
            if (current == upper.left_child) {
              // case 2.1
              current.left_child.color = Color.BLACK;
              current.right_child.color = Color.BLACK;
              current.color = Color.RED;
            } else if (current == (<RBInternal<K, V>>upper.left_child).left_child) {
              // case 2.2
              right_rotation(upper);
              upper.left_child.color = Color.RED;
              upper.right_child.color = Color.RED;
              let left_child = <RBInternal<K, V>>upper.left_child;
              left_child.left_child.color = Color.BLACK;
              left_child.right_child.color = Color.BLACK;
            } else {
              // case 2.3, current == upper.left_child.right_child
              left_rotation(upper.left_child);
              right_rotation(upper);
              upper.left_child.color = Color.RED;
              upper.right_child.color = Color.RED;
              (<RBInternal<K, V>>upper.left_child).right_child.color = Color.BLACK;
              (<RBInternal<K, V>>upper.right_child).left_child.color = Color.BLACK;
            }
          } else {
            // symmetric
            if (current == upper.right_child) {
              // case 3.1
              current.left_child.color = Color.BLACK;
              current.right_child.color = Color.BLACK;
              current.color = Color.RED;
            } else if (current == (<RBInternal<K, V>>upper.right_child).right_child) {
              // case 3.2
              left_rotation(upper);
              upper.left_child.color = Color.RED;
              upper.right_child.color = Color.RED;
              let right_child = <RBInternal<K, V>>upper.right_child;
              right_child.left_child.color = Color.BLACK;
              right_child.right_child.color = Color.BLACK;
            } else {
              // case 3.3, current == upper.right_child.left_child;
              right_rotation(upper.right_child);
              left_rotation(upper);
              upper.left_child.color = Color.RED;
              upper.right_child.color = Color.RED;
              (<RBInternal<K, V>>upper.left_child).right_child.color = Color.BLACK;
              (<RBInternal<K, V>>upper.right_child).left_child.color = Color.BLACK;
            }
          }
        }

        upper = current;
      }

      current = next;
    }

    // current is now a black leaf, it's possible to split it into a black internal node with two red leaves
    let internal = split_leaf(current, new_leaf, this.cmp, new RBFactory(Color.BLACK));
    internal.left_child.color = Color.RED;
    internal.right_child.color = Color.RED;

    if (current == this.root) {
      this.root = internal;
    }
  }

  delete(key: K): V | null {
    // the description of this part is well beyond my grasp
    throw new Error("Not implemented");
  }
}

class RBFactory<K, V> implements Factory<K, V, RBLeaf<K, V>, RBInternal<K, V>> {
  color: Color;

  constructor(color: Color) {
    this.color = color;
  }

  create_leaf(key: K, value: V): RBLeaf<K, V> {
    return new RBLeaf(key, value, this.color);
  }

  create_internal(key: K, left_child: RBNode<K, V>, right_child: RBNode<K, V>): RBInternal<K, V> {
    return new RBInternal(key, left_child, right_child, this.color);
  }
}
