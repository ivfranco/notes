export {
  factorize,
};

import { randomAB } from "../util";
import { euclid } from "./euclid";
import { millerRabin } from "./miller-rabin";
import { mod } from "./modular";

//  only handles n <= 9.5e7 as x^2 must be accurate, x^2 <= Number.MAX_SAFE_INTEGER
function pollardRho(n: number): number {
  let i = 1;
  let x = randomAB(0, n - 1);
  let y = x;
  let k = 2;
  while (true) {
    i++;
    x = mod(x * x - 1, n);
    let d = euclid(Math.abs(y - x), n);
    if (d !== 1 && d !== n) {
      return d;
    } else if (d === n) {
      //  y = x mod n, a loop
      //  restart with a new random x
      x = randomAB(0, n - 1);
      y = x;
      k = 2;
    }
    if (i === k) {
      y = x;
      k *= 2;
    }
  }
}

function factorize(n: number): number[] {
  let factors: number[] = [];
  while (n >= 2 && !millerRabin(n)) {
    let p = n;
    while (!millerRabin(p)) {
      p = pollardRho(p);
    }
    while (n % p === 0) {
      factors.push(p);
      n /= p;
    }
  }
  if (n > 1) {
    factors.push(n);
  }
  return factors;
}
