import { AVLTree, AVLNode } from "../src/AVL_tree";
import { native_comparator } from "../src/comparator";
import { expect } from "chai";
import { grow_random_tree, is_ordered, is_connected } from "./test_utils";

function random_tree(min: number, max: number): AVLTree<number, number> {
  let tree = new AVLTree<number, number>(native_comparator);
  grow_random_tree(min, max, tree);
  return tree;
}

function is_balanced<K, V>(node: AVLNode<K, V>): boolean {
  if (node.kind == "Leaf") {
    return true;
  } else {
    return (
      is_balanced(node.left_child) &&
      is_balanced(node.right_child) &&
      node.get_height() == node.recalc_height() &&
      Math.abs(node.left_child.get_height() - node.right_child.get_height()) <= 1
    );
  }
}

describe("AVL tree set operations", function () {
  describe("insert and find", function () {
    it("should find inserted keys and nothing else", function () {
      const SIZE: number = 20;
      let tree = random_tree(0, SIZE - 1);
      let root = <AVLNode<number, number>>tree.root;

      expect(is_ordered(root, tree.cmp), "is ordered after insertion").true;
      expect(is_connected(root), "nodes are correctly connected").true;
      expect(is_balanced(root), "nodes should be balanced").true;

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

      expect(tree.delete(3)).equal(3);
      expect(tree.delete(7)).equal(7);

      let root = <AVLNode<number, number>>tree.root;
      expect(is_ordered(root, tree.cmp), "is ordered after insertion and deletion").true;
      expect(is_connected(root), "nodes are correctly connected").true;
      expect(is_balanced(root), "nodes should be balanced").true;

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
