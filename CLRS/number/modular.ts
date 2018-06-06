export {
  linearSolver,
  mod,
  crt,
  crtInv,
  modExp,
};

import { extendedEuclid } from "./euclid";

//  the result of a % n may be negative in javascript
function mod(a: number, n: number): number {
  return (a % n + n) % n;
}

/* tslint:disable:no-bitwise */
function modExp(a: number, e: number, n: number): number {
  console.assert(e >= 0 && Math.floor(e) === e, "exponential must be a nonnegative integer");

  let pow = 1;
  while (e !== 0) {
    if ((e & 0x1) === 1) {
      pow = mod(pow * a, n);
    }
    a = mod(a * a, n);
    e >>= 1;
  }

  return pow;
}
/* tslint:enable:no-bitwise */

//  solve the equation ax ≡ b mod n, return all distinct solutions modulo n
function linearSolver(a: number, b: number, n: number): number[] {
  let [d, x, y] = extendedEuclid(a, n);
  let solutions: number[] = [];
  if (b % d === 0) {
    let x0 = mod(x * b / d, n);
    for (let i = 0; i < d; i++) {
      solutions.push(x0);
      x0 = mod(x0 + (n / d), n);
    }
  }

  return solutions;
}

function inverse(a: number, n: number): number {
  let solutions = linearSolver(a, 1, n);
  if (solutions.length === 0) {
    throw new Error(`Error: gcd(${a}, ${n}) != 1, inverse of a in Z${n} doesn't exist`);
  } else {
    return solutions[0];
  }
}

function crt(a: number, ns: number[]): number[] {
  return ns.map(n => mod(a, n));
}

function crtInv(as: number[], ns: number[]): number {
  let n = ns.reduce((l, r) => l * r);
  let cs = ns.map(ni => {
    let mi = n / ni;
    return mi * inverse(mi, ni);
  });
  return as.reduce((sum, ai, i) => mod(sum + ai * cs[i], n), 0);
}
