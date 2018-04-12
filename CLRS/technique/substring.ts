export {
  lcs,
  lis,
  quadraticLis,
  memoizedLcs,
  linearSpaceLcs,
  constrSubstring,
};

function lcs<T>(X: T[], Y: T[]): number[][] {
  let m = X.length;
  let n = Y.length;

  let c: number[][] = new Array(m + 1);
  for (let i = 0; i <= m; i++) {
    c[i] = new Array(n + 1);
    c[i][0] = 0;
  }
  for (let i = 0; i <= n; i++) {
    c[0][i] = 0;
  }

  for (let i = 0; i < m; i++) {
    for (let j = 0; j < n; j++) {
      if (X[i] === Y[j]) {
        c[i + 1][j + 1] = c[i][j] + 1;
      } else {
        let up = c[i][j + 1];
        let left = c[i + 1][j];
        c[i + 1][j + 1] = Math.max(up, left);
      }
    }
  }

  return c;
}

function constrSubstring<T>(X: T[], Y: T[], c: number[][]): T[] {
  let i = X.length;
  let j = Y.length;
  let s: T[] = [];

  while (i > 0 && j > 0) {
    if (X[i - 1] === Y[j - 1]) {
      s.push(X[i - 1]);
      i--;
      j--;
    } else {
      let up = c[i - 1][j];
      let left = c[i][j - 1];
      if (up >= left) {
        i--;
      } else {
        j--;
      }
    }
  }

  return s.reverse();
}

function memoizedLcs<T>(X: T[], Y: T[]): number[][] {
  //  closure, modifies c
  function aux(i: number, j: number): number {
    if (c[i][j] !== undefined) {
      return c[i][j];
    }

    if (X[i - 1] === Y[j - 1]) {
      let q = aux(i - 1, j - 1) + 1;
      c[i][j] = q;
      return q;
    } else {
      let q = Math.max(aux(i - 1, j), aux(i, j - 1));
      c[i][j] = q;
      return q;
    }
  }

  let m = X.length;
  let n = Y.length;
  let c: number[][] = new Array(m + 1);
  for (let i = 0; i <= m; i++) {
    c[i] = new Array(n + 1);
    c[i][0] = 0;
  }
  for (let i = 0; i <= n; i++) {
    c[0][i] = 0;
  }

  aux(m, n);
  return c;
}
function linearSpaceLcs<T>(X: T[], Y: T[]): number {
  if (X.length > Y.length) {
    return linearSpaceLcs(Y, X);
  }

  let m = X.length;
  let n = Y.length;

  let c: number[] = new Array(m);

  //  as c has only m entries now, the initial value 0 on each row cannot be stored in c
  //  must treat 0 (now -1) as a special case in a helper funtion
  function access(i: number): number {
    if (i === -1) {
      return 0;
    } else {
      return c[i];
    }
  }

  c.fill(0);
  for (let j = 0; j < n; j++) {
    //  always stores c[i][j] at the start of each iteration which assigns c[i+1][j+1]
    let top_left = 0;
    for (let i = 0; i < m; i++) {
      let prev = c[i];
      if (X[i] === Y[j]) {
        c[i] = top_left + 1;
      } else {
        c[i] = Math.max(c[i], access(i - 1));
      }
      top_left = prev;
    }
  }

  return c[m - 1];
}

function firstGreater<T>(k: T, A: T[], p: number, r: number): number {
  if (p < r) {
    let q = Math.floor((p + r) / 2);
    if (k >= A[q]) {
      return firstGreater(k, A, q + 1, r);
    } else {
      return firstGreater(k, A, p, q);
    }
  } else {
    return p;
  }
}

function lis(A: number[]): number[] {
  let best_end: number[] = [];
  let prev_end: Array<number | null> = [];

  for (let a of A) {
    let max_k = best_end.length - 1;

    let k = firstGreater(a, best_end, 0, max_k);
    if (best_end[k] === undefined || best_end[k] <= a) {
      //  best_end[k] === undefined only in the first iteration
      //  otherwise all elements in best_end <= A[i]
      best_end.push(a);
      prev_end[a] = max_k < 0 ? null : best_end[max_k];
    } else {
      //  best_end[k] > A[i]
      best_end[k] = a;
      prev_end[a] = k - 1 < 0 ? null : best_end[k - 1];
    }
  }

  let seq: number[] = [];
  let end: number | null = best_end[best_end.length - 1];
  while (end != null) {
    seq.push(end);
    end = prev_end[end];
  }
  return seq.reverse();
}

function quadraticLis(A: number[]): number[] {
  let S = A.slice();
  S.sort((a, b) => a - b);

  return constrSubstring(A, S, lcs(A, S));
}
