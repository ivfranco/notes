import { maximumOn } from "../util"

export {
  greedySetCover,
  linearSetCover,
  isCovered,
}

function greedySetCover<T>(X: T[], F: Array<T[]>): Array<T[]> {
  let U = new Set(X);
  let C: Array<T[]> = [];

  while (U.size > 0) {
    let S = maximumOn(F, S => {
      let count = 0;
      for (let s of S) {
        if (U.has(s)) {
          count += 1;
        }
      }
      return count;
    });

    for (let s of S) {
      U.delete(s);
    }

    C.push(S);
  }

  return C;
}

interface Wrapper<T> {
  set: T[],
  size: number,
}

// assume the elements of X can be mapped to numbers in range [0, |X| - 1]
// it's always possible by assigning numbers to elements of X
function linearSetCover<T>(X: T[], F: T[][], f: (t: T) => number): T[][] {
  let W: Wrapper<T>[] = F.map(S => {
    return { set: S, size: S.length };
  });

  // an array of subsets indexed by elements of X
  let D: Wrapper<T>[][] = [];
  for (let i = 0; i < X.length; i++) {
    D[i] = [];
  }
  // S ∈ D[s] iff s ∈ S
  for (let w of W) {
    for (let s of w.set) {
      D[f(s)].push(w);
    }
  }

  // an array of arrays of subsets indexed by their size
  let max_size = W.map(w => w.size).reduce((a, b) => Math.max(a, b));
  let L: Wrapper<T>[][] = [];
  for (let size = 0; size <= max_size; size++) {
    L[size] = [];
  }
  for (let w of W) {
    L[w.size].push(w);
  }

  let C: T[][] = [];

  for (let size = max_size; size > 0; size--) {
    for (let w of L[size]) {
      if (w.size === size) {
        C.push(w.set);
        for (let s of w.set) {
          // each most inner loop decreases size of a set by 1
          // total running time is O(Σ|S|, S ∈ F)
          for (let v of D[f(s)]) {
            v.size--;
          }
          D[f(s)] = [];
        }
      } else {
        L[w.size].push(w);
      }
    }
  }

  return C;
}

function isCovered<T>(X: T[], C: T[][]): boolean {
  let set = new Set(X);

  for (let S of C) {
    for (let s of S) {
      set.delete(s);
    }
  }

  return set.size === 0;
}