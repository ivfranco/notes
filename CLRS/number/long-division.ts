export {
  longDivision,
};

function longDivision(a: number, b: number): [number, number] {
  let na = a.toString(2).length;
  let nb = b.toString(2).length;
  let s = na - nb;
  let bits = longDivisionRecursive(a, b * (2 ** s), s);
  let q = 0;
  for (let i = 0, mul = 1; i <= s; i++) {
    q += mul * bits[i];
    mul *= 2;
  }
  return [q, a - q * b];
}

function longDivisionRecursive(a: number, b: number, s: number): number[] {
  if (s < 0) {
    return [];
  } else {
    let bits: number[];
    if (a >= b) {
      bits = longDivisionRecursive(a - b, b / 2, s - 1);
      bits.push(1);
    } else {
      bits = longDivisionRecursive(a, b / 2, s - 1);
      bits.push(0);
    }
    return bits;
  }
}
