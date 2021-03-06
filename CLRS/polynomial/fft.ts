export {
  recursiveFFT,
  recursiveFFTInv,
  fastMultiply,
  fastIterMultiply,
  iterativeFFT,
};

import { BitReversedCounter } from "../technique/bit-reverse";
import { Complex, unityRoot } from "./complex";
import { Coeff, Polynomial } from "./polynomial";

let EPSILON = 1e-6;

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

function recursiveFFT(a: Coeff[]): Complex[] {
  return gRecursiveFFT(a.map(r => new Complex(r, 0)), false);
}

function recursiveFFTInv(y: Complex[]): Coeff[] {
  let n = y.length;
  return gRecursiveFFT(y, true).map(z => z.real / n);
}

function recursiveConvolution(a: Coeff[], b: Coeff[]): Coeff[] {
  let ya = recursiveFFT(a);
  let yb = recursiveFFT(b);
  let y = ya.map((z, i) => z.mul(yb[i]));
  return recursiveFFTInv(y);
}

function fastMultiply(pa: Polynomial, pb: Polynomial, conv = recursiveConvolution): Polynomial {
  let a = pa.coffs.slice();
  let b = pb.coffs.slice();

  let next_power = 2 ** Math.ceil(Math.log2(Math.max(a.length, b.length)));
  while (a.length < 2 * next_power) {
    a.push(0);
  }
  while (b.length < 2 * next_power) {
    b.push(0);
  }

  let c = conv(a, b);
  while (c.length > 0 && Math.abs(c[c.length - 1]) <= EPSILON) {
    c.pop();
  }

  return new Polynomial(c);
}

function bitReverseCopy<T>(a: T[]): T[] {
  let A: T[] = [];
  let n = a.length;
  let counter = new BitReversedCounter(0, (n - 1).toString(2).length);
  for (let i = 0; i < n; i++) {
    A[counter.get()] = a[i];
    counter.increment();
  }
  return A;
}

function gIterativeFFT(a: Complex[], inverse: boolean): Complex[] {
  let n = a.length;
  let A = bitReverseCopy(a);

  for (let m = 2; m <= n; m *= 2) {
    let root = unityRoot(m);
    if (inverse) {
      root = root.inverse();
    }
    let w: Complex[] = [];
    w[0] = new Complex(1, 0);
    for (let i = 1; i < m / 2; i++) {
      w[i] = w[i - 1].mul(root);
    }
    for (let k = 0; k < n; k += m) {
      for (let j = 0; j < m / 2; j++) {
        butterfly(A, w[j], k + j, k + j + m / 2);
      }
    }
  }

  return A;
}

function butterfly(A: Complex[], w: Complex, i: number, j: number) {
  let t = w.mul(A[j]);
  let u = A[i];
  A[i] = u.add(t);
  A[j] = u.sub(t);
}

function inverseButterfly(A: Complex[], w: Complex, i: number, j: number) {
  let t = A[i].sub(A[j]).div(2);
  A[i] = A[i].sub(t);
  A[j] = t.div(w);
}

function gIterativeFFT2(a: Complex[], inverse: boolean): Complex[] {
  let n = a.length;
  let A = a.map(z => z.mul(n));
  for (let m = n; m >= 2; m /= 2) {
    let root = unityRoot(m);
    if (!inverse) {
      root = root.inverse();
    }
    for (let k = 0; k < n; k += m) {
      let w = new Complex(1, 0);
      for (let j = 0; j < m / 2; j++) {
        inverseButterfly(A, w, k + j, k + j + m / 2);
        w = w.mul(root);
      }
    }
  }

  return bitReverseCopy(A);
}

function iterativeFFT(a: Coeff[]): Complex[] {
  return gIterativeFFT(a.map(r => new Complex(r, 0)), false);
}

function iterativeFFTInv(y: Complex[]): Coeff[] {
  let n = y.length;
  return gIterativeFFT(y, true).map(z => z.real / n);
}

function iterativeConvolution(a: Coeff[], b: Coeff[]): Coeff[] {
  let ya = iterativeFFT(a);
  let yb = iterativeFFT(b);
  let y = ya.map((z, i) => z.mul(yb[i]));
  return iterativeFFTInv(y);
}

function fastIterMultiply(pa: Polynomial, pb: Polynomial): Polynomial {
  return fastMultiply(pa, pb, iterativeConvolution);
}
