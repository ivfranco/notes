export {
  SlackForm,
  simplex,
};

type Var = number;
type Coff = number;

const EPSILON = 1e-6;

class SlackForm {
  //  set of nonbasic variables, has fixed size same to n the number of variables in the standard form
  private N: Set<Var>;
  //  set of basic variables, has fixed size same to m the number of constraints in the standard form
  private B: Set<Var>;
  //  an (n + m) x (n + m) matrix of constraint cofficients
  private A: Coff[][];
  //  an array of length (n + m) that stores constants in constraints
  private b: Coff[];
  //  an array of length (n + m) that stores cofficients in objective functions
  private c: Coff[];
  //  the objective value of the basic solution
  private v: Coff;

  constructor(A: Coff[][], b: Coff[], c: Coff[]) {
    let m = A.length;
    let n = A[0].length;
    console.assert(m === b.length && n === c.length, "Invalid standard form");

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

  private removeFromBasis(i: Var) {
    let { A, B } = this;

    if (B.has(i)) {
      for (let j of this.nonbasic()) {
        if (A[i][j] !== 0) {
          this.pivot(j, i);
          return;
        }
      }
    }
  }

  private removeVariable(i: Var) {
    let { A, b, c, N, B } = this;
    A.forEach(r => r.splice(i, 1));
    A.splice(i, 1);
    b.splice(i, 1);
    c.splice(i, 1);
    N.delete(i);
    let Ns = Array.from(N).map(j => j > i ? j - 1 : j);
    let Bs = Array.from(B).map(j => j > i ? j - 1 : j);
    this.N = new Set(Ns);
    this.B = new Set(Bs);
  }

  public restoreObjective(i: Var, c: Coff[]) {
    let { N, B } = this;
    this.removeFromBasis(i);
    this.removeVariable(i);
    while (c.length <= N.size + B.size) {
      c.push(0);
    }
    this.c = c;

    for (let j of this.basic()) {
      this.substObject(j, j);
    }
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
    let n = this.N.size;
    let b = this.b;
    let x: Coff[] = [];

    for (let i of this.basic()) {
      x[i] = b[i];
    }
    for (let i of this.nonbasic()) {
      x[i] = 0;
    }

    return x.slice(0, n);
  }

  public value(): number {
    return this.v;
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
    B.sort((l, r) => l - r);
    let N = Array.from(this.nonbasic());
    N.sort((l, r) => l - r);

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
  let slack = initializeSimplex(A, b, c);
  if (slack) {
    return slack.simplex();
  } else {
    return null;
  }
}

class StandardForm {
  private A: Coff[][];
  private b: Coff[];
  private c: Coff[];

  constructor(A: Coff[][], b: Coff[], c: Coff[]) {
    this.A = A;
    this.b = b;
    this.c = c;
  }

  //  append -xn to each constraint, set objective function to -xn
  public auxProgram(): StandardForm {
    let { A, b, c } = this;
    let n = c.length;

    let A_aux = A.map(r => {
      let row = r.slice();
      row.push(-1);
      return row;
    });
    let c_aux = new Array(n + 1);
    c_aux.fill(0);
    c_aux[n] = -1;

    return new StandardForm(A_aux, b.slice(), c_aux);
  }

  public toSlackForm(): SlackForm {
    let { A, b, c } = this;
    return new SlackForm(A, b, c);
  }
}

function minIndex(A: Coff[]): number {
  let min = 0;
  for (let i = 0; i < A.length; i++) {
    if (A[i] <= A[min]) {
      min = i;
    }
  }
  return min;
}

function initializeSimplex(A: Coff[][], b: Coff[], c: Coff[]): SlackForm | null {
  let m = b.length;
  let n = c.length;
  console.assert(A.length === m && A[0].length === n, "Invalid input size");

  let k = minIndex(b);
  if (b[k] >= 0) {
    return new SlackForm(A, b, c);
  }

  let L = new StandardForm(A, b, c);
  let Laux = L.auxProgram().toSlackForm();
  Laux.pivot(n, n + k);
  Laux.simplex();
  if (Math.abs(Laux.value() - 0) <= EPSILON) {
    Laux.restoreObjective(n, b);
    return Laux;
  } else {
    console.log(Laux.value());
    console.error("Error: Linear program is infeasible");
    return null;
  }
}
