export {
  longDivision,
};

function longDivision(a: number, b: number): [number, number] {
  let na = Math.abs(a).toString(2).length;
  let nb = Math.abs(b).toString(2).length;
  let s = na - nb;
  let q = quotient(Math.abs(a), Math.abs(b), s);
  let sign = Math.sign(a) * Math.sign(b);
  q *= sign;
  if (Math.sign(q) * Math.sign(b) === -1) {
    q--;
  }
  return [q, a - q * b];
}

function quotient(a: number, b: number, s: number): number {
  let mul = 2 ** s;
  b *= mul;
  let q = 0;
  while (mul >= 1) {
    if (a >= b) {
      a -= b;
      q += mul;
    }
    b /= 2;
    mul /= 2;
  }
  return q;
}
