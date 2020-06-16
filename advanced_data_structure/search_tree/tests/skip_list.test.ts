import { SkipList, Level, SkipNode } from "../src/skip_list";
import { native_comparator, Comparator, sorted } from "../src/comparator";
import { expect } from "chai";
import { grow_random_tree } from "./test_utils";

function to_array<K, V>(level: Level<K, V>): Array<K> {
  let keys = [];
  let node = level.head;
  while (node) {
    keys.push(node.key);
    node = node.next;
  }
  return keys;
}

function is_subset(lhs: Array<number>, rhs: Array<number>): boolean {
  let set = new Set(rhs);
  return lhs.every((n) => set.has(n));
}

function structure_check(list: SkipList<number, number>): boolean {
  let level: Level<number, number> | null = list.top;

  while (level) {
    let keys = to_array(level);

    if (!sorted(keys, list.cmp)) {
      return false;
    }

    if (level.down) {
      let down_keys = to_array(level.down);
      if (!is_subset(keys, down_keys)) {
        return false;
      }
    }

    level = level.down;
  }

  return true;
}

function random_tree(min: number, max: number): SkipList<number, number> {
  let tree = new SkipList<number, number>(native_comparator);
  grow_random_tree(min, max, tree);
  return tree;
}

describe("Skip list set operations", function () {
  describe("insert and find", function () {
    it("should find inserted keys and nothing else", function () {
      const SIZE: number = 20;
      let tree = random_tree(0, SIZE - 1);

      expect(structure_check(tree), "each level should be sorted, higher levels are subset of lower levels").true;

      for (let i = 0; i < SIZE; i++) {
        expect(tree.find(i)).equal(i);
      }

      for (let i = SIZE; i < SIZE + 10; i++) {
        expect(tree.find(i)).equal(null);
      }
    });
  });

  describe("insert, delete and find", function () {
    it("should not find deleted keys", function () {
      const SIZE: number = 20;
      let tree = random_tree(0, SIZE - 1);

      expect(structure_check(tree), "each level should be sorted, higher levels are subset of lower levels").true;

      expect(tree.delete(3)).equal(3);
      expect(tree.delete(7)).equal(7);

      for (let i = 0; i < SIZE; i++) {
        if (i == 3 || i == 7) {
          expect(tree.find(i)).equal(null);
        } else {
          expect(tree.find(i)).equal(i);
        }
      }
    });
  });
});
