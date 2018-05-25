import { WeightedGraph } from "../graph/weighted-graph";
import { toCompactFlowLinearProgram, toFlowLinearProgram, toSSLinearProgram } from "./graph-linear-program";

function main() {
  problem_29_2_4();
}

function problem_29_2_2() {
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
  let V = G.vertexMap();
  let s = V["s"];
  let y = V["y"];

  console.log(toSSLinearProgram(G, s, y));
}

function problem_29_2_4() {
  let G = WeightedGraph.fromDirected(
    "s v1 v2 v3 v4 t",
    [
      "s v1 16", "s v2 13",
      "v1 v3 12",
      "v2 v1 4", "v2 v4 14",
      "v3 v2 9", "v3 t 20",
      "v4 v3 7", "v4 t 4",
    ],
  );
  let V = G.vertexMap();
  let s = V["s"];
  let t = V["t"];

  console.log(toFlowLinearProgram(G, s, t));
  //  problem_29_2_5
  console.log("\nCompact form: ");
  console.log(toCompactFlowLinearProgram(G, s, t));
}

main();
