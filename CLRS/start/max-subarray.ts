export {
  findMaximumSubarray,
  findMaximumSubarrayBrute,
  findMaximumSubarrayMix,
  findMaximumSubArrayLinear
};

type Triple = [number, number, number];

function findMaxCrossingSubarray(A: number[], low: number, mid: number, high: number): Triple {
  let leftSum = -Infinity, rightSum = -Infinity;
  // initial value never used, initialized purely to make type checker happy
  let maxLeft = 0, maxRight = 0;

  for (let i = mid, sum = 0; i >= low; i--) {
    sum += A[i];
    if (sum > leftSum) {
      leftSum = sum;
      maxLeft = i;
    }
  }

  for (let i = mid + 1, sum = 0; i <= high; i++) {
    sum += A[i];
    if (sum > rightSum) {
      rightSum = sum;
      maxRight = i;
    }
  }

  return [maxLeft, maxRight, leftSum + rightSum];
}

// B: Ord
function maxOn<A, B>(lhs: A, rhs: A, f: (a: A) => B): A {
  if (f(lhs) < f(rhs)) {
    return rhs;
  } else {
    return lhs;
  }
}

// B: Ord
function maximumOn<A, B>(arr: A[], f: (a: A) => B): A {
  return arr.reduce((lhs, rhs) => maxOn(lhs, rhs, f));
}

function findMaximumSubarray(A: number[], low: number, high: number): Triple {
  if (low == high) {
    return [low, high, A[low]];
  } else {
    let mid = Math.floor((low + high) / 2);
    let left = findMaximumSubarray(A, low, mid);
    let right = findMaximumSubarray(A, mid + 1, high);
    let cross = findMaxCrossingSubarray(A, low, mid, high);
    return maximumOn([left, right, cross], ([low, high, sum]) => sum);
  }
}

function findMaximumSubarrayBrute(A: number[], low: number, high: number): Triple {
  let max: Triple = [0, 0, -Infinity];
  for (let i = low; i <= high; i++) {
    let sum = 0;
    for (let j = i; j <= high; j++) {
      sum += A[j];
      if (sum > max[2]) {
        max = [i, j, sum];
      }
    }
  }

  return max;
}

const n0: number = 1024;

function findMaximumSubarrayMix(A: number[], low: number, high: number): Triple {
  if (low == high) {
    return [low, high, A[low]];
  } else if (high - low < n0) {
    return findMaximumSubarrayBrute(A, low, high);
  } else {
    let mid = Math.floor((low + high) / 2);
    let left = findMaximumSubarray(A, low, mid);
    let right = findMaximumSubarray(A, mid + 1, high);
    let cross = findMaxCrossingSubarray(A, low, mid, high);
    return maximumOn([left, right, cross], ([low, high, sum]) => sum);
  }
}

function findMaximumSubArrayLinear(A: number[], low: number, high: number): Triple {
  let max: Triple = [0, 0, -Infinity];
  let tail_max: Triple = [0, 0, -Infinity];

  function updateTail(idx: number) {
    let [low, high, sum] = tail_max;
    if (sum < 0) {
      tail_max = [idx, idx, A[idx]];
    } else {
      tail_max = [low, idx, sum + A[idx]];
    }
  }

  for (let i = 0; i < A.length; i++) {
    updateTail(i);
    max = maxOn(max, tail_max, ([low, high, sum]) => sum);
  }

  return max;
}