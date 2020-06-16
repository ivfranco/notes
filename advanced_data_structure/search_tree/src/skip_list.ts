export { SkipList, Level, SkipNode };

import { TreeLike } from "./lib";
import { Comparator, Ordering } from "./comparator";

const P: number = 1 / Math.E;

function random_height(prob: number): number {
  let height = 1;
  while (Math.random() < prob) {
    height += 1;
  }

  return height;
}

class SkipList<K, V> implements TreeLike<K, V> {
  cmp: Comparator<K>;
  top: Level<K, V>;
  height: number;

  constructor(cmp: Comparator<K>) {
    this.cmp = cmp;
    this.top = new Level();
    this.height = 1;
  }

  find(search_key: K): V | null {
    return this.top.find(search_key, this.cmp);
  }

  extend_height(new_height: number) {
    while (this.height < new_height) {
      let new_level = new Level<K, V>();
      new_level.down = this.top;
      this.top = new_level;
      this.height += 1;
    }
  }

  insert(key: K, value: V) {
    let node = this.top.narrow_to_top_node(key, this.cmp);
    // replace old value if key exists
    if (node) {
      node.down_to_leaf().value = value;
      return;
    }

    let height = random_height(P);
    if (this.height < height) {
      this.extend_height(height);
    }

    let level = this.top;
    let level_height = this.height;

    while (level_height > height) {
      level = level.down!;
      level_height -= 1;
    }

    let new_node = SkipInternal.create_vertical_list(key, value, height);
    for (let i = 0; i < height; i++) {
      level.insert(new_node, this.cmp);
      level = level.down!;
      new_node = <SkipInternal<K, V>>new_node.down;
    }
  }

  delete(key: K): V | null {
    let level: Level<K, V> | null = this.top;
    let node!: SkipInternal<K, V> | null;

    // executed at least once
    while (level) {
      node = level.delete(key, this.cmp);
      level = level.down;
    }

    if (node) {
      return node.down_to_leaf().value;
    } else {
      return null;
    }
  }

  show(): string {
    let str = "";
    let level: Level<K, V> | null = this.top;
    while (level) {
      str += `${level.show()}\n`;
      level = level.down;
    }

    return str;
  }
}

class Level<K, V> {
  head: SkipInternal<K, V> | null = null;
  down: Level<K, V> | null = null;

  find_max(search_key: K, cmp: Comparator<K>): SkipInternal<K, V> | null {
    if (this.head == null) {
      return null;
    }

    return this.head.find_max(search_key, cmp);
  }

  // return the highest level node with a given key
  narrow_to_top_node(search_key: K, cmp: Comparator<K>): SkipInternal<K, V> | null {
    if (this.head == null) {
      // level is empty
      return null;
    }

    return this.head.narrow_to_top_node(search_key, cmp);
  }

  find(search_key: K, cmp: Comparator<K>): V | null {
    let level: Level<K, V> | null = this;

    while (level) {
      // if the level is empty or the first element in the level has key greater than search_key, go down a level
      if (level.head == null || cmp(search_key, level.head.key) == Ordering.LT) {
        level = level.down;
      } else {
        let node = level.head.narrow_to_top_node(search_key, cmp);
        if (node) {
          return node.down_to_leaf().value;
        } else {
          return null;
        }
      }
    }

    // search_key is smaller than the smallest key in the skip list
    return null;
  }

  insert(node: SkipInternal<K, V>, cmp: Comparator<K>) {
    if (this.head == null) {
      this.head = node;
      return;
    }

    let head = this.head!;
    let max = head.find_max(node.key, cmp);
    if (max) {
      max.insert(node);
    } else {
      this.head = node;
      node.next = head;
    }
  }

  delete(key: K, cmp: Comparator<K>): SkipInternal<K, V> | null {
    if (this.head == null) {
      return null;
    }

    let node = this.head;

    if (cmp(key, node.key) == Ordering.EQ) {
      this.head = node.next;
      return node;
    }

    // invariant: node.key < delete key
    while (node.next) {
      let next = node.next;
      switch (cmp(key, next.key)) {
        case Ordering.EQ:
          node.next = next.next;
          return next;
        case Ordering.GT:
          node = next;
          break;
        case Ordering.LT:
          return null;
      }
    }

    return null;
  }

  show(): string {
    let str = "Level:";
    let node = this.head;

    while (node) {
      str += ` ${node.key}`;
      node = node.next;
    }

    return str;
  }
}

type SkipNode<K, V> = SkipLeaf<V> | SkipInternal<K, V>;

class SkipInternal<K, V> {
  kind: "Internal" = "Internal";
  key: K;
  next: SkipInternal<K, V> | null = null;
  down: SkipNode<K, V>;

  constructor(key: K, down: SkipNode<K, V>) {
    this.key = key;
    this.down = down;
  }

  static create_vertical_list<K, V>(key: K, value: V, height: number): SkipInternal<K, V> {
    console.assert(height >= 1);

    let leaf = new SkipLeaf(value);
    let node = new SkipInternal(key, leaf);

    for (let i = 1; i < height; i++) {
      node = new SkipInternal(key, node);
    }

    return node;
  }

  down_to_leaf(): SkipLeaf<V> {
    let node: SkipNode<K, V> = this;

    while (node.kind == "Internal") {
      node = node.down;
    }

    return node;
  }

  // return the node to the right of the current node, including the current node,
  // with greatest key smaller than or equal to the given search key
  // or null if the current node has key greater than the given search key
  find_max(search_key: K, cmp: Comparator<K>): SkipInternal<K, V> | null {
    let node: SkipInternal<K, V> = this;

    if (cmp(search_key, node.key) == Ordering.LT) {
      return null;
    }

    while (node.next) {
      if (cmp(search_key, node.next.key) != Ordering.LT) {
        node = node.next;
      } else {
        break;
      }
    }

    return node;
  }

  narrow_to_top_node(search_key: K, cmp: Comparator<K>): SkipInternal<K, V> | null {
    let node: SkipNode<K, V> = this;
    while (node.kind == "Internal") {
      let max = node.find_max(search_key, cmp);
      if (max == null) {
        return null;
      }
      if (cmp(search_key, max.key) == Ordering.EQ) {
        return max;
      } else {
        node = max.down;
      }
    }

    return null;
  }

  // insert a new node between this and this.next
  insert(node: SkipInternal<K, V>) {
    let next = this.next;
    this.next = node;
    node.next = next;
  }
}

class SkipLeaf<V> {
  kind: "Leaf" = "Leaf";
  value: V;

  constructor(value: V) {
    this.value = value;
  }
}
