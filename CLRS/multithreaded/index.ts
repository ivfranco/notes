import { matrixMultiplication } from "../start/matrix-mul";
import { pMatrixMultiplyDivide, pSquareMatrixMultiply, pStrassen } from "./p-matrix-multiply";

async function main() {
  await matrixTest();
}

async function matrixTest() {
  let A = [
    [1, 3],
    [7, 5],
  ];
  let B = [
    [6, 8],
    [4, 2],
  ];

  console.log("Serial implementation:");
  console.log(matrixMultiplication(A, B));
  console.log("P-SQUARE-MATRIX-MULTIPLY:");
  let C = await pSquareMatrixMultiply(A, B);
  console.log(C);
  console.log("P-MATRIX-MULTIPLY-RECURSIVE:");
  C = await pMatrixMultiplyDivide(A, B);
  console.log(C);
  console.log("P-STRASSEN-RECURSIVE:");
  C = await pStrassen(A, B);
  console.log(C);
}

main();
