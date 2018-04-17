export {
  greedy,
  unitTaskScheduling,
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

interface Task {
  index: number;
  deadline: number;
  weight: number;
}

function unitTaskScheduling(T: Task[]): Task[] {
  let n = T.length;
  let matroid = {
    S: T,
    weight(t: Task): number {
      return t.weight;
    },
    independent(A: Task[]): boolean {
      let D: number[] = new Array(n + 1);
      D.fill(0);
      for (let t of A) {
        let d = t.deadline;
        D[d]++;
        if (D[d] > d) {
          return false;
        }
      }
      return true;
    },
  };

  let opt = greedy(matroid);
  opt.sort((a, b) => a.deadline - b.deadline);
  return opt;
}
