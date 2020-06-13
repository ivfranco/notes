import { RBNode, Color, RBTree } from "../src/RB_tree";
import { native_comparator } from "../src/comparator";
import { expect } from "chai";
import { grow_random_tree, is_ordered, is_connected } from "./test_utils";

function random_tree(min: number, max: number): RBTree<number, number> {
  let tree = new RBTree<number, number>(native_comparator);
  grow_random_tree(min, max, tree);
  return tree;
}

function well_colored<K, V>(tree: RBTree<K, V>): boolean {
  let root = tree.root;
  if (root) {
    return root.color == Color.BLACK && well_colored_node(root) !== false;
  } else {
    return true;
  }
}

// in most case `===` and `==` is the same in typescript
// but in this function number and boolean value may be compared
function well_colored_node<K, V>(node: RBNode<K, V>): number | false {
  if (node.kind == "Leaf") {
    return node.color == Color.BLACK ? 1 : 0;
  } else {
    let left_black_height = well_colored_node(node.left_child);
    let right_black_height = well_colored_node(node.right_child);

    if (left_black_height === false || right_black_height === false || left_black_height !== right_black_height) {
      // children of a node must have the same black height
      return false;
    } else if (
      // both children of a red node must be black
      node.color == Color.BLACK ||
      (node.left_child.color == Color.BLACK && node.right_child.color == Color.BLACK)
    ) {
      return node.color == Color.BLACK ? left_black_height + 1 : left_black_height;
    } else {
      return false;
    }
  }
}

describe("RB tree set operations", function () {
  describe("insert and find", function () {
    it("should find inserted keys and nothing else", function () {
      const SIZE: number = 20;
      let tree = random_tree(0, SIZE - 1);
      let root = <RBNode<number, number>>tree.root;

      // console.log(
      //   tree.to_dot((node) => {
      //     let color = node.color == Color.BLACK ? "B" : "R";
      //     return `${node.key}, ${color}`;
      //   })
      // );

      expect(is_ordered(root, tree.cmp), "is ordered after insertion").true;
      expect(is_connected(root), "nodes are correctly connected").true;
      expect(well_colored(tree), "nodes should be balanced").true;

      for (let i = 0; i < SIZE; i++) {
        expect(tree.find(i)).equal(i);
      }

      for (let i = SIZE; i < SIZE + 10; i++) {
        expect(tree.find(i)).equal(null);
      }
    });
  });

  // describe("insert, delete and find", function () {
  //   it("should not find deleted keys", function () {
  //     const SIZE: number = 20;
  //     let tree = random_tree(0, SIZE - 1);

  //     expect(tree.delete(3)).equal(3);
  //     expect(tree.delete(7)).equal(7);

  //     let root = <RBNode<number, number>>tree.root;
  //     expect(is_ordered(root, tree.cmp), "is ordered after insertion and deletion").true;
  //     expect(is_connected(root), "nodes are correctly connected").true;
  //     expect(well_colored(tree), "nodes should be balanced").true;

  //     for (let i = 0; i < SIZE; i++) {
  //       if (i == 3 || i == 7) {
  //         expect(tree.find(i)).equal(null);
  //       } else {
  //         expect(tree.find(i)).equal(i);
  //       }
  //     }
  //   });
  // });
});
