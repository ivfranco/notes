import { randomAB, shuffle, isSorted } from "../util";
import { Queue } from "./queue";
import { SList, SNode } from "./slist";
import { DList, DNode } from "./dlist";
import {
  Tree,
  TreeNode,
  randomTree,
  printTree,
  printTreeStack,
  printTreeConstant,
  treeParent
} from "./tree";
import { Huge } from "./direct";
import {
  LinearProbing,
  QuadraticProbing,
  DoubleHashing
} from "./hashtable";
import { RadixTree } from "./radix-tree";
import { RBTree } from "./redblack-tree";

function main() {
  problem_13_3_2();
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
  let node = randomTree([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
  printTree(node);
  console.log("");
  printTreeStack(node);
  console.log("");
  printTreeConstant(node);
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

  console.log(keys.map(k => Math.floor(m * ((k * A) % 1))));
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
  console.log("Quadratic probing:")
  quadratic.report();
  console.log("Double hashing:")
  double.report();
}

function treeOfHeight(h: number): TreeNode<number> {
  let A = [1, 4, 5, 10, 16, 17, 21];
  let node = randomTree(A);
  while (node.height() !== h) {
    node = randomTree(A);
  }
  return node;
}

function problem_12_1_1() {
  for (let i = 2; i <= 6; i++) {
    console.log(`Height ${i}:`);
    console.log(treeOfHeight(i).show());
  }
}

function problem_12_1_3() {
  let A = [1, 4, 5, 10, 16, 17, 21];
  let node = randomTree(A);
  let sorted = [];
  for (let a of node.inorder()) {
    sorted.push(a);
  }
  console.log(node.show());
  printTreeConstant(node);
  console.log(sorted);
  return isSorted(sorted);
}

function problem_12_3_5() {
  let A = [1, 4, 5, 10, 16, 17, 21];
  let k = A[randomAB(0, A.length - 1)];
  let T = new Tree();
  T.root = randomTree(A);

  let p = treeParent(<TreeNode<number>>T.search(k), T);
  console.log(T.root.show());
  console.log(k)
  console.log(p !== null ? p.key : null);
}

function problem_12_4_3() {
  let A = [1, 2, 3];
  for (let i = 0; i < 10; i++) {
    let node = randomTree(A);
    console.log(node.show());
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
  let node = randomTree(A);
  while (node.height() !== 3) {
    node = randomTree(A);
  }
  console.log(node.show());
}

function problem_13_3_2() {
  let A = [41, 38, 31, 12, 19, 8];
  let rb = new RBTree();

  for (let k of A) {
    rb.insert(k);
  }

  console.log(rb.show());

  for (let k of A) {
    let z = rb.search(k);
    if (z === null) {
      throw "Error: Search fail";
    } else {
      rb.delete(z);
    }
    console.log(rb.show());
  }

}

main();