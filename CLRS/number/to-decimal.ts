export {
  toDecimal,
};

function toDecimal(a: number): string {
  if (a >= 10) {
    let k = Math.ceil(Math.log10(a) / 2);
    let b = 10 ** k;
    let q = Math.floor(a / b);
    let r = a % b;
    let hi = toDecimal(q);
    let lo = toDecimal(r);
    while (lo.length < k) {
      //  assuming prepending a string is O(1), it may be O(n)
      lo = "0" + lo;
    }
    return hi + lo;
  } else {
    return a.toString(10);
  }
}
