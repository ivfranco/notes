export {
  pReduce,
  pForEach,
  pFoldMap,
  pScan,
};

const GRAIN_SIZE = 1;

import { id, noop } from "../util";

async function pFoldMap<A, B>(
  A: A[], i: number, j: number, f: (a: A, i?: number) => B, g: (a: B, b: B) => B,
): Promise<B> {
  if (j - i + 1 <= GRAIN_SIZE) {
    let acc = f(A[i], i);
    for (let k = i + 1; k <= j; k++) {
      acc = g(acc, f(A[k], k));
    }
    return acc;
  } else {
    let mid = Math.floor((i + j) / 2);
    let handle = pFoldMap(A, i, mid, f, g);
    let rhs = await pFoldMap(A, mid + 1, j, f, g);
    let lhs = await handle;
    return g(lhs, rhs);
  }
}

async function pReduce<T>(A: T[], i: number, j: number, f: (a: T, b: T) => T): Promise<T> {
  return pFoldMap(A, i, j, id, f);
}

async function pForEach<T>(A: T[], i: number, j: number, f: (a: T, i?: number) => void): Promise<void> {
  return pFoldMap(A, i, j, f, noop);
}

async function pScan<T>(x: T[], f: (a: T, b: T) => T): Promise<T[]> {
  let n = x.length;
  let y: T[] = [];
  let t: T[] = [];
  y[0] = x[0];
  if (n > 1) {
    await pScanUp(x, t, 1, n - 1, f);
    await pScanDown(x[0], x, t, y, 1, n - 1, f);
  }
  return y;
}

async function pScanUp<T>(x: T[], t: T[], i: number, j: number, f: (a: T, b: T) => T): Promise<T> {
  if (i === j) {
    return x[i];
  } else {
    let k = Math.floor((i + j) / 2);
    let handle = pScanUp(x, t, i, k, f);
    let right = await pScanUp(x, t, k + 1, j, f);
    t[k] = await handle;
    return f(t[k], right);
  }
}

async function pScanDown<T>(v: T, x: T[], t: T[], y: T[], i: number, j: number, f: (a: T, b: T) => T) {
  if (i === j) {
    y[i] = f(v, x[i]);
  } else {
    let k = Math.floor((i + j) / 2);
    let handle = pScanDown(v, x, t, y, i, k, f);
    await pScanDown(f(v, t[k]), x, t, y, k + 1, j, f);
    await handle;
  }
}
