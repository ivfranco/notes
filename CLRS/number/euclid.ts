export {
  gcd,
  lcm,
  euclid,
  extendedGcd,
  extendedEuclid,
  binaryGcd,
};

function euclid(a: number, b: number): number {
  while (b !== 0) {
    let temp = b;
    b = a % b;
    a = temp;
  }
  return a;
}

function extendedEuclid(a: number, b: number): [number, number, number] {
  if (b === 0) {
    return [a, 1, 0];
  } else {
    let [d, x, y] = extendedEuclid(b, a % b);
    return [d, y, x - Math.floor(a / b) * y];
  }
}

function gcd(...as: number[]): number {
  return as.reduce((a, b) => euclid(a, b));
}

/* tslint:disable:no-bitwise */
function binaryGcd(a: number, b: number): number {
  let k = 0;
  while (a !== 0 && b !== 0) {
    if ((a & 0x1) === 0 && (b & 0x1) === 0) {
      k++;
      a >>= 1;
      b >>= 1;
    } else if ((a & 0x1) === 0) {
      a >>= 1;
    } else if ((b & 0x1) === 0) {
      b >>= 1;
    } else {
      let max = Math.max(a, b);
      let min = Math.min(a, b);
      a = (max - min) >> 1;
      b = min;
    }
  }

  return Math.max(a, b) << k;
}
/* tslint:enable:no-bitwise */

function extendedGcd(...as: number[]): [number, number[]] {
  let n = as.length;
  let coeffs: number[] = [];
  coeffs[n - 1] = 1;
  let s = as[n - 1];
  for (let i = n - 2; i >= 0; i--) {
    let [d, x, y] = extendedEuclid(as[i], s);
    coeffs[i] = x;
    for (let j = i + 1; j < n; j++) {
      coeffs[j] *= y;
    }
    s = d;
  }

  return [s, coeffs];
}

function lcm(...as: number[]): number {
  return as.reduce((a, b) => a * b / euclid(a, b));
}
