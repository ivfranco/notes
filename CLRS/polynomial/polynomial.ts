export {
  Polynomial,
  coffMultiply,
  interpolate,
  Coeff,
};

type Coeff = number;

class Polynomial {
  public readonly coffs: Coeff[];

  constructor(coffs: Coeff[]) {
    this.coffs = coffs;
  }

  public degree(): number {
    return this.coffs.length - 1;
  }

  public map(f: (a: number) => number): Polynomial {
    return new Polynomial(this.coffs.map(f));
  }

  public evaluate(x: number): number {
    let mul = 1;
    let sum = 0;
    let k = this.degree();
    let c = this.coffs;

    for (let i = 0; i <= k; i++) {
      sum += c[i] * mul;
      mul *= x;
    }

    return sum;
  }

  public show(): string {
    let n = this.degree();
    let coffs = this.coffs;

    let rep: string[] = [];
    for (let i = n; i >= 0; i--) {
      let c = coffs[i];
      if (c !== 0) {
        let sign = "";
        if (c > 0 && rep.length > 0) {
          sign = "+ ";
        } else if (c < 0) {
          sign = "- ";
        }
        let term = "";
        if (i === 1) {
          term = "x";
        } else if (i > 1) {
          term = `x^${i}`;
        }
        rep.push(`${sign}${Math.abs(c)}${term}`);
      }
    }

    return rep.join(" ");
  }
}

function quotient(p: Polynomial, x0: number): [Polynomial, Coeff] {
  let k = p.degree();
  let q: Coeff[] = [];
  let c = p.coffs;
  q[k - 1] = c[k];
  for (let i = k - 2; i >= 0; i--) {
    q[i] = c[i + 1] + x0 * q[i + 1];
  }
  let r = c[0] - x0 * q[0];
  return [new Polynomial(q), r];
}

function coffSum(pa: Polynomial, pb: Polynomial): Polynomial {
  let na = pa.degree();
  let nb = pb.degree();

  let coffs: Coeff[] = new Array(Math.max(na, nb) + 1);
  coffs.fill(0);
  for (let i = 0; i <= na; i++) {
    coffs[i] += pa.coffs[i];
  }
  for (let i = 0; i <= nb; i++) {
    coffs[i] += pb.coffs[i];
  }

  return new Polynomial(coffs);
}

function coffMultiply(pa: Polynomial, pb: Polynomial): Polynomial {
  let na = pa.degree();
  let nb = pb.degree();

  let coffs: Coeff[] = [];
  for (let i = 0; i <= na + nb; i++) {
    coffs[i] = 0;
    for (let k = 0; k <= i; k++) {
      if (k <= na && i - k <= nb) {
        coffs[i] += pa.coffs[k] * pb.coffs[i - k];
      }
    }
  }

  return new Polynomial(coffs);
}

interface Point {
  x: number;
  y: number;
}

function interpolate(pts: Point[]): Polynomial {
  let x = pts.map(p => p.x);
  let y = pts.map(p => p.y);
  let n = x.length - 1;

  let mul = new Polynomial([1]);
  for (let i = 0; i <= n; i++) {
    let diff = new Polynomial([-x[i], 1]);
    mul = coffMultiply(mul, diff);
  }

  let sum = new Polynomial([0]);
  for (let i = 0; i <= n; i++) {
    let [quot] = quotient(mul, x[i]);
    let denom = 1;
    for (let j = 0; j <= n; j++) {
      if (j !== i) {
        denom *= x[i] - x[j];
      }
    }
    sum = coffSum(sum, quot.map(a => y[i] * a / denom));
  }

  return sum;
}
