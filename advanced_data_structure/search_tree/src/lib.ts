export {
  BNode,
  Internal,
  Leaf,
  Factory,
  TreeLike,
  Tree,
  Interval,
  left_rotation,
  right_rotation,
  connect_left,
  connect_right,
  connect_by_key,
  find,
  find_interval,
  narrow_to_leaf,
  split_leaf,
  join_leaf,
  make_tree_bottom_up as make_tree,
  make_tree_top_down,
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

  open_open(key: K, cmp: Comparator<K>): boolean {
    return (
      (this.min == null || cmp(this.min, key) == Ordering.LT) && (this.max == null || cmp(key, this.max) == Ordering.LT)
    );
  }
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

function internal_by_key<K, V, L extends Leaf<K, V>, I extends Internal<K, V>>(
  first_child: L | I,
  second_child: L | I,
  cmp: Comparator<K>,
  factory: Factory<K, V, L, I>
): I {
  if (cmp(first_child.key, second_child.key) == Ordering.LT) {
    return factory.create_internal(second_child.key, first_child, second_child);
  } else if (cmp(second_child.key, first_child.key) == Ordering.LT) {
    return factory.create_internal(first_child.key, second_child, first_child);
  } else {
    throw new Error("Duplicated key");
  }
}

function split_leaf<K, V, L extends Leaf<K, V>, I extends Internal<K, V>>(
  old_leaf: L,
  new_leaf: L,
  cmp: Comparator<K>,
  factory: Factory<K, V, L, I>
): I {
  let parent = old_leaf.parent;
  let internal = internal_by_key(old_leaf, new_leaf, cmp, factory);

  if (parent) {
    if (parent.left_child == old_leaf) {
      connect_left(parent, internal);
    } else {
      connect_right(parent, internal);
    }
  }

  return internal;
}

function join_leaf<K, V>(leaf: Leaf<K, V>): BNode<K, V> {
  console.assert(leaf.parent != null);

  let parent = <Internal<K, V>>leaf.parent;

  let other_child: BNode<K, V>;
  if (parent.left_child == leaf) {
    other_child = parent.right_child;
  } else {
    other_child = parent.left_child;
  }

  let grand_parent = parent.parent;
  if (grand_parent) {
    if (grand_parent.left_child == parent) {
      connect_left(grand_parent, other_child);
    } else {
      connect_right(grand_parent, other_child);
    }
  }

  return other_child;
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

// bottom up construction of optimal tree
// expect (key, value) pair sorted by keys
function make_tree_bottom_up<K, V, L extends Leaf<K, V>, I extends Internal<K, V>>(
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

// the top down version doesn't fit well into the type system
// key and value of a leaf may be placeholders instead of the expected type during construction
// expect (key, value) pairs sorted by keys
function make_tree_top_down<K, V, L extends Leaf<K | null, V | null>, I extends Internal<K | null, V | null>>(
  pairs: [K, V][],
  factory: Factory<K | null, V | null, L, I>
): L | I | null {
  interface StackObject {
    node: L;
    target: I | null;
    leaves: number;
  }

  function split_leaf(leaf: L): I {
    let left_child = factory.create_leaf(null, null);
    let right_child = factory.create_leaf(null, null);
    let internal = factory.create_internal(null, left_child, right_child);

    // fix parent pointers
    let parent = leaf.parent;
    if (parent) {
      if (parent.left_child == leaf) {
        connect_left(parent, internal);
      } else {
        connect_right(parent, internal);
      }
    }

    return internal;
  }

  if (pairs.length == 0) {
    return null;
  }

  pairs = pairs.reverse();
  // need access to one leaf to traverse back to the root
  let leaf = factory.create_leaf(null, null);
  let stack: StackObject[] = [
    {
      node: leaf,
      target: null,
      leaves: pairs.length,
    },
  ];

  while (stack.length > 0) {
    let { node, target, leaves } = <StackObject>stack.pop();
    if (leaves == 1) {
      // left child is expended first, pairs can be inserted in increasing order
      let [key, value] = <[K, V]>pairs.pop();
      node.key = key;
      node.value = value;
      if (target) {
        target.key = key;
      }
      leaf = node;
    } else {
      // key of an internal node should be the key of the left-most leaf of its right subtree
      let internal = split_leaf(node);
      let left_object = {
        node: <L>internal.left_child,
        // `internal` is in the right subtree of `target`
        // its left subtree inherits its target
        target: target,
        // WARNING: javascript has no integer type
        leaves: Math.floor(leaves / 2),
      };
      let right_object = {
        node: <L>internal.right_child,
        // the key of `internal` comes from its right tree
        target: internal,
        leaves: leaves - left_object.leaves,
      };

      // order here is crucial, left-most child must be on top of the stack
      stack.push(right_object);
      stack.push(left_object);
    }
  }

  let root: I | L = leaf;
  while (root.parent) {
    root = <I | L>root.parent;
  }

  return root;
}

interface TreeLike<K, V> {
  insert(key: K, value: V): void;
  delete(key: K): V | null;
  find(key: K): V | null;
}

abstract class Tree<K, V, N extends BNode<K, V>> implements TreeLike<K, V> {
  abstract root: N | null;
  abstract cmp: Comparator<K>;

  abstract insert(key: K, value: V): void;
  abstract delete(key: K): V | null;

  find(search_key: K): V | null {
    if (this.root == null) {
      return null;
    } else {
      return find(search_key, this.root, this.cmp);
    }
  }

  find_interval(min: K, max: K): [K, V][] {
    if (this.root == null) {
      return [];
    } else {
      return find_interval(min, max, this.root, this.cmp);
    }
  }

  to_dot(render: (node: N) => string): string {
    if (this.root) {
      return to_dot(this.root, render);
    } else {
      return "Empty tree";
    }
  }
}

function to_dot<K, V, N extends BNode<K, V>>(root: N, render: (node: N) => string): string {
  let last_index = 0;
  let stack: [N, number][] = [[root, last_index]];
  let dot = "";

  while (stack.length > 0) {
    let [node, index] = <[N, number]>stack.pop();
    dot += `node${index} [label = "${render(node)}"]\n`;

    if (node.kind == "Internal") {
      let left_child = (<Internal<K, V>>node).left_child;
      let right_child = (<Internal<K, V>>node).right_child;

      dot += `node${index} -> node${last_index + 1}\n`;
      dot += `node${index} -> node${last_index + 2}\n`;

      stack.push([<N>left_child, last_index + 1]);
      stack.push([<N>right_child, last_index + 2]);
      last_index += 2;
    }
  }

  return `Digraph G {
    ${dot}
  }`;
}
