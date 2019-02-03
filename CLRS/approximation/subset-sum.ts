export {
  exactSubsetSum,
  approxSubsetSum,
}

interface Subset {
  set: number[],
  sum: number,
}

function add(S: Subset, x: number): Subset {
  let set = S.set.slice();
  set.push(x);
  return {
    set,
    sum: S.sum + x,
  }
}

// L and R are consumed by this procedure
function mergeLists(L: Subset[], R: Subset[]): Subset[] {
  let ret: Subset[] = [];
  let inf: Subset = {
    set: [],
    sum: +Infinity,
  };
  L.push(inf);
  R.push(inf);
  let i = 0;
  let j = 0;

  while (L[i].sum !== +Infinity || R[j].sum !== +Infinity) {
    if (L[i].sum > R[j].sum) {
      ret.push(R[j]);
      j++;
    } else {
      ret.push(L[i]);
      i++;
    }
  }

  return dedup(ret);
}

function dedup(L: Subset[]): Subset[] {
  let ret: Subset[] = [L[0]];

  for (let i = 1; i < L.length; i++) {
    if (L[i].sum !== L[i - 1].sum) {
      ret.push(L[i]);
    }
  }

  return ret;
}

function exactSubsetSum(S: number[], t: number): Subset {
  let n = S.length;
  let L: Subset[] = [{
    set: [],
    sum: 0,
  }];

  for (let i = 0; i < n; i++) {
    let x = S[i];
    L = mergeLists(L, L.map(S => add(S, x)))
      .filter(S => S.sum <= t);
  }

  return L.reverse().find(C => C.sum <= t) as Subset;
}

function trim(L: Subset[], delta: number): Subset[] {
  let ret = [L[0]];
  let last = L[0].sum;

  for (let i = 1; i < L.length; i++) {
    if (L[i].sum > last * (1 + delta)) {
      ret.push(L[i]);
      last = L[i].sum;
    }
  }

  return ret;
}

function approxSubsetSum(S: number[], t: number, epsilon: number): Subset {
  let n = S.length;
  let L: Subset[] = [{
    set: [],
    sum: 0,
  }];

  for (let i = 0; i < n; i++) {
    let x = S[i];
    L = mergeLists(L, L.map(S => add(S, x))).filter(S => S.sum <= t);
    L = trim(L, epsilon / (2 * n));
  }
  return L.reverse().find(S => S.sum <= t) as Subset;
}