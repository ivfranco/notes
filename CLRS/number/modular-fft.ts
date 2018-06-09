export {
  modularFFT,
  simpleModularDFT,
};

import { Polynomial } from "../polynomial/polynomial";
import { mod, modExp } from "./modular";

function modularFFT(a: number[], p: number, root: number): number[] {
  let n = a.length;
  if (n === 1) {
    return a;
  }
  let a_odds: number[] = [];
  let a_evens: number[] = [];
  for (let i = 0; i < n / 2; i++) {
    a_evens.push(a[2 * i]);
    a_odds.push(a[2 * i + 1]);
  }
  let sroot = modExp(root, 2, p);
  let y_odds = modularFFT(a_odds, p, sroot);
  let y_evens = modularFFT(a_evens, p, sroot);
  let y: number[] = [];
  for (let i = 0, w = 1; i < n / 2; i++) {
    y[i] = mod(y_evens[i] + w * y_odds[i], p);
    y[i + n / 2] = mod(y_evens[i] - w * y_odds[i], p);
    w = mod(w * root, p);
  }

  return y;
}

function simpleModularDFT(a: number[], p: number, root: number): number[] {
  let n = a.length;
  let poly = new Polynomial(a);
  let y: number[] = [];
  for (let i = 0; i < n; i++) {
    y[i] = mod(poly.evaluate(modExp(root, i, p)), p);
  }
  return y;
}
