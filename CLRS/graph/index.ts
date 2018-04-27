import { bfs, PlainGraph, Vertex } from "./directed-graph";

function main() {
  problem_22_2_2();
}

function problem_22_2_1() {
  let G = new PlainGraph();
  let V: Vertex[] = [];

  for (let i = 1; i <= 6; i++) {
    V[i] = G.createVertex("" + i);
  }

  [
    [1, 2],
    [1, 4],
    [2, 5],
    [3, 5],
    [3, 6],
    [4, 2],
    [5, 4],
    [6, 6],
  ].forEach(([i, j]) => G.createEdge(V[i], V[j]));

  let [d, p] = bfs(G, V[3]);
  for (let i = 1; i <= 6; i++) {
    let parent = p[V[i].key];
    let distance = d[V[i].key];
    console.log(`${V[i].name}.d = ${distance}, ${V[i].name}.π = ${parent ? parent.name : "NIL"}`);
  }
}

function problem_22_2_2() {
  let G = new PlainGraph();
  let V: { [index: string]: Vertex } = {};

  "r s t u v w x y"
    .split(" ")
    .map(name => G.createVertex(name))
    .forEach(v => V[v.name] = v);

  [
    "r v",
    "r s",
    "s w",
    "w t",
    "w x",
    "t x",
    "t u",
    "u x",
    "x y",
    "u y",
  ].forEach(pair => {
    let [u, v] = pair.split(" ");
    G.createEdge(V[u], V[v]);
    G.createEdge(V[v], V[u]);
  });

  let [d, p] = bfs(G, V["u"]);
  for (let name of Object.keys(V)) {
    let v = V[name];
    let dist = d[v.key];
    let parent = p[v.key];
    console.log(`${name}.d = ${dist}, ${name}.π = ${parent ? parent.name : "NIL"}`);
  }
}

main();
