import { longDivision } from "./long-division";
import { nontrivalPower } from "./nontrival-power";

function main() {
  problem_31_1_12();
}

function problem_31_1_8() {
  let testCase = [
    4096,
    4097,
  ];
  testCase.forEach(n => {
    let res = nontrivalPower(n);
    if (res) {
      let [a, k] = res;
      console.log(`${a}^${k} == ${n}`);
      console.assert(a ** k === n);
    } else {
      console.log(`${n} is not a nontrival power`);
    }
  });
}

function problem_31_1_12() {
  let testCase = [
    [4092, 2],
    [785, 3],
  ];
  testCase.forEach(([a, b]) => {
    let [q, r] = longDivision(a, b);
    console.log(`${a} = ${q} * ${b} + ${r}`);
    console.assert(a === q * b + r, `${a} != ${q} * ${b} + ${r}`);
    console.assert(0 <= r && r < b, "Remainder cannot be negative or greater than divisor");
  });
}

main();
