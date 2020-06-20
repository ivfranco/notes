import { AVLTree, AVLNode, join, split } from "../src/AVL_tree";
import { native_comparator } from "../src/comparator";
import { expect } from "chai";
import { random_int, grow_random_tree, is_ordered, is_connected } from "./test_utils";

function random_tree(min: number, max: number): AVLTree<number, number> {
  let tree = new AVLTree<number, number>(native_comparator);
  grow_random_tree(min, max, tree);
  return tree;
}

function height_balanced<K, V>(node: AVLNode<K, V> | null): boolean {
  if (node == null) {
    return true;
  }

  if (node.kind == "Leaf") {
    return true;
  } else {
    return (
      height_balanced(node.left_child) &&
      height_balanced(node.right_child) &&
      node.get_height() == node.recalc_height() &&
      Math.abs(node.left_child.get_height() - node.right_child.get_height()) <= 1
    );
  }
}

describe("AVL tree set operations", function () {
  describe("insert and find", function () {
    it("should find inserted keys and nothing else", function () {
      for (let i = 0; i < 10; i++) {
        const SIZE: number = random_int(0, 20);

        let tree = random_tree(0, SIZE - 1);
        let root = <AVLNode<number, number>>tree.root;

        expect(is_ordered(tree), "is ordered after insertion").true;
        expect(is_connected(root), "nodes are correctly connected").true;
        expect(height_balanced(root), "nodes should be balanced").true;
        expect(tree.size(), "should has the expected size").equals(SIZE);

        for (let i = 0; i < SIZE; i++) {
          expect(tree.find(i)).equal(i);
        }

        for (let i = SIZE; i < SIZE + 10; i++) {
          expect(tree.find(i)).equal(null);
        }
      }
    });
  });

  describe("insert, delete and find", function () {
    it("should not find deleted keys", function () {
      const SIZE: number = 20;
      let tree = random_tree(0, SIZE - 1);

      expect(tree.delete(3)).equal(3);
      expect(tree.delete(7)).equal(7);

      let root = <AVLNode<number, number>>tree.root;
      expect(is_ordered(tree), "is ordered after insertion and deletion").true;
      expect(is_connected(root), "nodes are correctly connected").true;
      expect(height_balanced(root), "nodes should be balanced").true;

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

describe("AVL tree join and split", function () {
  it("should join two trees to a valid AVL tree", function () {
    for (let i = 0; i < 10; i++) {
      const LEFT_SIZE = random_int(1, 10);
      const RIGHT_SIZE = random_int(1, 10);

      let left_tree = new AVLTree<number, number>(native_comparator);
      grow_random_tree(0, LEFT_SIZE - 1, left_tree);
      let right_tree = new AVLTree<number, number>(native_comparator);
      grow_random_tree(LEFT_SIZE, LEFT_SIZE + RIGHT_SIZE - 1, right_tree);

      let tree = join(LEFT_SIZE, left_tree, right_tree);

      expect(is_ordered(tree), "join tree should be ordered").true;
      expect(is_connected(tree.root!), "join tree should be connected").true;
      expect(height_balanced(tree.root!), "join tree should be height balanced").true;
      expect(tree.size(), "join tree should has the expected size").equals(LEFT_SIZE + RIGHT_SIZE);

      for (let i = 0; i < LEFT_SIZE + RIGHT_SIZE; i++) {
        expect(tree.find(i)).equals(i);
      }
    }
  });

  it("should split tree into two valid AVL trees", function () {
    for (let i = 0; i < 1000; i++) {
      const SIZE = 6;
      let split_key = random_int(0, SIZE);

      let tree = new AVLTree<number, number>(native_comparator);
      grow_random_tree(0, SIZE - 1, tree);
      // let dot = tree.to_dot((n) => `${n.key}`);

      let [left_tree, right_tree] = split(split_key, tree);

      // let snapshot = `
      //   split_key: ${split_key}\n
      //   tree: ${dot}\n
      //   left_tree: ${left_tree.to_dot((n) => `${n.key}`)}\n
      //   right_tree: ${right_tree.to_dot((n) => `${n.key}`)}\n
      // `;

      expect(is_ordered(left_tree), "left tree should be ordered").true;
      expect(is_connected(left_tree.root), "left tree should be connected").true;
      expect(height_balanced(left_tree.root), "left tree should be height balanced").true;
      expect(left_tree.size(), "left tree should has the expected size").equals(split_key);

      expect(is_ordered(right_tree), "right tree should be ordered").true;
      expect(is_connected(right_tree.root), "right tree should be connected").true;
      expect(height_balanced(right_tree.root), "right tree should be height balanced").true;
      expect(right_tree.size(), "right tree should has the expected size").equals(SIZE - split_key);

      for (let i = 0; i < split_key; i++) {
        expect(left_tree.find(i)).equals(i);
        expect(right_tree.find(i)).equals(null);
      }

      for (let i = split_key; i < SIZE; i++) {
        expect(left_tree.find(i)).equals(null);
        expect(right_tree.find(i)).equals(i);
      }
    }
  });
});
