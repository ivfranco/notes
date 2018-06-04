export {
  nontrivalPower,
};

function binarySearchBase(n: number, power: number, lower: number, upper: number): number {
  if (upper > lower) {
    let mid = Math.floor((upper + lower) / 2);
    let pow = mid ** power;
    if (pow === n) {
      return mid;
    } else if (pow > n) {
      return binarySearchBase(n, power, lower, mid - 1);
    } else {
      return binarySearchBase(n, power, mid + 1, upper);
    }
  } else {
    return lower;
  }
}

function nontrivalPower(n: number): [number, number] | null {
  let beta = n.toString(2).length;
  for (let k = 2; k < beta; k++) {
    let a = binarySearchBase(n, k, 1, n);
    if (a ** k === n) {
      return [a, k];
    }
  }
  return null;
}
