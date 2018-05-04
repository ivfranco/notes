import {
  alterTopologicalSort,
  bfs,
  dfs,
  DFS,
  dfsReport,
  numberOfPaths,
  PlainGraph,
  scc,
  singlyConnected,
  topologicalSort,
  Vertex,
} from "./directed-graph";
import { spBellmanFord, spDag, spDijkstra, spReport } from "./shortest-path";
import { mstKruskal, mstPrim, showWeighted, WeightedGraph } from "./weighted-graph";

function main() {
  problem_24_3_1();
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

function problem_22_3_2() {
  let G = PlainGraph.fromDirected(
    "q r s t u v w x y z",
    [
      "q s", "q t", "q w",
      "r u", "r y",
      "s v",
      "t x", "t y",
      "u y",
      "v w",
      "w s",
      "x z",
      "y q",
      "z x",
    ],
  );

  let [v_attr, e_attr] = dfs(G);
  dfsReport(G, v_attr, e_attr);
}

function problem_22_3_3() {
  let G = PlainGraph.fromDirected(
    "u v w x y z",
    [
      "u v", "u x",
      "v y",
      "w y", "w z",
      "x v",
      "y x",
      "z z",
    ],
  );

  let [v_attr, e_attr] = dfs(G);
  dfsReport(G, v_attr, e_attr);
}

function problem_22_3_13() {
  let G1 = PlainGraph.fromDirected(
    "u v w x y z",
    [
      "u v", "u x",
      "v y",
      "w y", "w z",
      "x v",
      "y x",
      "z z",
    ],
  );

  let G2 = PlainGraph.fromDirected(
    "u v w x y z",
    [
      "u v",
      "v y",
      "w y", "w z",
      "x v",
      "y x",
      "z z",
    ],
  );

  console.log(singlyConnected(G1));
  console.log(singlyConnected(G2));
}

function problem_22_4_1() {
  let G = PlainGraph.fromDirected(
    "m n o p q r s t u v w x y z",
    [
      "m q", "m r", "m x",
      "n o", "n q", "n u",
      "o r", "o s", "o v",
      "p o", "p s", "p z",
      "q t",
      "r u", "r y",
      "s r",
      "u t",
      "v w", "v x",
      "w z",
      "y v",
    ],
  );

  console.log(topologicalSort(G).map(v => v.name).join(" -> "));
  //  problem_22_4_2
  let V = G.vertexMap();
  console.log(numberOfPaths(G, V["p"], V["v"]));
  //  problem_22_4_5
  console.log(alterTopologicalSort(G).map(v => v.name).join(" -> "));
}

function problem_22_5_2() {
  let G = PlainGraph.fromDirected(
    "q r s t u v w x y z",
    [
      "q s", "q t", "q w",
      "r u", "r y",
      "s v",
      "t x", "t y",
      "u y",
      "v w",
      "w s",
      "x z",
      "y q",
      "z x",
    ],
  );

  scc(G);
}

function mstTests() {
  let G = WeightedGraph.fromUndirected(
    "a b c d e f g h i",
    [
      "a b 4", "a h 8",
      "b c 8", "b h 11",
      "c d 7", "c f 4", "c i 2",
      "d e 9", "d f 14",
      "e f 10",
      "f g 2",
      "g i 6", "g h 1",
      "h i 7",
    ],
  );

  let [p, A] = mstKruskal(G);
  console.log(p);
  console.log(A.map(showWeighted));

  let a = G.vertexMap()["a"];
  [p, A] = mstPrim(G, a);
  console.log(p);
  console.log(A.map(showWeighted));
}

function problem_24_1_1() {
  let G = WeightedGraph.fromDirected(
    "s t x y z",
    [
      "t x 5", "t y 8", "t z -4",
      "x t -2",
      "y x -3", "y z 9",
      "z x 7", "z s 2",
      "s t 6", "s y 7",
    ],
  );
  let s = G.vertexMap()["s"];
  console.log("w(z, x) = 7");
  spReport(G, spBellmanFord(G, s));

  let H = WeightedGraph.fromDirected(
    "s t x y z",
    [
      "t x 5", "t y 8", "t z -4",
      "x t -2",
      "y x -3", "y z 9",
      "z x 4", "z s 2",
      "s t 6", "s y 7",
    ],
  );
  s = H.vertexMap()["s"];
  console.log("\nw(z, x) = 4");
  spReport(G, spBellmanFord(H, s));
}

function problem_24_2_1() {
  let G = WeightedGraph.fromDirected(
    "r s t x y z",
    [
      "r s 5", "r t 3",
      "s t 2", "s x 6",
      "t x 7", "t y 4", "t z 2",
      "x y -1", "x z 1",
      "y z -2",
    ],
  );
  let s = G.vertexMap()["s"];

  spReport(G, spDag(G, s));
}

function problem_24_3_1() {
  let G = WeightedGraph.fromDirected(
    "s t x y z",
    [
      "s t 3", "s y 5",
      "t x 6", "t y 2",
      "x z 11",
      "y t 3", "y x 4", "y z 6",
      "z s 3", "z x 7",
    ],
  );
  let map = G.vertexMap();
  let s = map["s"];
  let z = map["z"];

  console.log("Starting from vertex s");
  spReport(G, spDijkstra(G, s));
  console.log("Starting from vertex s");
  spReport(G, spDijkstra(G, z));
}

main();
