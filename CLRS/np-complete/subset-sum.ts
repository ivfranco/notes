export function subsetSum(S: number[], t: number): boolean {
  let n = S.length;
  let memo: boolean[][] = [];

  for (let i = 0; i <= n; i++) {
    memo[i] = [];
  }

  function aux(i: number, t: number): boolean {
    if (memo[i][t] !== undefined) {
      return memo[i][t];
    }

    let ret;

    if (t === 0) {
      ret = true;
    } else if (t < 0) {
      ret = false;
    } else if (i === n) {
      ret = t === 0;
    } else {
      let s0 = S[i];
      ret = aux(i + 1, t) || aux(i + 1, t - s0);
    }

    memo[i][t] = ret;
    return ret;
  }

  return aux(0, t);
}