export { grow_random_tree, is_ordered, is_connected };

import { Tree, BNode, Interval } from "../src/lib";
import { Comparator } from "../src/comparator";

// generates a random integer in the range [min, cap - 1]
function random_int(min: number, cap: number): number {
  return Math.floor(Math.random() * (cap - min)) + min;
}

function grow_random_tree<N extends BNode<number, number>>(min: number, max: number, tree: Tree<number, number, N>) {
  let inputs = [];
  for (let i = min; i <= max; i++) {
    inputs.push(i);
  }
  shuffle(inputs);

  for (let i of inputs) {
    tree.insert(i, i);
  }
}

function swap<T>(array: T[], i: number, j: number) {
  let temp = array[i];
  array[i] = array[j];
  array[j] = temp;
}

function shuffle<T>(array: T[]) {
  for (let i = 0; i < array.length; i++) {
    let j = random_int(i, array.length);
    swap(array, i, j);
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

function is_connected<K, V>(node: BNode<K, V>): boolean {
  if (node.kind == "Leaf") {
    return true;
  } else {
    return (
      node.left_child.parent == node &&
      node.right_child.parent == node &&
      is_connected(node.left_child) &&
      is_connected(node.right_child)
    );
  }
}
