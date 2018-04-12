import { isSorted, randomAB, shuffle } from "../util";
import { greedyMatrixChain, matrixChainOrder, optimalParens } from "./matrix-chain";
import { extendedBottomUpCutRod, extendedMemoizedCutRod } from "./rod";
import { constrSubstring, lcs, linearSpaceLcs, lis, memoizedLcs, quadraticLis } from "./substring";

function main() {
  problem_15_4_6();
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

main();
