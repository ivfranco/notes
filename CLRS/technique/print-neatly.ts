export {
  printNeatly,
};

function printNeatly(w: string[], M: number): [number, string] {
  let l = w.map(word => word.length);
  let n = l.length;
  let ws: number[][] = [];
  for (let i = 0; i < n; i++) {
    ws[i] = [];
    let len = l[i];
    //  assume no word is longer than M
    ws[i][i] = M - len;
    for (let j = i + 1; j < n; j++) {
      len += l[j] + 1;
      ws[i][j] = M - len;
    }
  }

  let wc: number[][] = [];
  for (let i = 0; i < n; i++) {
    wc[i] = [];
    for (let j = i; j < n; j++) {
      let spaces = ws[i][j];
      if (spaces < 0) {
        wc[i][j] = +Infinity;
      } else if (j === n - 1) {
        wc[i][j] = 0;
      } else {
        wc[i][j] = ws[i][j] ** 3;
      }
    }
  }

  let cost: number[] = [];
  let r: number[] = [];

  function aux(j: number): number {
    if (cost[j] !== undefined) {
      return cost[j];
    }
    if (j < 0) {
      // base case if the paragraph contains no word
      return 0;
    }

    cost[j] = +Infinity;
    for (let k = 0; k <= j; k++) {
      let q = aux(k - 1) + wc[k][j];
      if (q < cost[j]) {
        cost[j] = q;
        r[j] = k;
      }
    }
    return cost[j];
  }

  aux(n - 1);
  return [cost[n - 1], constrParagraph(w, r)];
}

function constrParagraph(w: string[], r: number[]): string {
  let j = w.length - 1;
  let lines: string[] = [];
  while (j >= 0) {
    let i = r[j];
    lines.push(w.slice(i, j + 1).join(" "));
    j = i - 1;
  }

  return lines.reverse().join("\n");
}
