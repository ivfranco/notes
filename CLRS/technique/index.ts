import { isSorted, randomAB, shuffle } from "../util";
import { activitySelection, greedyActivitySelection } from "./activity-selection";
import { bitonicTour, constrBitonicTour } from "./bitonic-tour";
import { breakString } from "./break-string";
import { editDistance } from "./edit-distance";
import { huffman } from "./huffman";
import { fractionalKnapsack, knapsack, linearKnapsack } from "./knapsack";
import { greedyMatrixChain, matrixChainOrder, optimalParens } from "./matrix-chain";
import { constructOptimalBST, optimalBST, quadraticOptimalBST } from "./optimal-bst";
import { printNeatly } from "./print-neatly";
import { extendedBottomUpCutRod, extendedMemoizedCutRod } from "./rod";
import { constrSubstring, lcs, linearSpaceLcs, lis, memoizedLcs, quadraticLis } from "./substring";

function main() {
  problem_16_3_3();
}

function problem_15_1_4() {
  let prices = [0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
  let [b_p, b_cuts] = extendedBottomUpCutRod(prices, 4);
  let [m_p, m_cuts] = extendedMemoizedCutRod(prices, 4);
  console.log(b_p, b_cuts);
  console.log(m_p, m_cuts);
}

function problem_15_2_1() {
  let B = [5, 10, 3, 12, 5, 50, 6];
  let [m, s] = matrixChainOrder(B);
  console.log(m, optimalParens(s, 0, B.length - 2));
}

function problem_15_3_4() {
  // let A = [];
  // for (let i = 0; i < 4; i++) {
  //   A[i] = randomAB(1, 1000);
  // }
  let A = [30, 20, 10, 1];
  console.log(A);
  console.log(greedyMatrixChain(A));
  let [q, m] = matrixChainOrder(A);
  console.log(q, optimalParens(m, 0, A.length - 2));
}

function problem_15_4_1() {
  let X = "10010101".split("");
  let Y = "010110110".split("");

  console.log(constrSubstring(X, Y, lcs(X, Y)).join(""));
}

function problem_15_4_3() {

  let X = "ABCBDAB".split("");
  let Y = "BDCABA".split("");

  console.log(constrSubstring(X, Y, memoizedLcs(X, Y)).join(""));
}

function problem_15_4_4() {
  let X = "ABCBDAB".split("");
  let Y = "BDCABA".split("");

  console.log(linearSpaceLcs(X, Y));
  let c = lcs(X, Y);
  console.log(c);
  console.log(constrSubstring(X, Y, c).join(""));
}

function problem_15_4_6() {
  let A = [0, 1, 5, 8, 9, 10, 17, 20, 24, 30];
  shuffle(A);

  console.log(A);
  let L1 = lis(A);
  let L2 = quadraticLis(A);
  console.log(L1);
  console.log(L2);
  console.assert(isSorted(L1));
  console.assert(isSorted(L2));
  console.assert(L1.length === L2.length);
}

function problem_15_5_1() {
  let p = [0, 0.15, 0.10, 0.05, 0.10, 0.20];
  let q = [0.05, 0.10, 0.05, 0.05, 0.05, 0.10];

  let [e, root] = optimalBST(p, q, p.length - 1);
  console.log(constructOptimalBST(root, p.length - 1).show());
}

function problem_15_5_2() {
  let p = [0, 0.04, 0.06, 0.08, 0.02, 0.10, 0.12, 0.14];
  let q = [0.06, 0.06, 0.06, 0.06, 0.05, 0.05, 0.05, 0.05];

  let [e, root] = optimalBST(p, q, p.length - 1);
  console.log(e);
  console.log(constructOptimalBST(root, p.length - 1).show());
}

function problem_15_5_4() {
  let p = [0, 0.04, 0.06, 0.08, 0.02, 0.10, 0.12, 0.14];
  let q = [0.06, 0.06, 0.06, 0.06, 0.05, 0.05, 0.05, 0.05];

  let [e, root] = optimalBST(p, q, p.length - 1);
  console.log(e);
  console.log(constructOptimalBST(root, p.length - 1).show());
  [e, root] = quadraticOptimalBST(p, q, p.length - 1);
  console.log(e);
  console.log(constructOptimalBST(root, p.length - 1).show());
}

function problem_15_2() {
  let s = "character".split("");
  let r = s.slice().reverse();
  console.log(s);

  console.log(constrSubstring(s, r, lcs(s, r)).join(""));
}

function problem_15_3() {
  let p = [
    [0, 0],
    [2, 3],
    [5, 2],
    [7, 1],
    [8, 4],
    [6, 5],
    [1, 6],
  ].map(([x, y]) => {
    return { x, y };
  });

  let [b, r] = bitonicTour(p);
  console.log(b);
  console.log(constrBitonicTour(r, p.length));
}

function problem_15_4() {
  let words = `Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Quisque porttitor ipsum ut sollicitudin dapibus. \
Praesent luctus nibh sit amet orci facilisis finibus. \
Maecenas vitae ante sit amet augue aliquet iaculis. \
Nunc imperdiet diam quis ex condimentum, vel semper nisl lobortis. \
Fusce vel dignissim ante. Duis maximus faucibus pulvinar. \
Integer fermentum tortor in maximus ultrices`
    .split(/[.,]?\s/);
  let [cost, para] = printNeatly(words, 20);
  console.log(cost);
  console.log(para);
}

function problem_15_5() {
  let x = "algorithm";
  let y = "altruistic";

  let cost = {
    copy: 1,
    delete: 2,
    insert: 2,
    kill: 5,
    replace: 2,
    twiddle: 3,
  };

  let [distance, ops] = editDistance(x, y, cost);
  console.log(distance);
  console.log(ops);
}

function problem_15_9() {
  let bp = [1, 7, 9];
  let n = 20;

  console.log(breakString(bp, n));
}

function problem_16_1_1() {
  let A = [
    [1, 4],
    [3, 5],
    [0, 6],
    [5, 7],
    [3, 9],
    [5, 9],
    [6, 10],
    [8, 11],
    [8, 12],
    [2, 14],
    [12, 16],
  ].map(([s, f]) => {
    return { s, f };
  });

  console.log(activitySelection(A)[1]);
  console.log(greedyActivitySelection(A));
}

function problem_16_2_2() {
  let I = [
    [10, 60],
    [20, 100],
    [30, 120],
  ].map(([w, v]) => {
    return { w, v };
  });
  let W = 50;

  console.log("Fractional solution");
  console.log(fractionalKnapsack(I, W));
  console.log("\n0-1 solution");
  console.log(knapsack(I, W));
  console.log("\nLinear fractional solution");
  console.log(linearKnapsack(I, W));
}

function problem_16_3_3() {
  let C = [
    [1, "a"],
    [1, "b"],
    [2, "c"],
    [3, "d"],
    [5, "e"],
    [8, "f"],
    [13, "g"],
    [21, "h"],
  ] as Array<[number, string]>;

  console.log(huffman(C).show());
}

main();
