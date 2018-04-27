import { randomAB } from "../util";
import { BTree, BTreeNode } from "./b-tree";
import { BHeap } from "./binomial-heap";
import { DSTreeNode } from "./disjoint-set-forest";
import { DSNode } from "./disjoint-set-list";
import { FHeap, FHeapNode } from "./fibonacci-heap";
import { offlineMinimum } from "./offline-minimum";
import { ProtoVEBTree } from "./proto-veb-tree";
import { VEBTree } from "./veb-tree";

function main() {
  problem_20_3_1();
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

function fheapnode_test() {
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

function problem_19_2_1() {
  let n39 = new FHeapNode(39);
  let n41 = new FHeapNode(41);
  let n18 = new FHeapNode(18);
  n18.insert(n39);
  let n52 = new FHeapNode(52);
  let n38 = new FHeapNode(38);
  n38.insert(n41);
  let n3 = new FHeapNode(3);
  n3.insert(n18);
  n3.insert(n52);
  n3.insert(n38);

  let n30 = new FHeapNode(30);
  let n17 = new FHeapNode(17);
  n17.insert(n30);

  let n35 = new FHeapNode(35);
  let n26 = new FHeapNode(26);
  n26.insert(n35);
  let n46 = new FHeapNode(46);
  let n24 = new FHeapNode(24);
  n24.insert(n26);
  n24.insert(n46);

  n3.append(n17);
  n17.append(n24);

  let H: FHeap<number> = new FHeap();
  H.min = n3;
  H.n = 12;

  H.diagnose();

  H.insert(23);
  H.insert(7);
  H.insert(21);

  H.diagnose();

  H.extractMin();
  console.log(`\n${H.show()}`);
  H.diagnose();

  H.extractMin();
  console.log(`\n${H.show()}`);
  H.diagnose();

  H.delete(n18);
  console.log(`\n${H.show()}`);
  H.diagnose();
}

function problem_19_2() {
  let bheap = new BHeap();
  for (let i = 0; i < 20; i++) {
    bheap.insert(i);
    bheap.diagnose();
  }
  console.log(bheap.show());
  bheap.diagnose();

  for (let i = 0; i < 20; i++) {
    bheap.extractMin();
    console.log(`\n${bheap.show()}`);
    bheap.diagnose();
  }
}

function problem_20_2_3() {
  let u = 256;
  let tree: ProtoVEBTree<number> = ProtoVEBTree.factory(u);

  //  insertion test
  for (let i = 0; i < u; i++) {
    tree.insert(i, i);
    tree.diagnose();
  }

  //  successor and search test
  for (let i = 0; i < u - 1; i++) {
    console.assert(tree.successor(i) === i + 1, `Successor test ${i}`);
    console.assert(tree.search(i) === i, `Search test ${i}`);
  }

  //  deletion and search test
  for (let i = 0; i < u; i++) {
    tree.delete(randomAB(0, u - 1));
    tree.diagnose();
    let v = tree.search(i);
    if (v) {
      console.assert(v === i);
    }
  }
}

function problem_20_3_1() {
  let u = 256;
  let tree: VEBTree<number> = VEBTree.factory(u);

  //  insertion test
  for (let i = 0; i < u; i++) {
    tree.insert(i, i);
    tree.diagnose();
  }

  //  successor and search test
  for (let i = 0; i < u - 1; i++) {
    console.assert(tree.successor(i) === i + 1, `Successor test ${i}`);
    console.assert(tree.search(i) === i, `Search test ${i}`);
  }

  //  deletion and search test
  for (let i = 0; i < u; i++) {
    tree.delete(randomAB(0, u - 1));
    tree.diagnose();
    let v = tree.search(i);
    if (v) {
      console.assert(v === i);
    }
  }
}

function problem_21_1_1() {
  let vertices = "a b c d e f g h i j k".split(" ").map(s => new DSNode(s));
  let [a, b, c, d, e, f, g, h, i, j, k] = vertices;
  let edges = [
    [d, i],
    [f, k],
    [g, i],
    [b, g],
    [a, h],
    [i, j],
    [d, k],
    [b, j],
    [d, f],
    [g, j],
    [a, e],
  ];

  for (let [u, v] of edges) {
    if (u.findSet() !== v.findSet()) {
      u.union(v);
      console.log(`Unioned ${u.key} and ${v.key}`);
      let set = new Set(vertices.map(s => s.set));
      for (let s of set) {
        console.log(s.show());
      }
    }
  }
}

function problem_21_2_2() {
  let x = [];
  for (let i = 0; i < 16; i++) {
    x[i] = new DSNode(i + 1);
  }
  for (let i = 0; i + 1 < 16; i += 2) {
    x[i].union(x[i + 1]);
  }
  for (let i = 0; i + 2 < 16; i += 4) {
    x[i].union(x[i + 2]);
  }

  x[0].union(x[4]);
  x[10].union(x[12]);
  x[0].union(x[9]);
  console.log("set of x2");
  console.log(x[1].set.show());
  console.log("set of x9");
  console.log(x[8].set.show());
}

function problem_21_3_1() {
  let x: Array<DSTreeNode<number>> = [];
  for (let i = 0; i < 16; i++) {
    x[i] = new DSTreeNode(i + 1);
  }
  for (let i = 0; i + 1 < 16; i += 2) {
    x[i].union(x[i + 1]);
  }
  for (let i = 0; i + 2 < 16; i += 4) {
    x[i].union(x[i + 2]);
  }

  x[0].union(x[4]);
  x[10].union(x[12]);
  x[0].union(x[9]);
  console.log("set of x2");
  console.log(x[1].show());
  console.log("set of x9");
  console.log(x[8].show());
}

function problem_21_1() {
  let input = "3 7 E 2 E 8 1 5 E E E 0 6 E 4";
  console.log(offlineMinimum(input, 9, 6));
}

main();
