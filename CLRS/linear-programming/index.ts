import { WeightedGraph } from "../graph/weighted-graph";
import { toCompactFlowLinearProgram, toFlowLinearProgram, toSSLinearProgram } from "./graph-linear-program";
import { simplex, SlackForm } from "./simplex";

function main() {
  slackFormTest();
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

function slackFormTest() {
  let A = [
    [1, 1, 3],
    [2, 2, 5],
    [4, 1, 2],
  ];
  let b = [30, 24, 36];
  let c = [3, 1, 2];

  let slack = new SlackForm(A, b, c);
  console.log(slack.show());
  console.log("\nPivot x0 and x5");
  slack.pivot(0, 5);
  console.log(slack.show());
  console.log("\nPivot x2 and x4");
  slack.pivot(2, 4);
  console.log(slack.show());
  console.log("\nPivot x1 and x2");
  slack.pivot(1, 2);
  console.log(slack.show());
  slack.simplex();
  console.log("Solution to primal:");
  console.log(slack.basicSolution());
  console.log("Solution to dual:");
  console.log(slack.dualSolution());
}

function problem_29_3_5() {
  let A = [
    [1, 1],
    [1, 0],
    [0, 1],
  ];
  let b = [20, 12, 16];
  let c = [18, 12.5];

  console.log(simplex(A, b, c));
}

function problem_29_3_6() {
  let A = [
    [1, -1],
    [2, 1],
  ];
  let b = [1, 2];
  let c = [5, -3];

  console.log(simplex(A, b, c));
}

function problem_29_3_7() {
  let A = [
    [2, 20],
    [7.5, 5],
    [3, 10],
  ];
  let b = [1, 1, 1];
  let c = [10000, 30000];

  let slack = new SlackForm(A, b, c);
  slack.simplex();
  console.log("Final slack form:");
  console.log(slack.show());
  console.log("Dual solution:");
  console.log(slack.dualSolution());
}

function problem_29_5_5() {
  let A = [
    [1, -1],
    [-1, -1],
    [-1, 4],
  ];
  let b = [8, -3, 2];
  let c = [1, 3];

  console.log(simplex(A, b, c));
}

function problem_29_5_6() {
  let A = [
    [1, 2],
    [-2, -6],
    [0, 1],
  ];
  let b = [4, -12, 1];
  let c = [1, -2];

  console.log(simplex(A, b, c));
}

function problem_29_5_7() {
  let A = [
    [-1, 1],
    [-1, -1],
    [-1, 4],
  ];
  let b = [-1, -3, 2];
  let c = [1, 3];

  console.log(simplex(A, b, c));
}

function problem_29_5_8() {
  let A = [
    [2, -8, 0, -10],
    [-5, -2, 0, 0],
    [-3, 5, -10, 2],
  ];
  let b = [-50, -100, -25];
  let c = [-1, -1, -1, -1];

  console.log(simplex(A, b, c));
}

main();
