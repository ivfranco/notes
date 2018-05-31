import { Complex, unityRoot } from "../polynomial/complex";
import { parallelFor } from "./p-matrix-multiply";

async function pFFT(a: Complex[]): Promise<Complex[]> {
  let n = a.length;
  if (n === 1) {
    return a;
  }

  let w = unityRoot(n);
  let a_odds: Complex[] = [];
  let a_evens: Complex[] = [];
  await parallelFor(0, n / 2 - 1, async i => {
    a_evens[i] = a[2 * i];
    a_odds[i] = a[2 * i + 1];
  });
  let handle = pFFT(a_odds);
  let y_evens = await pFFT(a_evens);
  let y_odds = await handle;
  let y: Complex[] = [];
  await parallelFor(0, n / 2 - 1, async i => {
    let twiddle = w.pow(i).mul(y_odds[i]);
    y[i] = y_evens[i].add(twiddle);
    y[i + n / 2] = y_evens[i].sub(twiddle);
  });

  return y;
}
