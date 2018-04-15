export {
  activitySelection,
  greedyActivitySelection,
};

interface Act {
  s: number;
  f: number;
}

//  assume the array A is sorted according to finish time
function greedyActivitySelection(A: Act[]): Act[] {
  let n = A.length;
  let opt = [A[0]];
  for (let i = 1, k = 0; i < n; i++) {
    if (A[i].s >= A[k].f) {
      opt.push(A[i]);
      k = i;
    }
  }
  return opt;
}

function activitySelection(A: Act[]): [number, Act[]] {
  let n = A.length;
  //  add two dummy activities to the set so the main problem can be addressed just like subproblems
  let dummy_start = { s: 0, f: 0 };
  let dummy_end = { s: A[n - 1].f, f: A[n - 1].f };
  //  input array untouched
  A = A.slice();
  A.push(dummy_start, dummy_end);
  //  sort anyway, won't affect the asymptotic running time
  A.sort((a1, a2) => a1.f - a2.f);
  n = A.length;

  let c: number[][] = [];
  //  records optimal choices of k
  let r: number[][] = [];
  for (let i = 0; i < n; i++) {
    c[i] = [];
    r[i] = [];
  }

  for (let i = 0; i + 1 < n; i++) {
    //  as reasons stated below, solution to subproblem (i, i+1) is 0
    c[i][i + 1] = 0;
  }

  function aux(i: number, j: number): number {
    if (c[i][j] !== undefined) {
      return c[i][j];
    }

    //  if an activity k finishes before A[j] starts, A[k].f <= A[j].s < A[j].f, k < j
    //  if an activity k starts after A[i] finishes, A[i].f <= A[k].s < A[k].f, k > i
    //  so it's sufficient to search k among i < k < j
    c[i][j] = 0;
    for (let k = i + 1; k < j; k++) {
      if (A[k].s >= A[i].f && A[k].f <= A[j].s) {
        let q = aux(i, k) + aux(k, j) + 1;
        if (q > c[i][j]) {
          c[i][j] = q;
          r[i][j] = k;
        }
      }
    }

    return c[i][j];
  }

  aux(0, n - 1);
  return [c[0][n - 1], constrSelection(A, r, 0, n - 1)];
}

function constrSelection(A: Act[], r: number[][], i: number, j: number): Act[] {
  let k = r[i][j];
  if (k === undefined) {
    return [];
  } else {
    return [
      ...constrSelection(A, r, i, k),
      A[k],
      ...constrSelection(A, r, k, j),
    ];
  }
}
