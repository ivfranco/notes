export {
  Automaton,
  finiteAutomatonMatcher,
  automatonGapMatcher,
};

import { computePrefixFunction } from "./kmp";

interface Transit {
  [index: string]: number;
}

function isSuffixOf(P: string, x: string): boolean {
  let n = x.length;
  let m = P.length;

  for (let i = 0; i < m; i++) {
    if (x[n - i - 1] !== P[m - i - 1]) {
      return false;
    }
  }
  return true;
}

function extractAlphabet(P: string): string {
  let char_set: Transit = Object.create(null);
  for (let a of P) {
    char_set[a] = 0;
  }
  let alphabet = "";
  for (let a of Object.keys(char_set)) {
    alphabet += a;
  }
  return alphabet;
}

function computeTransits(P: string): Transit[] {
  let m = P.length;
  let alphabet = extractAlphabet(P);
  let trans: Transit[] = [];
  for (let i = 0; i <= m; i++) {
    trans[i] = Object.create(null);
  }
  let prefix = computePrefixFunction(P);

  for (let a of alphabet) {
    trans[0][a] = P[0] === a ? 1 : 0;
    for (let q = 1; q <= m; q++) {
      if (q === m || P[q] !== a) {
        trans[q][a] = trans[prefix[q]][a];
      } else {
        trans[q][a] = q + 1;
      }
    }
  }

  return trans;
}

class Automaton {
  private q: number;
  private m: number;
  private trans: Transit[];
  private alphabet: string;

  constructor(P: string) {
    let alphabet = extractAlphabet(P);
    let m = P.length;
    // let trans: Transit[] = [];
    // for (let q = 0; q <= m; q++) {
    //   trans[q] = Object.create(null);
    //   for (let a of alphabet) {
    //     let k = Math.min(m, q + 1);
    //     while (!isSuffixOf(P.substr(0, k), P.substr(0, q) + a)) {
    //       k--;
    //     }
    //     trans[q][a] = k;
    //   }
    // }
    this.q = 0;
    this.m = m;
    this.trans = computeTransits(P);
    this.alphabet = alphabet;
  }

  public transit(a: string) {
    let { q, trans } = this;
    // console.log(`δ(${q}, ${a}) = ${trans[q][a]}`);
    q = trans[q][a];
    if (q === undefined) {
      q = 0;
    }
    this.q = q;
  }

  public accepted(): boolean {
    return this.q === this.m;
  }

  public print() {
    let { m, alphabet, trans } = this;
    let inputs = alphabet.split("");
    inputs.sort();

    console.log(`inputs: ${inputs.join(", ")}`);
    for (let q = 0; q <= m; q++) {
      let transit = trans[q];
      console.log(`${q}:`, inputs.map(i => transit[i]));
    }
  }
}

function finiteAutomatonMatcher(T: string, P: string): number[] {
  let n = T.length;
  let m = P.length;
  let M = new Automaton(P);
  let shifts: number[] = [];
  for (let i = 0; i < n; i++) {
    M.transit(T[i]);
    if (M.accepted()) {
      shifts.push(i - m + 1);
    }
  }

  return shifts;
}

function automatonGapMatcher(T: string, P: string[]): boolean {
  let n = T.length;
  let m = P.length;
  let Ms = P.map(p => new Automaton(p));
  for (let i = 0, j = 0; i < n; i++) {
    Ms[j].transit(T[i]);
    if (Ms[j].accepted()) {
      j++;
      if (j >= m) {
        return true;
      }
    }
  }
  return false;
}
