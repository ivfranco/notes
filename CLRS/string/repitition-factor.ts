import { computePrefixFunction } from "./kmp";

function repitition(P: string): number[] {
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
