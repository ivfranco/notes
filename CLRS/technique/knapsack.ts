export {
  fractionalKnapsack,
  knapsack,
  linearKnapsack,
};

import { median } from "../sort/order";

type Weight = number;
type Value = number;

interface Item {
  w: Weight;
  v: Value;
}

function fractionalKnapsack(I: Item[], W: Weight): Array<[Weight, Item]> {
  I = I.slice();
  //  sorted by value density (value / weight)
  I.sort((i1, i2) => (i1.v / i1.w) - (i2.v / i2.w));
  let sack: Array<[number, Item]> = [];

  for (let i = I.length - 1; W > 0 && i >= 0; i--) {
    let w = Math.min(I[i].w, W);
    W -= w;
    sack.push([w, I[i]]);
  }

  return sack;
}

function knapsack(I: Item[], W: Weight): Item[] {
  let n = I.length;
  let value: Value[][] = [];
  let taken: boolean[][] = [];
  for (let i = 0; i < n; i++) {
    value[i] = [];
    taken[i] = [];
  }

  function aux(i: number, w: Weight): Value {
    if (i >= n) {
      return 0;
    }
    if (value[i][w] !== undefined) {
      return value[i][w];
    }

    value[i][w] = -Infinity;
    let q;
    if (I[i].w <= w) {
      q = aux(i + 1, w - I[i].w) + I[i].v;
      if (q > value[i][w]) {
        value[i][w] = q;
        taken[i][w] = true;
      }
    }
    q = aux(i + 1, w);
    if (q > value[i][w]) {
      value[i][w] = q;
      taken[i][w] = false;
    }

    return value[i][w];
  }

  aux(0, W);
  return constrSack(I, W, taken);
}

function constrSack(I: Item[], W: Weight, taken: boolean[][]): Item[] {
  let n = I.length;
  let sack = [];

  for (let i = 0; i < n && W > 0; i++) {
    if (taken[i][W]) {
      sack.push(I[i]);
      W -= I[i].w;
    }
  }

  return sack;
}

function linearKnapsack(I: Item[], W: Weight): Array<[Weight, Item]> {
  let n = I.length;
  if (n === 0) {
    return [];
  }
  if (n === 1) {
    return [[Math.min(I[0].w, W), I[0]]];
  }

  let m = median(I.map(i => i.v / i.w));
  let higher = I.filter(i => i.v / i.w > m);
  let lower = I.filter(i => i.v / i.w <= m);
  let w = higher.reduce((sum, i) => sum + i.w, 0);

  if (W === w) {
    return higher.map(i => [i.w, i] as [Weight, Item]);
  } else if (W > w) {
    return linearKnapsack(lower, W - w).concat(higher.map(i => [i.w, i] as [Weight, Item]));
  } else {
    return linearKnapsack(higher, W);
  }
}
