export {
  BitReversedCounter,
};

import { SWAP } from "../util";

function rev(a: number, k: number): number {
  let b = 0;
  for (let i = 0; i < k; i++) {
    b *= 2;
    b += a % 2;
    a = Math.floor(a / 2);
  }
  return b;
}

function bitReversalPermutation<T>(A: T[], k: number) {
  let n = A.length;
  console.assert(n === 2 ** k);

  let flags: boolean[] = new Array(n);
  flags.fill(false);

  for (let i = 0; i < n; i++) {
    if (!flags[i]) {
      let r = rev(i, k);
      SWAP(A, i, r);
      flags[i] = true;
      flags[r] = true;
    }
  }
}

/* tslint:disable:no-bitwise */
class BitReversedCounter {
  private b: number;
  private k: number;

  constructor(b: number, k: number) {
    //  the initial value
    this.b = b;
    //  the number of bits
    this.k = k;
  }

  public increment() {
    let k = this.k;
    let b = this.b;

    let mask = 0x1 << (k - 1);
    while ((mask & b) !== 0) {
      b = mask ^ b;
      mask = mask >>> 1;
    }
    b += mask;
    this.b = b;
  }

  public get(): number {
    return this.b;
  }
}
