export {
  extendedBottomUpCutRod,
  extendedMemoizedCutRod,
};

function constrCuts(s: number[], n: number): number[] {
  let cuts = [];
  while (n !== 0) {
    cuts.push(s[n]);
    n -= s[n];
  }
  return cuts;
}

function extendedBottomUpCutRod(prices: number[], n: number): [number, number[]] {
  let r: number[] = new Array(n + 1);
  let s: number[] = new Array(n + 1);

  r[0] = 0;
  for (let j = 1; j <= n; j++) {
    //  total price, initially uncut
    let q = prices[j];
    //  length of first cut, initially the whole rod
    let c = j;
    for (let i = 1; i < j; i++) {
      if (q < prices[i] + r[j - i]) {
        q = prices[i] + r[j - i];
        c = i;
      }
      r[j] = q;
      s[j] = c;
    }
  }

  return [r[n], constrCuts(s, n)];
}

//  c: fixed cut cost
function cutRodCC(prices: number[], c: number, n: number): number {
  let r: number[] = new Array(n + 1);

  r[0] = 0;

  for (let j = 1; j <= n; j++) {
    let q = prices[j];
    for (let i = 1; i < j; i++) {
      q = Math.max(q, prices[i] + r[j - i] - c);
    }
    r[j] = q;
  }

  return r[n];
}

function extendedMemoizedCutRod(prices: number[], n: number): [number, number[]] {
  //  closure, modifies r and s
  function aux(k: number): number {
    if (r[k] !== undefined) {
      return r[k];
    }
    //  assumes prices[0] = 0;
    let q = prices[k];
    s[k] = k;
    for (let i = 1; i < k; i++) {
      let qi = prices[i] + aux(k - i);
      if (q < qi) {
        q = qi;
        s[k] = i;
      }
    }
    r[k] = q;
    return q;
  }

  let r: number[] = new Array(n + 1);
  let s: number[] = new Array(n + 1);

  let p = aux(n);
  return [p, constrCuts(s, n)];
}

function fibonacci(n: number) {
  let f = new Array(n + 1);
  f[0] = 0;
  f[1] = 1;

  for (let i = 2; i <= n; i++) {
    f[i] = f[i - 1] + f[i - 2];
  }

  return f[n];
}
