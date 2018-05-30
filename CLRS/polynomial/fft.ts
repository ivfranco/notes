export {
  recursiveFFT,
  recursiveFFTInv,
  fastMultiply,
};

import { Complex, unityRoot } from "./complex";
import { Coff, Polynomial } from "./polynomial";

function gRecursiveFFT(a: Complex[], inverse: boolean): Complex[] {
  let n = a.length;
  if (n === 1) {
    return a;
  }

  let w = new Complex(1, 0);
  let root = unityRoot(n);
  if (inverse) {
    root = root.inverse();
  }

  let a_odds: Complex[] = [];
  let a_evens: Complex[] = [];
  for (let i = 0; i < n / 2; i++) {
    a_evens.push(a[2 * i]);
    a_odds.push(a[2 * i + 1]);
  }
  let y_odds = gRecursiveFFT(a_odds, inverse);
  let y_evens = gRecursiveFFT(a_evens, inverse);
  let y: Complex[] = [];
  for (let i = 0; i < n / 2; i++) {
    let twiddle = w.mul(y_odds[i]);
    y[i] = y_evens[i].add(twiddle);
    y[i + n / 2] = y_evens[i].sub(twiddle);
    w = w.mul(root);
  }

  return y;
}

function recursiveFFT(a: Coff[]): Complex[] {
  return gRecursiveFFT(a.map(r => new Complex(r, 0)), false);
}

function recursiveFFTInv(y: Complex[]): Coff[] {
  let n = y.length;
  return gRecursiveFFT(y, true).map(z => z.real / n);
}

function fastMultiply(pa: Polynomial, pb: Polynomial): Polynomial {
  let a = pa.coffs;
  let b = pb.coffs;

  let next_power = 2 ** Math.ceil(Math.log2(Math.max(a.length, b.length)));
  while (a.length < 2 * next_power) {
    a.push(0);
  }
  while (b.length < 2 * next_power) {
    b.push(0);
  }

  let ya = recursiveFFT(a);
  let yb = recursiveFFT(b);
  let y = ya.map((z, i) => z.mul(yb[i]));
  let c = recursiveFFTInv(y);
  while (c.length > 0 && c[c.length - 1] === 0) {
    c.pop();
  }

  return new Polynomial(c);
}
