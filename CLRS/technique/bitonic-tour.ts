export {
  bitonicTour,
  constrBitonicTour,
};

interface Point {
  x: number;
  y: number;
}

function distance(p: Point, q: Point): number {
  return Math.hypot(p.x - q.x, p.y - q.y);
}

function bitonicTour(p: Point[]): [number, number[][]] {
  function aux(i: number, j: number): number {
    if (b[i][j] != null) {
      return b[i][j];
    }

    if (i === j - 1) {
      b[j - 1][j] = +Infinity;
      for (let k = 0; k < j - 1; k++) {
        let q = aux(k, j - 1) + distance(p[k], p[j]);
        if (q < b[j - 1][j]) {
          b[j - 1][j] = q;
          r[j - 1][j] = k;
        }
      }
    } else {
      b[i][j] = aux(i, j - 1) + distance(p[j - 1], p[j]);
      r[i][j] = j - 1;
    }

    return b[i][j];
  }

  p.sort((p1, p2) => p1.x - p2.x);
  let n = p.length;
  let b: number[][] = new Array(n);
  let r: number[][] = new Array(n);
  for (let i = 0; i < n; i++) {
    b[i] = [];
    r[i] = [];
  }
  b[0][0] = 0;

  for (let i = 1; i < n; i++) {
    b[0][i] = b[0][i - 1] + distance(p[i - 1], p[i]);
  }

  aux(n - 2, n - 1);
  b[n - 1][n - 1] = b[n - 2][n - 1] + distance(p[n - 2], p[n - 1]);
  return [b[n - 1][n - 1], r];
}

function constrBitonicTour(r: number[][], n: number): string {
  function aux(i: number, j: number): string {
    if (i < j) {
      let k = r[i][j];
      if (k > 0) {
        return `${aux(i, k)} -> p${k}`;
      } else {
        return `-> p${k}`;
      }
    } else {
      let k = r[j][i];
      if (k > 0) {
        return `-> p${k} ${aux(k, j)}`;
      } else {
        return `-> p${k}`;
      }
    }
  }

  let s = `p${n - 1} -> p${n - 2}`;
  return s + aux(n - 2, n - 1);
}
