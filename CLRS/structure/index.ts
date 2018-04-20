import { BTree, BTreeNode } from "./b-tree";

function main() {
  problem_18_3_1();
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

main();
