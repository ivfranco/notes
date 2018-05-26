export {
  SlackForm,
  simplex,
};

type Var = number;
type Coff = number;

class SlackForm {
  private N: Set<Var>;
  private B: Set<Var>;
  private A: Coff[][];
  private b: Coff[];
  private c: Coff[];
  private v: Coff;

  constructor(A: Coff[][], b: Coff[], c: Coff[]) {
    let m = A.length;
    let n = A[0].length;
    console.assert(m === b.length && n === c.length, "Invalid standard form");
    console.assert(b.every(coff => coff >= 0), "Infeasible standard form");

    let N = new Set();
    let B = new Set();
    let A_extend: Coff[][] = [];
    let b_extend: Coff[] = [];
    for (let i = m - 1; i >= 0; i--) {
      B.add(n + i);
      A_extend[n + i] = A[i].slice();
      b_extend[n + i] = b[i];
    }
    for (let i = 0; i < n; i++) {
      N.add(i);
      A_extend[i] = [];
      b_extend[i] = 0;
    }

    this.N = N;
    this.B = B;
    this.A = A_extend;
    this.b = b_extend;
    this.c = c.slice();
    this.v = 0;
  }

  public *basic(): IterableIterator<Var> {
    yield* this.B;
  }

  public *nonbasic(): IterableIterator<Var> {
    yield* this.N;
  }

  public isOptimal(): boolean {
    let c = this.c;
    for (let i of this.nonbasic()) {
      if (c[i] > 0) {
        return false;
      }
    }
    return true;
  }

  public nextPivot(): [Var, Var] | null {
    let { N, B, A, b, c } = this;
    let es = Array.from(this.nonbasic()).filter(i => c[i] > 0);
    //  assume the base solution is not yet optimal
    let e = es.reduce((i, j) => Math.min(i, j));
    let D = new Array(N.size + B.size);
    D.fill(+Infinity);
    for (let i of this.basic()) {
      if (A[i][e] > 0) {
        D[i] = b[i] / A[i][e];
      }
    }
    let min = D.reduce((i, j) => Math.min(i, j));
    if (min === +Infinity) {
      console.error("Error: Linear program is unbounded");
      return null;
    }
    let ls: Var[] = [];
    for (let i of this.basic()) {
      if (D[i] === min) {
        ls.push(i);
      }
    }
    let l = ls.reduce((i, j) => Math.min(i, j));
    return [e, l];
  }

  private exchangeRole(e: Var, l: Var) {
    let { A, b } = this;

    b[e] = b[l] / A[l][e];
    for (let j of this.nonbasic()) {
      if (j !== e) {
        A[e][j] = A[l][j] / A[l][e];
      }
    }
    A[e][l] = 1 / A[l][e];
  }

  private substConstraint(e: Var, i: Var, l: Var) {
    let { A, b } = this;

    b[i] -= A[i][e] * b[e];
    for (let j of this.nonbasic()) {
      if (j !== e) {
        A[i][j] -= A[i][e] * A[e][j];
      }
    }
    A[i][l] = -A[i][e] * A[e][l];
  }

  private substObject(e: Var, l: Var) {
    let { A, b, c } = this;

    this.v += c[e] * b[e];
    for (let j of this.nonbasic()) {
      if (j !== e) {
        c[j] -= c[e] * A[e][j];
      }
    }
    c[l] = -c[e] * A[e][l];
  }

  public pivot(e: Var, l: Var) {
    let { N, B } = this;
    console.assert(N.has(e), "Entering variable must be non-basic");
    console.assert(B.has(l), "Leaving variable must be basic");

    this.exchangeRole(e, l);
    for (let i of this.basic()) {
      if (i !== l) {
        this.substConstraint(e, i, l);
      }
    }
    this.substObject(e, l);
    N.delete(e);
    N.add(l);
    B.delete(l);
    B.add(e);
  }

  public basicSolution(): Coff[] {
    let b = this.b;
    let n = this.N.size;
    let x: Coff[] = [];

    for (let i of this.basic()) {
      x[i] = b[i];
    }
    for (let i of this.nonbasic()) {
      x[i] = 0;
    }

    return x.slice(0, n);
  }

  public dualSolution(): Coff[] {
    let { N, B, c } = this;
    let m = B.size;
    let n = N.size;
    let y = new Array(m);
    y.fill(0);

    for (let i of this.nonbasic()) {
      if (i - n >= 0) {
        y[i - n] = -c[i];
      }
    }

    return y;
  }

  public simplex(): Coff[] | null {
    while (!this.isOptimal()) {
      let pivot = this.nextPivot();
      if (!pivot) {
        return null;
      } else {
        let [e, l] = pivot;
        this.pivot(e, l);
      }
    }

    return this.basicSolution();
  }

  public show(): string {
    function withSign(coff: Coff, x: Var): string {
      if (coff > 0) {
        return ` + ${coff}x${x}`;
      } else if (coff < 0) {
        return ` - ${-coff}x${x}`;
      } else {
        return "";
      }
    }

    let B = Array.from(this.basic());
    B.sort((a, b) => a - b);
    let N = Array.from(this.nonbasic());
    N.sort((a, b) => a - b);

    let { A, b, c, v } = this;

    let object = `maximize ${v}`;
    for (let i of N) {
      object += withSign(c[i], i);
    }

    let constraints: string[] = [];
    for (let i of B) {
      let constraint = `x${i} = ${b[i]}`;
      for (let j of N) {
        constraint += withSign(-A[i][j], j);
      }
      constraints.push(constraint);
    }

    return [
      object,
      "subject to",
      ...constraints,
    ].join("\n");
  }
}

function simplex(A: Coff[][], b: Coff[], c: Coff[]): Coff[] | null {
  let n = A[0].length;
  let slack = new SlackForm(A, b, c);

  return slack.simplex();
}
