export {
  computePrefixFunction,
  kmpMatcher,
  isCyclic,
};

function computePrefixFunction(P: string): number[] {
  let m = P.length;
  let prefix: number[] = [];
  prefix[1] = 0;
  let k = 0;
  for (let q = 2; q <= m; q++) {
    while (k > 0 && P[k] !== P[q - 1]) {
      k = prefix[k];
    }
    if (P[k] === P[q - 1]) {
      k++;
    }
    prefix[q] = k;
  }

  return prefix;
}

function compress(P: string, prefix: number[]): number[] {
  let m = P.length;
  let compressed: number[] = [];
  for (let q = 1; q <= m; q++) {
    if (prefix[q] === 0) {
      compressed[q] = 0;
    } else if (P[prefix[q]] === P[q]) {
      compressed[q] = compressed[prefix[q]];
    } else {
      compressed[q] = prefix[q];
    }
  }
  return compressed;
}

function kmpMatcher(T: string, P: string): number[] {
  let n = T.length;
  let m = P.length;
  let prefix = computePrefixFunction(P);
  let compressed = compress(P, prefix);

  let q = 0;
  let shifts: number[] = [];
  for (let i = 0; i < n; i++) {
    while (q > 0 && P[q] !== T[i]) {
      q = compressed[q];
    }
    if (P[q] === T[i]) {
      q++;
    }
    if (q === m) {
      shifts.push(i - m + 1);
      q = prefix[q];
    }
  }

  return shifts;
}

function isCyclic(T1: string, T2: string): boolean {
  return T1.length === T2.length && kmpMatcher(T1, T2 + T2).length > 0;
}
