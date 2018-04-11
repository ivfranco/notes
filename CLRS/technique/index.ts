import { matrixChainOrder, optimalParens } from "./matrix-chain";
import { extendedBottomUpCutRod, extendedMemoizedCutRod } from "./rod";

function main() {
  problem_15_2_1();
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

main();
