import { randomAB } from "../util";
import { euclid, extendedEuclid, extendedGcd, gcd } from "./euclid";
import { longDivision } from "./long-division";
import { millerRabin } from "./miller-rabin";
import { crtInv, inverse, linearSolver, modExp } from "./modular";
import { nontrivalPower } from "./nontrival-power";
import { toDecimal } from "./to-decimal";

function main() {
  //
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

function problem_31_3_1() {
  let add_group: number[][] = [];
  let mul_group: number[][] = [];
  for (let i = 0; i < 4; i++) {
    add_group[i] = [];
    mul_group[i] = [];
    for (let j = 0; j < 4; j++) {
      add_group[i][j] = (i + j) % 4;
      mul_group[i][j] = ((i + 1) * (j + 1)) % 5;
    }
  }

  console.log(add_group.map(r => r.join(" ")).join("\n"));
  console.log("\n");
  console.log(mul_group.map(r => r.join(" ")).join("\n"));
}

function problem_31_3_2() {
  let z = 13;
  for (let i = 1; i <= 12; i++) {
    let sub: number[] = [1];
    let g = i;
    while (g !== 1) {
      sub.push(g);
      g = (g * i) % z;
    }
    console.log(i, sub);
  }
}

function problem_31_4_1() {
  let a = 35;
  let b = 10;
  let n = 50;
  let solutions = linearSolver(a, b, n);
  console.log(solutions);
  solutions.forEach(x => {
    console.assert((a * x) % n === b % n);
    console.log(`${a} * ${x} mod ${n} == ${(a * x) % n} == ${b % n}`);
  });
}

function problem_31_5_1() {
  let ns = [5, 11];
  let as = [4, 5];
  let a = crtInv(as, ns);
  let n = ns.reduce((l, r) => l * r);
  console.log(`${a} + k * ${n} for k ∈ Z`);
  for (let i = 0; i < ns.length; i++) {
    console.assert(a % ns[i] === as[i]);
  }
}

function problem_31_5_2() {
  let ns = [9, 8, 7];
  let as = [1, 2, 3];
  let a = crtInv(as, ns);
  let n = ns.reduce((l, r) => l * r);
  console.log(`${a} + k * ${n} for k ∈ Z`);
  for (let i = 0; i < ns.length; i++) {
    console.assert(a % ns[i] === as[i]);
  }
}

function problem_31_6_1() {
  let p = 11;
  let subs: number[][] = [];
  for (let i = 0; i < p - 1; i++) {
    subs[i] = [1];
    let basis = i + 1;
    let a = basis;
    while (a !== 1) {
      subs[i].push(a);
      a = (a * basis) % p;
    }
  }
  subs.forEach((sub, i) => {
    console.log(`ord(${i + 1}) = ${sub.length}:`, sub);
  });

  let g = subs.findIndex(sub => sub.length === p - 1) + 1;
  let generator = subs[g - 1];
  generator.forEach((x, i) => {
    console.log(`ind${p},${g}(${x}) = ${i}`);
  });
}

function problem_31_7_1() {
  let p = 11;
  let q = 29;
  let n = p * q;
  let e = 3;

  let d = inverse(e, (p - 1) * (q - 1));
  let M = 100;
  let C = modExp(M, e, n);
  console.log(`encryption of ${M} is ${C}`);
  console.assert(modExp(C, d, n) === M);
}

main();
