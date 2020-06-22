import { Interval } from "../src/lib";
import { SplayTree, SplayNode } from "../src/splay_tree";
import { native_comparator, Comparator } from "../src/comparator";
import { expect } from "chai";
import { grow_random_tree, random_int } from "./test_utils";

function random_tree(min: number, max: number): SplayTree<number, number> {
  let tree = new SplayTree<number, number>(native_comparator);
  grow_random_tree(min, max, tree);
  return tree;
}

function is_ordered<K, V>(node: SplayNode<K, V>, cmp: Comparator<K>): boolean {
  let stack: [SplayNode<K, V>, Interval<K>][] = [[node, new Interval<K>(null, null)]];

  while (stack.length != 0) {
    let [node, interval] = stack.pop()!;
    if (!interval.open_open(node.key, cmp)) {
      return false;
    }

    if (node.left_child) {
      stack.push([node.left_child, new Interval(interval.min, node.key)]);
    }

    if (node.right_child) {
      stack.push([node.right_child, new Interval(node.key, interval.max)]);
    }
  }

  return true;
}

function is_connected<K, V>(node: SplayNode<K, V>): boolean {
  return (
    (node.left_child == null || (node.left_child.parent == node && is_connected(node.left_child))) &&
    (node.right_child == null || (node.right_child.parent == node && is_connected(node.right_child)))
  );
}

describe("Splay tree set operations", function () {
  describe("insert and find", function () {
    it("should find inserted keys and nothing else", function () {
      for (let i = 0; i < 100; i++) {
        const SIZE: number = random_int(1, 20);
        let tree = random_tree(0, SIZE - 1);
        let root = tree.root!;

        expect(is_connected(root), "nodes are correctly connected").true;
        expect(is_ordered(root, tree.cmp), "is ordered after insertion").true;

        for (let i = 0; i < SIZE; i++) {
          expect(tree.find(i)).equal(i);
          // find operation changes the structure of splay tree
          expect(is_connected(root), "nodes are correctly connected after find").true;
          expect(is_ordered(root, tree.cmp), "is ordered after insertion after find").true;
        }

        for (let i = SIZE; i < SIZE + 10; i++) {
          expect(tree.find(i)).equal(null);
        }
      }
    });
  });

  describe("insert, delete and find", function () {
    it("should not find deleted keys", function () {
      for (let i = 0; i < 10; i++) {
        const SIZE: number = 20;
        let tree = random_tree(0, SIZE - 1);

        expect(tree.delete(2)).equal(2);

        let root = tree.root!;
        expect(is_connected(root), "nodes are correctly connected").true;
        expect(is_ordered(root, tree.cmp), "is ordered after insertion and deletion").true;

        for (let j = 0; j < SIZE; j++) {
          if (j == 2) {
            expect(tree.find(j)).equal(null);
          } else {
            expect(tree.find(j)).equal(j);
            expect(is_connected(root), "nodes are correctly connected after find").true;
            expect(is_ordered(root, tree.cmp), "is ordered after find").true;
          }
        }
      }
    });
  });
});
