import { randomAB } from "../util";
import { euclid, extendedEuclid, extendedGcd, gcd } from "./euclid";
import { longDivision } from "./long-division";
import { nontrivalPower } from "./nontrival-power";
import { toDecimal } from "./to-decimal";

function main() {
  problem_31_2_7();
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
      console.assert(a ** k === n);
      console.log(`${a}^${k} == ${n}`);
    } else {
      console.log(`${n} is not a nontrival power`);
    }
  });
}

function problem_31_1_12() {
  let testCase = [
    [4092, 2],
    [785, 3],
    [-785, 3],
    [4092, -2],
  ];
  testCase.forEach(([a, b]) => {
    let [q, r] = longDivision(a, b);
    console.assert(a === q * b + r, `${a} != ${q} * ${b} + ${r}`);
    console.log(`${a} = ${q} * ${b} + ${r}`);
    console.assert(0 <= r && r < Math.abs(b), "Remainder cannot be negative or greater than absolute divisor");
  });
}

function problem_31_1_13() {
  let testCase = [];
  for (let i = 0; i < 10; i++) {
    testCase.push(randomAB(0, 1000000000));
  }
  testCase.forEach(a => {
    let rep = toDecimal(a);
    console.assert(rep === a.toString(10), `${a} != ${rep}`);
    console.log(`${a} = ${rep}`);
  });
}

function problem_31_2_2() {
  let a = 899;
  let b = 493;
  let [d, x, y] = extendedEuclid(a, b);
  console.assert(a * x + b * y === d);
  console.log(d, x, y);
}

function problem_31_2_6() {
  let a = 1;
  let b = 0;
  for (let i = 0; i < 10; i++) {
    console.log(i, extendedEuclid(a, b));
    a += b;
    b = a - b;
  }
}

function problem_31_2_7() {
  let testCase: number[] = [];
  let n = randomAB(2, 10);
  for (let i = 0; i < n; i++) {
    testCase.push(randomAB(1, 10000));
  }
  let [d, x] = extendedGcd(...testCase);
  console.log(testCase);
  console.log(d, x);
  console.assert(d === gcd(...testCase));
  console.assert(testCase.map((a, i) => a * x[i]).reduce((a, b) => a + b) === d);
}

main();
