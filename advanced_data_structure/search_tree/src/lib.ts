export {
  BNode,
  Internal,
  Leaf,
  Factory,
  left_rotation,
  right_rotation,
  is_ordered,
  connect_by_key,
  find,
  narrow_to_leaf,
  find_interval,
  make_tree,
  depth,
  size,
};

import { Comparator, Ordering } from "./comparator";

type BNode<K, V> = Internal<K, V> | Leaf<K, V>;

interface Internal<K, V> {
  kind: "Internal";
  parent: Internal<K, V> | null;
  key: K;
  left_child: BNode<K, V>;
  right_child: BNode<K, V>;
}

interface Leaf<K, V> {
  kind: "Leaf";
  parent: Internal<K, V> | null;
  key: K;
  value: V;
}

function depth<K, V>(node: BNode<K, V>): number {
  if (node.kind == "Leaf") {
    return 0;
  } else {
    return Math.max(depth(node.left_child), depth(node.right_child));
  }
}

function size<K, V>(node: BNode<K, V>): number {
  if (node.kind == "Leaf") {
    return 1;
  } else {
    return size(node.left_child) + size(node.right_child);
  }
}

class Interval<K> {
  min: K | null;
  max: K | null;

  constructor(min: K | null, max: K | null) {
    this.min = min;
    this.max = max;
  }

  close_open(key: K, cmp: Comparator<K>): boolean {
    return (
      (this.min == null || cmp(this.min, key) != Ordering.GT) && (this.max == null || cmp(key, this.max) == Ordering.LT)
    );
  }

  close_close(key: K, cmp: Comparator<K>): boolean {
    return (
      (this.min == null || cmp(this.min, key) != Ordering.GT) && (this.max == null || cmp(key, this.max) != Ordering.GT)
    );
  }
}

function is_ordered<K, V>(node: BNode<K, V>, cmp: Comparator<K>): boolean {
  let stack: [BNode<K, V>, Interval<K>][] = [[node, new Interval<K>(null, null)]];

  while (stack.length != 0) {
    let [node, interval] = <[BNode<K, V>, Interval<K>]>stack.pop();
    if (!interval.close_open(node.key, cmp)) {
      return false;
    }

    if (node.kind == "Internal") {
      stack.push([node.left_child, new Interval(interval.min, node.key)]);
      stack.push([node.right_child, new Interval(node.key, interval.max)]);
    }
  }

  return true;
}

function connect_left<K, V>(parent: Internal<K, V>, child: BNode<K, V>) {
  parent.left_child = child;
  child.parent = parent;
}

function connect_right<K, V>(parent: Internal<K, V>, child: BNode<K, V>) {
  parent.right_child = child;
  child.parent = parent;
}

function connect_by_key<K, V>(parent: Internal<K, V>, child: BNode<K, V>, cmp: Comparator<K>) {
  if (cmp(child.key, parent.key) == Ordering.LT) {
    connect_left(parent, child);
  } else {
    connect_right(parent, child);
  }
}

function left_rotation<K, V>(node: BNode<K, V>) {
  if (node.kind == "Leaf") {
    return;
  }

  let right_child = node.right_child;
  if (right_child.kind == "Leaf") {
    return;
  }

  connect_right(node, right_child.right_child);
  connect_right(right_child, right_child.left_child);
  connect_left(right_child, node.left_child);
  connect_left(node, right_child);

  let temp_key = node.key;
  node.key = right_child.key;
  right_child.key = temp_key;
}

function right_rotation<K, V>(node: BNode<K, V>) {
  if (node.kind == "Leaf") {
    return;
  }

  let left_child = node.left_child;
  if (left_child.kind == "Leaf") {
    return;
  }

  connect_left(node, left_child.left_child);
  connect_left(left_child, left_child.right_child);
  connect_right(left_child, node.right_child);
  connect_right(node, left_child);

  let temp_key = node.key;
  node.key = left_child.key;
  left_child.key = temp_key;
}

function narrow_to_leaf<K, V>(search_key: K, node: BNode<K, V>, cmp: Comparator<K>): Leaf<K, V> {
  while (node.kind == "Internal") {
    if (cmp(search_key, node.key) == Ordering.LT) {
      node = node.left_child;
    } else {
      node = node.right_child;
    }
  }

  return node;
}

function find<K, V>(search_key: K, node: BNode<K, V>, cmp: Comparator<K>): V | null {
  let leaf = narrow_to_leaf(search_key, node, cmp);
  if (cmp(leaf.key, search_key) == Ordering.EQ) {
    return leaf.value;
  } else {
    return null;
  }
}

function find_interval<K, V>(min: K, max: K, node: BNode<K, V>, cmp: Comparator<K>): [K, V][] {
  let hits: [K, V][] = [];
  let stack: BNode<K, V>[] = [node];
  let interval = new Interval(min, max);

  while (stack.length != 0) {
    let node = <BNode<K, V>>stack.pop();
    if (node.kind == "Leaf") {
      if (interval.close_close(node.key, cmp)) {
        hits.push([node.key, node.value]);
      }
    } else {
      if (cmp(max, node.key) == Ordering.LT) {
        stack.push(node.left_child);
      } else if (cmp(node.key, min) != Ordering.GT) {
        stack.push(node.right_child);
      } else {
        stack.push(node.left_child);
        stack.push(node.right_child);
      }
    }
  }

  return hits;
}

interface Factory<K, V, L extends Leaf<K, V>, I extends Internal<K, V>> {
  create_leaf(key: K, value: V): L;
  create_internal(key: K, left_child: L | I, right_child: L | I): I;
}

function make_tree<K, V, L extends Leaf<K, V>, I extends Internal<K, V>>(
  pairs: [K, V][],
  factory: Factory<K, V, L, I>
): L | I | null {
  if (pairs.length == 0) {
    return null;
  }

  // nodes with the smallest key under them
  let nodes: [I | L, K][] = pairs.map(([k, v]) => [factory.create_leaf(k, v), k]);
  while (nodes.length > 1) {
    let new_nodes: [I | L, K][] = [];
    for (let i = 0; i < nodes.length; i += 2) {
      if (i + 1 >= nodes.length) {
        new_nodes.push(nodes[i]);
      } else {
        let [left_child, left_min] = nodes[i];
        let [right_child, right_min] = nodes[i + 1];
        new_nodes.push([factory.create_internal(right_min, left_child, right_child), left_min]);
      }
    }
    nodes = new_nodes;
  }

  return nodes[0][0];
}
