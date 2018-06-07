import { randomAB } from "../util";
import { euclid } from "./euclid";
import { mod } from "./modular";

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
    }
    if (i === k) {
      y = x;
      k *= 2;
    }
  }
}
