export {
  match,
  naiveStringMatcher,
  gapStringMatcher,
};

function match(T: string, P: string, off: number): boolean {
  let m = P.length;
  for (let i = 0; i < m; i++) {
    if (T[i + off] !== P[i]) {
      // console.log(`compared ${P.substr(0, i + 1)} to the text at offset ${off}`);
      return false;
    }
  }
  // console.log(`compared ${P} to the text at offset ${off}`);
  return true;
}

function naiveStringMatcher(T: string, P: string): number[] {
  let shifts: number[] = [];
  let n = T.length;
  let m = P.length;
  for (let s = 0; s <= n - m; s++) {
    if (match(T, P, s)) {
      shifts.push(s);
    }
  }
  return shifts;
}

function gapStringMatcher(T: string, P: string[]): boolean {
  let off = T.length;
  for (let i = P.length - 1; i >= 0; i--) {
    let S = P[i];
    off -= S.length;
    //  .lastIndexOf will set the second argument to 0 if it's negative
    //  the negativity of `off` must be checked beforehand
    if (off < 0) {
      return false;
    }
    off = T.lastIndexOf(S, off);
    if (off < 0) {
      return false;
    }
  }

  return true;
}
