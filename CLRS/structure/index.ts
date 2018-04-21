import { BTree, BTreeNode } from "./b-tree";
import { FHeapNode } from "./fibonacci-heap";

function main() {
  fheap_test();
}

function problem_18_2_1() {
  let A = "F S Q K C L H T V W M R N P A B X Y D Z E".split(" ");
  let btree: BTree<string> = new BTree(2);

  for (let k of A) {
    btree.insert(k);
    btree.diagnose();
  }

  console.log("Final configuration");
  console.log(btree.show());
}

function problem_18_3_1() {
  let leaves = [
    "A C",
    "J K",
    "N O",
    "Q R S",
    "U V",
    "Y Z",
  ].map(s => {
    let key = s.split(" ");
    let n = key.length;
    let node: BTreeNode<string> = new BTreeNode();
    node.n = n;
    node.key = key;
    return node;
  });

  let root = new BTreeNode();
  root.leaf = false;
  root.key = "E L P T X".split(" ");
  root.n = root.key.length;
  root.c = leaves;

  let btree = new BTree(3);
  btree.root = root;

  console.log(btree.show());

  let D = "C P V".split(" ");
  for (let k of D) {
    console.log(`Deleting ${k}`);
    btree.delete(k);
    btree.diagnose();
    console.log(btree.show());
  }
}

function problem_18_3_2() {
  let A = "F S Q K C L H T V W M R N P A B X Y D Z E".split(" ");
  let btree: BTree<string> = new BTree(2);

  for (let k of A) {
    btree.insert(k);
    btree.diagnose();
  }

  console.log(btree.show());

  for (let k of A) {
    console.log(`Deleting ${k}`);
    btree.delete(k);
    console.log(btree.show());
    console.assert(btree.search(k) === null, "Deleted key still in the tree");
    btree.diagnose();
  }
}

function problem_18_2() {
  let A = "F S Q K C L H T V W M R N P A B X Y D Z E".split(" ");
  let btree: BTree<string> = new BTree(2);

  for (let k of A) {
    btree.insert(k);
    btree.diagnose();
  }

  let [LT, GT] = btree.split("L");
  console.log(LT.show());
  console.log(GT.show());
  LT.diagnose();
  GT.diagnose();
}

function fheap_test() {
  function adjacent<T>(node: FHeapNode<T>): [T, T, T] {
    return [node.left.key, node.key, node.right.key];
  }
  let x = FHeapNode.from([1, 2, 3, 4, 5]);
  let y = FHeapNode.from([6, 7, 8, 9, 0]);

  let A = Array.from(x.siblings()).map(adjacent);
  let B = Array.from(y.siblings()).map(adjacent);
  console.log(A);
  console.log(B);

  x.concat(y);
  let C = Array.from(x.siblings()).map(adjacent);
  console.log(C);

  while (!x.isSingleton()) {
    x.right.remove();
    let D = Array.from(x.siblings()).map(adjacent);
    console.log(D);
  }
}

main();
