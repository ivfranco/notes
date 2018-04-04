import { randomAB, shuffle, isSorted } from "../util";
import { Queue } from "./queue";
import { SList, SNode } from "./slist";
import { DList, DNode } from "./dlist";
import {
  TreeNode,
  randomTree,
  printTree,
  printTreeStack,
  printTreeConstant
} from "./tree";
import { Huge } from "./direct";
import {
  LinearProbing,
  QuadraticProbing,
  DoubleHashing
} from "./hashtable";

function main() {
  problem_12_1_1();
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

main();