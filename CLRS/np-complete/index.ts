import { subsetSum } from "./subset-sum"

function main() {
  problem_34_5_4();
}

function problem_34_5_4() {
  console.assert(subsetSum([7, 3, 2, 5, 8], 17), "sum 17");
  console.assert(subsetSum([7, 3, 2, 5, 8], 16), "sum 16");
  console.assert(subsetSum([7, 3, 2, 5, 8], 15), "sum 15");
  console.assert(subsetSum([7, 3, 2, 5, 8], 14), "sum 14");
  console.assert(subsetSum([7, 3, 2, 5, 8], 13), "sum 13");
  console.assert(subsetSum([7, 3, 2, 5, 8], 12), "sum 12");
  console.assert(subsetSum([7, 3, 2, 5, 8], 11), "sum 11");
  console.assert(subsetSum([7, 3, 2, 5, 8], 10), "sum 10");
  console.assert(subsetSum([7, 3, 2, 5, 8], 9), "sum 9");
  console.assert(subsetSum([7, 3, 2, 5, 8], 8), "sum 8");
  console.assert(subsetSum([7, 3, 2, 5, 8], 7), "sum 7");
  console.assert(!subsetSum([7, 3, 2, 5, 8], 6), "sum 6");
}

main();