export {
  editDistance,
  alignDNA,
};

interface Cost {
  copy: number;
  replace: number;
  delete: number;
  insert: number;
  twiddle: number;
  kill: number;
}

enum Op {
  COPY = "copy",
  REPLACE = "replace",
  DELETE = "delete",
  INSERT = "insert",
  TWIDDLE = "twiddle",
  KILL = "kill",
}

type RevOp = (x: string, y: string, i: number, j: number) => [number, number] | null;

//  determines possible last operations, and the value of i and j before that operation (if possible)
const rCopy: RevOp = (x, y, i, j) => {
  if (i >= 1 && j >= 1 && x[i - 1] === y[j - 1]) {
    return [i - 1, j - 1];
  } else {
    return null;
  }
};

const rReplace: RevOp = (x, y, i, j) => {
  if (i >= 1 && j >= 1) {
    return [i - 1, j - 1];
  } else {
    return null;
  }
};

const rDelete: RevOp = (x, y, i, j) => {
  if (i >= 1) {
    return [i - 1, j];
  } else {
    return null;
  }
};

const rInsert: RevOp = (x, y, i, j) => {
  if (j >= 1) {
    return [i, j - 1];
  } else {
    return null;
  }
};

const rTwiddle: RevOp = (x, y, i, j) => {
  if (i >= 2 && j >= 2 && x[i - 1] === y[j - 2] && x[i - 2] === y[j - 1]) {
    return [i - 2, j - 2];
  } else {
    return null;
  }
};

//  kill is a special case as it does not have unique previous values of i and j
//  will be treated directly in the main procedure

const revOps: { [k in Op]: RevOp } = {
  copy: rCopy,
  delete: rDelete,
  insert: rInsert,
  kill: (x, y, i, j) => null,
  replace: rReplace,
  twiddle: rTwiddle,
};

function editDistance(x: string, y: string, cost: Cost): [number, string] {
  let n = x.length;
  let m = y.length;
  let dis: number[][] = new Array(n + 1);
  //  a table storing both the previous operation and the previous value of i and j
  //  in all case except kill, previous i, j can be derived from previous operation and current i, j
  let rev: Array<Array<[Op, [number, number]]>> = new Array(n + 1);
  for (let i = 0; i <= n; i++) {
    dis[i] = new Array(m + 1);
    rev[i] = new Array(m + 1);
  }
  dis[0][0] = 0;

  function aux(i: number, j: number): number {
    if (dis[i][j] !== undefined) {
      return dis[i][j];
    }

    dis[i][j] = +Infinity;
    //  handle kill
    if (i === n && j === m) {
      //  previous i can be anything from 0 to n-1
      for (let k = 0; k < i; k++) {
        let q = aux(k, j) + cost.kill;
        if (q < dis[i][j]) {
          dis[i][j] = q;
          rev[i][j] = [Op.KILL, [k, j]];
        }
      }
    }

    for (let op of [Op.COPY, Op.REPLACE, Op.DELETE, Op.INSERT, Op.TWIDDLE]) {
      let revOp = revOps[op];
      let prevs = revOp(x, y, i, j);
      if (prevs) {
        let [p_i, p_j] = prevs;
        let q = aux(p_i, p_j) + cost[op];
        if (q < dis[i][j]) {
          dis[i][j] = q;
          rev[i][j] = [op, [p_i, p_j]];
        }
      }
    }

    return dis[i][j];
  }

  aux(n, m);
  return [dis[n][m], opSequence(rev, n, m)];
}

function opSequence(rev: Array<Array<[Op, [number, number]]>>, n: number, m: number): string {
  let seq: string[] = [];
  while (n > 0 && m > 0) {
    let [op, [i, j]] = rev[n][m];
    seq.push(op);
    n = i;
    m = j;
  }

  return seq.reverse().join(" -> ");
}

function alignDNA(x: string, y: string): number {
  let cost = {
    copy: -1,
    delete: 2,
    insert: 2,
    kill: +Infinity,
    replace: 1,
    twiddle: +Infinity,
  };

  let [distance, _] = editDistance(x, y, cost);
  return -distance;
}
