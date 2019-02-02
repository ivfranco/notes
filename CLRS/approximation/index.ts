import { NAryTree, treeVertexCover } from "./vertex-cover";
import { greedySetCover, linearSetCover, isCovered } from "./set-cover";

function main() {
  problem_35_3_3();
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

function problem_35_3_1() {
  let F: string[][] = [
    "arid",
    "dash",
    "drain",
    "heard",
    "lost",
    "nose",
    "shun",
    "slate",
    "snare",
    "thread",
  ].map(s => s.split(""));

  let X = Array.from(F.reduce((X, S) => {
    for (let s of S) {
      X.add(s);
    }
    return X;
  }, new Set<string>()));

  let C = greedySetCover(X, F);
  console.log(C.map(S => S.join("")));
  console.assert(isCovered(X, C), "Error: Set is not covered");
}

function problem_35_3_3() {
  let F: string[][] = [
    "arid",
    "dash",
    "drain",
    "heard",
    "lost",
    "nose",
    "shun",
    "slate",
    "snare",
    "thread",
  ].map(s => s.split(""));

  let X = Array.from(F.reduce((X, S) => {
    for (let s of S) {
      X.add(s);
    }
    return X;
  }, new Set<string>()));

  let C = linearSetCover(X, F, (c: string) => X.indexOf(c));
  console.log(C.map(S => S.join("")));
  console.assert(isCovered(X, C), "Error: Set is not covered")
}

main();