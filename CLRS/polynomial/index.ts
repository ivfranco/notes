import { modExp } from "../number/modular";
import { modularFFT, simpleModularDFT } from "../number/modular-fft";
import {
  fastIterMultiply,
  fastMultiply,
  iterativeFFT,
  recursiveFFT,
  recursiveFFTInv,
} from "./fft";
import { coffMultiply, interpolate, Polynomial } from "./polynomial";

function main() {
  problem_30_6();
}

function problem_30_1_1() {
  let A = new Polynomial([-10, 1, -1, 7]);
  let B = new Polynomial([3, -6, 0, 8]);

  console.log(coffMultiply(A, B).show());
}

function problem_30_1_5() {
  let p = new Polynomial([1, 2, 3, 4, 5]);
  let pts = [0, 1, 2, 3, 4].map(x => {
    return { x, y: p.evaluate(x) };
  });

  console.log(interpolate(pts).show());
}

function problem_30_2_2() {
  let a = [0, 1, 2, 3];
  let y = recursiveFFT(a);
  console.log(y.map(z => z.show()));
  console.log(recursiveFFTInv(y));
}

function problem_30_2_3() {
  let A = new Polynomial([-10, 1, -1, 7]);
  let B = new Polynomial([3, -6, 0, 8]);

  console.log(fastMultiply(A, B).show());
  console.log(fastIterMultiply(A, B).show());
}

function problem_30_3_1() {
  let A = [0, 2, 3, -1, 4, 5, 7, 9];
  iterativeFFT(A);
}

function problem_30_6() {
  let a = [0, 5, 3, 7, 7, 2, 1, 6];
  let p = 17;
  let k = (p - 1) / a.length;
  let g = 3;
  let w = modExp(g, k, p);

  let fft = modularFFT(a, p, w);
  let simple = simpleModularDFT(a, p, w);
  console.log(fft);
  console.log(simple);
  for (let i = 0; i < a.length; i++) {
    console.assert(fft[i] === simple[i]);
  }
}

main();
