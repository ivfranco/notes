import { Leaf, Internal, Tree, left_rotation, right_rotation, connect_left, connect_right } from "./lib";
import { Comparator } from "./comparator";

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
  height: number;
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

class AVLTree<K, V> extends Tree<K, V> {
  cmp: Comparator<K>;
  root: AVLNode<K, V> | null = null;

  constructor(cmp: Comparator<K>) {
    super();
    this.cmp = cmp;
  }

  insert(key: K, value: V) {
    throw new Error("Not implemented");
  }

  delete(key: K): V | null {
    throw new Error("Not implemented");
  }

  find(search_key: K): V | null {
    throw new Error("Not implemented");
  }
}
