export { IntervalTree, ClosedInterval, INode };

import { Comparator, Ordering, ord_to_int } from "./comparator";
import { Interval } from "./lib";

class ClosedInterval<K> extends Interval<K> {
  min!: K;
  max!: K;

  constructor(min: K, max: K) {
    super(min, max);
  }
}

type INode<K> = ILeaf<K> | IInternal<K>;

class ILeaf<K> {
  kind: "Leaf" = "Leaf";
  key: K;
  left_list: ClosedInterval<K>[] = [];
  right_list: ClosedInterval<K>[] = [];

  constructor(key: K) {
    this.key = key;
  }
}

class IInternal<K> {
  kind: "Internal" = "Internal";
  key: K;
  left_list: ClosedInterval<K>[] = [];
  right_list: ClosedInterval<K>[] = [];
  left_child: INode<K>;
  right_child: INode<K>;

  constructor(key: K, left_child: INode<K>, right_child: INode<K>) {
    this.key = key;
    this.left_child = left_child;
    this.right_child = right_child;
  }
}

function make_tree<K>(sorted_keys: K[]): INode<K> | null {
  if (sorted_keys.length == 0) {
    return null;
  }

  // nodes with the smallest key under them
  let nodes: Array<[K, INode<K>]> = sorted_keys.map((k) => [k, new ILeaf(k)]);
  while (nodes.length > 1) {
    let new_nodes: Array<[K, INode<K>]> = [];
    for (let i = 0; i < nodes.length; i += 2) {
      if (i + 1 >= nodes.length) {
        new_nodes.push(nodes[i]);
      } else {
        let [left_min, left_child] = nodes[i];
        let [right_min, right_child] = nodes[i + 1];
        new_nodes.push([left_min, new IInternal(right_min, left_child, right_child)]);
      }
    }
    nodes = new_nodes;
  }

  return nodes[0][1];
}

function narrow_to_node<K>(cmp: Comparator<K>, interval: ClosedInterval<K>, node: INode<K>): INode<K> {
  while (node.kind == "Internal") {
    if (interval.close_close(node.key, cmp)) {
      return node;
    } else if (cmp(interval.max, node.key) == Ordering.LT) {
      node = node.left_child;
    } else {
      // node.key > interval.max
      node = node.right_child;
    }
  }

  return node;
}

function insert_interval_left<K>(cmp: Comparator<K>, interval: ClosedInterval<K>, node: INode<K>) {
  node = narrow_to_node(cmp, interval, node);
  node.left_list.push(interval);
}

function insert_interval_right<K>(cmp: Comparator<K>, interval: ClosedInterval<K>, node: INode<K>) {
  node = narrow_to_node(cmp, interval, node);
  node.right_list.push(interval);
}

class IntervalTree<K> {
  cmp: Comparator<K>;
  root: INode<K> | null;

  constructor(cmp: Comparator<K>, intervals: ClosedInterval<K>[]) {
    let sorted_keys = intervals
      .map((i) => [i.min, i.max])
      .reduce((prev, next) => prev.concat(next), [])
      .filter((n) => n != null) as K[];

    sorted_keys = sorted_keys.sort((a, b) => ord_to_int(cmp(a, b)));

    let root = make_tree(sorted_keys);

    if (root) {
      intervals = intervals.sort((a, b) => ord_to_int(cmp(b.min, a.min)));
      for (let interval of intervals) {
        insert_interval_left(cmp, interval, root);
      }

      intervals = intervals.sort((a, b) => ord_to_int(cmp(a.max, b.max)));
      for (let interval of intervals) {
        insert_interval_right(cmp, interval, root);
      }
    }

    this.cmp = cmp;
    this.root = root;
  }

  find_intervals(search_key: K): ClosedInterval<K>[] {
    let found = [];
    let node = this.root;
    let cmp = this.cmp;

    while (node) {
      if (cmp(search_key, node.key) == Ordering.LT) {
        let i = node.left_list.length - 1;
        while (i >= 0 && cmp(node.left_list[i].min, search_key) != Ordering.GT) {
          found.push(node.left_list[i]);
          i -= 1;
        }

        if (node.kind == "Internal") {
          node = node.left_child;
        } else {
          node = null;
        }
      } else {
        let i = node.right_list.length - 1;
        while (i >= 0 && cmp(search_key, node.right_list[i].max) != Ordering.GT) {
          found.push(node.right_list[i]);
          i -= 1;
        }

        if (node.kind == "Internal") {
          node = node.right_child;
        } else {
          node = null;
        }
      }
    }

    return found;
  }
}
