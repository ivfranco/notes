import { isSorted, randomAB, shuffle } from "../util";
import { josephus, OSTree } from "./augmented-redblack-tree";
import { AVLTree } from "./avl-tree";
import { Huge } from "./direct";
import { DList, DNode } from "./dlist";
import {
  DoubleHashing,
  LinearProbing,
  QuadraticProbing,
} from "./hashtable";
import { PTree } from "./persistent-tree";
import { Queue } from "./queue";
import { RadixTree } from "./radix-tree";
import { RBTree } from "./redblack-tree";
import { SList, SNode } from "./slist";
import { Treap } from "./treap";
import {
  printTree,
  printTreeConstant,
  printTreeStack,
  randomTree,
  Tree,
  TreeNode,
  treeParent,
} from "./tree";

function main() {
  problem_14_2();
}

function problem_10_1_1() {
  let S = [];
  S.push(4);
  console.log(S);
  S.push(1);
  console.log(S);
  S.push(3);
  console.log(S);
  console.log("Pop: ", S.pop());
  console.log(S);
  S.push(8);
  console.log(S);
  console.log("Pop: ", S.pop());
  console.log(S);
}

function problem_10_1_3() {
  let Q = new Queue(6);

  Q.enqueue(4);
  Q.report();
  Q.enqueue(1);
  Q.report();
  Q.enqueue(3);
  Q.report();
  console.log("Dequeue: ", Q.dequeue());
  Q.report();
  Q.enqueue(8);
  Q.report();
  console.log("Dequeue: ", Q.dequeue());
  Q.report();
}

function problem_10_2_6() {
  let dlist = new DList();
  for (let i = 0; i < 10; i++) {
    dlist.insert(i);
  }
  console.log(dlist.show());
  for (let i = 0; i < 10; i++) {
    let node = dlist.search(i);
    if (node !== null) {
      dlist.delete(node);
    }
  }
  console.log(dlist.show());

  let dlist_a = DList.fromArray([1, 2, 3, 4, 5]);
  let dlist_b = DList.fromArray([6, 7, 8, 9, 10]);
  dlist_a.concat(dlist_b);
  console.log(dlist_a.show());
}

function problem_10_2_7() {
  let A = [1, 2, 3, 4, 5];
  let slist = new SList();
  for (let a of A) {
    slist.insert(a);
  }
  slist.reverse();
  console.log(slist.show());
}

function problem_10_4_2() {
  let tree: Tree<number> = new Tree();
  randomTree(tree, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
  let root = tree.root as TreeNode<number>;
  printTree(root);
  console.log("");
  printTreeStack(root);
  console.log("");
  printTreeConstant(root);
}

function problem_11_1_4() {
  let huge = new Huge();
  let copy = new Set();
  for (let i = 0; i < 10; i++) {
    let n = randomAB(0, 10);
    huge.insert(n);
    copy.add(n);
  }
  for (let i = 0; i < 10; i++) {
    let n = randomAB(0, 10);
    huge.delete(n);
    copy.delete(n);
  }

  console.log(copy);
  console.log(huge.list());
}

function problem_11_3_4() {
  let keys = [61, 62, 63, 64, 65];
  let A = (Math.sqrt(5) - 1) / 2;
  let m = 1000;

  console.log(keys.map((k) => Math.floor(m * ((k * A) % 1))));
}

function problem_11_4_1() {
  let A = [10, 22, 31, 4, 15, 28, 17, 88, 59];
  let m = 11;

  let linear = new LinearProbing(m);
  let quadratic = new QuadraticProbing(m);
  let double = new DoubleHashing(m);
  for (let k of A) {
    linear.insert(k);
    quadratic.insert(k);
    double.insert(k);
  }
  console.log("Linear probing:");
  linear.report();
  console.log("Quadratic probing:");
  quadratic.report();
  console.log("Double hashing:");
  double.report();
}

function treeOfHeight(h: number): TreeNode<number> {
  let A = [1, 4, 5, 10, 16, 17, 21];
  let tree: Tree<number> = new Tree();
  randomTree(tree, A);
  while (!tree.root || tree.root.height() !== h) {
    randomTree(tree, A);
  }
  return tree.root;
}

function problem_12_1_1() {
  for (let i = 2; i <= 6; i++) {
    console.log(`Height ${i}:`);
    console.log(treeOfHeight(i).show());
  }
}

function problem_12_1_3() {
  let A = [1, 4, 5, 10, 16, 17, 21];
  let tree: Tree<number> = new Tree();
  randomTree(tree, A);
  let sorted = [];
  for (let a of tree) {
    sorted.push(a);
  }
  console.log(tree.show());
  let root = tree.root as TreeNode<number>;
  printTreeConstant(root);
  console.log(sorted);
  return isSorted(sorted);
}

function problem_12_3_5() {
  let A = [1, 4, 5, 10, 16, 17, 21];
  let k = A[randomAB(0, A.length - 1)];
  let T: Tree<number> = new Tree();
  randomTree(T, A);

  let p = treeParent(T.search(k) as TreeNode<number>, T);
  console.log(T.show());
  console.log(k);
  console.log(p !== null ? p.key : null);
}

function problem_12_4_3() {
  let A = [1, 2, 3];
  for (let i = 0; i < 10; i++) {
    let tree: Tree<number> = new Tree();
    randomTree(tree, A);
    console.log(tree.show());
  }
}

function problem_12_2() {
  let A = ["0", "011", "10", "100", "1011"];
  shuffle(A);
  let rtree = new RadixTree();
  for (let str of A) {
    rtree.insert(str);
  }
  let sorted = [];
  for (let str of rtree.preorder()) {
    sorted.push(str);
  }
  console.log(sorted);
}

function problem_13_1_1() {
  let A = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
  let tree: Tree<number> = new Tree();
  while (tree.height() !== 3) {
    tree = new Tree();
    randomTree(tree, A);
  }
  console.log(tree.show());
}

function problem_13_3_2() {
  let A = [41, 38, 31, 12, 19, 8];
  let rb = new RBTree();

  for (let k of A) {
    rb.insert(k);
  }

  console.log(rb.show());
}

function problem_13_4_3() {
  let A = [41, 38, 31, 12, 19, 8];
  let D = [8, 12, 19, 31, 38, 41];

  let rb = new RBTree();

  for (let k of A) {
    rb.insert(k);
  }

  console.log(rb.show());

  for (let k of D) {
    console.log(`Deleting ${k}`);
    let z = rb.search(k);
    if (z) {
      rb.delete(z);
    }
    console.log(rb.show());
  }
}

function problem_13_1() {
  let A = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  shuffle(A);
  let trees: Array<PTree<number>> = [];
  let ptree: PTree<number> = new PTree();
  for (let k of A) {
    ptree = ptree.insert(k);
    trees.push(ptree);
  }
  for (let t of trees) {
    console.log(t.show());
  }
}

function problem_13_3() {
  let A = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  shuffle(A);
  let avl = new AVLTree();
  for (let k of A) {
    console.log(`Inserting ${k}`);
    avl.insert(k);
    console.log(avl.show());
  }
}

function problem_13_4() {
  let A = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  shuffle(A);
  let treap = new Treap();
  for (let k of A) {
    treap.insert(k);
  }
  console.log(treap.show());
  treap.diagnose();
}

function problem_14_1_1() {
  let A = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  shuffle(A);
  let os: OSTree<number> = new OSTree();
  randomTree(os, A);

  console.log(os.show());
  let i = randomAB(1, 10);
  console.log(`Selecting rank ${i}...`);
  let r = os.select(i);
  console.log(r.show());
  console.log(r.rank());
  os.diagose();
}

function problem_14_2() {
  console.log(josephus(7, 3));
}

main();
