export {
  greedy,
};

interface Matroid<T> {
  S: T[];
  weight(a: T): number;
  independent(A: T[]): boolean;
}

function greedy<T>(M: Matroid<T>): T[] {
  let S = M.S.slice();
  S.sort((a, b) => M.weight(b) - M.weight(a));

  let A: T[] = [];
  for (let a of S) {
    A.push(a);
    if (!M.independent(A)) {
      A.pop();
    }
  }

  return A;
}
