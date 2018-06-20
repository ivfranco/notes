export {
  repetitionMatcher,
};

import { computePrefixFunction } from "./kmp";

function repetition(P: string): number[] {
  let m = P.length;
  let rho: number[] = [];
  rho[1] = 1;
  let prefix = computePrefixFunction(P);

  for (let i = 2; i <= m; i++) {
    rho[i] = 1;
    for (let j = prefix[i]; j > 0; j = prefix[j]) {
      let r = i / j;
      if (Number.isInteger(r) && rho[i - j] === r - 1) {
        rho[i] = Math.max(r, rho[i]);
      }
    }
  }

  return rho;
}

function repetitionMatcher(T: string, P: string): number[] {
  let m = P.length;
  let n = T.length;
  let k = repetition(P).reduce((a, b) => Math.max(a, b));
  let q = 0;
  let s = 0;

  let shifts: number[] = [];
  while (s <= n - m) {
    if (T[s + q] === P[q]) {
      q++;
      if (q === m) {
        shifts.push(s);
      }
    }
    if (q === m || T[s + q] !== P[q]) {
      s = s + Math.max(1, Math.ceil(q / k));
      q = 0;
    }
  }

  return shifts;
}
