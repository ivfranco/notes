export { SegmentTree };

import { Comparator, Ordering, WithBottom, Bottom, ord_to_int, cmp_with_bottom, dedup_sorted } from "./comparator";
import { Interval } from "./lib";
import { make_tree } from "./interval_tree";

class SNode<K> {
  key: WithBottom<K>;
  list: Interval<K>[] = [];
  left: SNode<K> | null = null;
  right: SNode<K> | null = null;

  static new_leaf<K>(key: K): SNode<K> {
    return new SNode(key);
  }

  static new_internal<K>(key: WithBottom<K>, left: SNode<K>, right: SNode<K>): SNode<K> {
    let internal = new SNode(key);
    internal.left = left;
    internal.right = right;
    return internal;
  }

  constructor(key: WithBottom<K>) {
    this.key = key;
  }
}

function make_segment_tree<K>(sorted_keys: K[]): SNode<K> | null {
  return make_tree(sorted_keys, SNode.new_leaf, SNode.new_internal);
}

function insert_segment<K>(cmp: Comparator<WithBottom<K>>, interval: Interval<K>, node: SNode<K>) {
  let stack: Array<[Interval<K>, SNode<WithBottom<K>>]> = [[Interval.R<K>(), node]];

  while (stack.length > 0) {
    let [i, n] = stack.pop()!;

    if (interval.contain(i, cmp)) {
      n.list.push(interval);
    } else if (interval.overlap(i, cmp)) {
      if (n.left) {
        // node with key `Bottom` must be a leaf in bottom-up optimal tree
        stack.push([new Interval(i.min, <K>n.key), n.left]);
      }
      if (n.right) {
        stack.push([new Interval(<K>n.key, i.max), n.right]);
      }
    }
  }
}

class SegmentTree<K> {
  cmp: Comparator<WithBottom<K>>;
  root: SNode<K> | null = null;

  constructor(cmp: Comparator<K>, intervals: Interval<K>[]) {
    this.cmp = cmp_with_bottom(cmp);

    let end_points = intervals
      .map((i) => i.min)
      .concat(intervals.map((i) => i.max))
      .filter((e) => e != null) as WithBottom<K>[];

    end_points.push(Bottom);

    end_points = end_points.sort((a, b) => ord_to_int(this.cmp(a, b)));
    dedup_sorted(this.cmp, end_points);

    let root = make_segment_tree(end_points) as SNode<K> | null;

    if (root) {
      for (let interval of intervals) {
        insert_segment(this.cmp, interval, root);
      }
    }

    this.root = root;
  }

  find_intervals(search_key: K): Interval<K>[] {
    let found: Interval<K>[] = [];
    let node = this.root;

    while (node) {
      found = found.concat(node.list);
      if (this.cmp(search_key, node.key) == Ordering.LT) {
        node = node.left;
      } else {
        node = node.right;
      }
    }

    return found;
  }
}
