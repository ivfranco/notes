export {
  breakString,
};

function breakString(bp: number[], n: number): [number, number[][]] {
  bp = bp.slice();
  bp.push(0, n - 1);
  bp.sort((a, b) => a - b);
  let m = bp.length;

  let costs: number[][] = [];
  let breaks: number[][] = [];
  for (let i = 0; i < m - 1; i++) {
    costs[i] = [];
    costs[i][i + 1] = 0;
    breaks[i] = [];
  }

  function aux(i: number, j: number): number {
    if (costs[i][j] !== undefined) {
      return costs[i][j];
    }

    costs[i][j] = +Infinity;
    for (let k = i + 1; k < j; k++) {
      let q = aux(i, k) + aux(k, j) + bp[j] - bp[i] + 1;
      if (q < costs[i][j]) {
        costs[i][j] = q;
        breaks[i][j] = k;
      }
    }

    return costs[i][j];
  }

  console.log(bp);
  aux(0, m - 1);
  return [costs[0][m - 1], breaks];
}
