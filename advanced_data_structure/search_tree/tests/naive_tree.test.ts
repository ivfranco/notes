import { BNode, is_ordered, make_tree, make_tree_top_down, size, is_connected } from "../src/lib";
import { NaiveTree, NaiveFactory } from "../src/naive_tree";
import { native_comparator, array_comparator, Ordering } from "../src/comparator";
import { expect } from "chai";

// generates a random integer in the range [min, cap - 1]
function random_int(min: number, cap: number): number {
  return Math.floor(Math.random() * (cap - min)) + min;
}

function random_tree(min: number, max: number): NaiveTree<number, number> {
  let tree: NaiveTree<number, number> = new NaiveTree(native_comparator);

  let inputs = [];
  for (let i = min; i <= max; i++) {
    inputs.push(i);
  }
  shuffle(inputs);

  for (let i of inputs) {
    tree.insert(i, i);
  }

  return tree;
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

describe("tree set operations", function () {
  describe("insert and find", function () {
    it("should find inserted keys and nothing else", function () {
      let tree = random_tree(0, 9);

      expect(is_ordered(<BNode<number, number>>tree.root, tree.cmp), "is ordered after insertion").true;
      expect(is_connected(<BNode<number, number>>tree.root), "nodes are correctly connected").true;

      for (let i = 0; i < 10; i++) {
        expect(tree.find(i)).equal(i);
      }

      for (let i = 10; i < 20; i++) {
        expect(tree.find(i)).equal(null);
      }
    });
  });

  describe("insert, delete and find", function () {
    it("should not find deleted keys", function () {
      let tree = random_tree(0, 9);

      expect(tree.delete(3)).equal(3);
      expect(tree.delete(7)).equal(7);

      expect(is_ordered(<BNode<number, number>>tree.root, tree.cmp), "is ordered after insertion and deletion").true;

      for (let i = 0; i < 10; i++) {
        if (i == 3 || i == 7) {
          expect(tree.find(i)).equal(null);
        } else {
          expect(tree.find(i)).equal(i);
        }
      }
    });
  });

  describe("find interval", function () {
    it("should find all the keys in an interval", function () {
      let tree = random_tree(0, 19);
      tree.delete(10);
      let hits = tree
        .find_interval(8, 12)
        .map(([k, _]) => k)
        .sort((a, b) => a - b);
      let cmp = array_comparator(native_comparator);

      expect(cmp(hits, [8, 9, 11, 12])).equal(Ordering.EQ);
    });
  });
});

describe("make tree", function () {
  describe("bottom up make tree", function () {
    it("build a tree with provided keys and values", function () {
      const SIZE: number = 20;
      let pairs: [number, number][] = [];
      for (let i = 0; i < SIZE; i++) {
        pairs.push([i, i]);
      }

      let factory = new NaiveFactory();
      let node = <BNode<number, number>>make_tree(pairs, factory);

      expect(size(node), "expected size").equal(SIZE);
      expect(is_ordered(node, native_comparator)).true;
      expect(is_connected(node)).true;

      let tree = new NaiveTree(native_comparator);
      tree.root = node;

      for (let i = 0; i < SIZE; i++) {
        expect(tree.find(i), "expected value").equal(i);
      }
    });
  });

  describe("top down make tree", function () {
    it("build a tree with provided keys and values", function () {
      const SIZE: number = 20;
      let pairs: [number, number][] = [];
      for (let i = 0; i < SIZE; i++) {
        pairs.push([i, i]);
      }

      let factory = new NaiveFactory();
      let node = <BNode<number, number>>make_tree_top_down(pairs, factory);

      expect(size(node), "expected size").equal(SIZE);
      expect(is_ordered(node, native_comparator)).true;
      expect(is_connected(node)).true;

      let tree = new NaiveTree(native_comparator);
      tree.root = node;

      for (let i = 0; i < SIZE; i++) {
        expect(tree.find(i), "expected value").equal(i);
      }
    });
  });
});
