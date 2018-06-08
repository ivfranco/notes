export {
  millerRabin,
};

import { randomAB } from "../util";
import { modExp } from "./modular";

/* tslint:disable:no-bitwise */

//  returns true if a is a witness of the compositeness of n, i.e. n is composite
function witness(a: number, n: number): boolean {
  console.assert(n >= 1 && Math.floor(n) === n && (n & 0x1) === 1, "n must be a positive odd integer");

  let t = 0;
  let u = n - 1;
  while ((u & 0x1) === 0) {
    u >>= 1;
    t++;
  }

  let x0 = modExp(a, u, n);
  for (let i = 0; i < t; i++) {
    let x1 = modExp(x0, 2, n);
    if (x1 === 1 && x0 !== 1 && x0 !== n - 1) {
      return true;
    }
    x0 = x1;
  }
  if (x0 !== 1) {
    return true;
  } else {
    return false;
  }
}

const RETRY = 50;

//  return false if n is composite, return true if n is very likely prime
function millerRabin(n: number): boolean {
  console.assert(n >= 1 && Math.floor(n) === n, "n must be a positive integer");
  if ((n & 0x1) === 0) {
    return n === 2;
  }
  for (let j = 0; j < RETRY; j++) {
    let a = randomAB(1, n - 1);
    if (witness(a, n)) {
      return false;
    }
  }
  return true;
}
