import { SplayTree } from "../src/splay_tree";
import { native_comparator } from "../src/comparator";
import { expect } from "chai";
import { grow_random_tree, is_ordered } from "./test_utils";

function random_tree(min: number, max: number): SplayTree<number, number> {
  let tree = new SplayTree<number, number>(native_comparator);
  grow_random_tree(min, max, tree);
  return tree;
}
