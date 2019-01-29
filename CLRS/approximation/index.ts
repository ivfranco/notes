import { NAryTree, treeVertexCover } from "./vertex-cover";

function main() {
  problem_34_1_4();
}

function problem_34_1_4() {
  let tree = new NAryTree("a", null, []);
  let b = tree.createChild("b");
  let c = tree.createChild("c");
  b.createChild("d");
  let e = b.createChild("e");
  c.createChild("f");
  e.createChild("g");
  e.createChild("h");

  let cover = treeVertexCover(tree);
  console.assert(cover.length === 3 &&
    cover.indexOf("b") >= 0 &&
    cover.indexOf("c") >= 0 &&
    cover.indexOf("e") >= 0, "Error: the vertex cover is not minimal");
}

main();