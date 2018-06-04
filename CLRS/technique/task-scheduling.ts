export {
  unitTaskScheduling,
  unitTaskScheduling2,
};

import { DSTreeNode } from "../structure/disjoint-set-forest";
import { greedy } from "./matroid";

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
      }
      for (let i = 1; i <= n; i++) {
        D[i] += D[i - 1];
      }
      for (let t of A) {
        let d = t.deadline;
        if (D[d] > d) {
          return false;
        }
      }
      return true;
    },
  };

  let opt = greedy(matroid);
  opt.sort((a, b) => a.deadline - b.deadline);
  //  returns the set of tasks scheduled before their deadline
  return opt;
}

class LastSlotNode extends DSTreeNode<number> {
  //  last points to the last empty slot with last <= this.key
  //  last = null if all previous slots are filled
  public last: number | null;

  constructor(key: number, last: number) {
    super(key);
    this.last = last;
  }
}

function fill(last_empty: LastSlotNode[], i: number) {
  let rep = last_empty[i].findSet();
  if (i === 0) {
    rep.last = null;
  } else {
    let last_rep = last_empty[i - 1].findSet();
    rep.last = last_rep.last;
    last_rep.union(rep);
  }
}

function unitTaskScheduling2(T: Task[]): Task[] {
  let n = T.length;
  let last_empty: LastSlotNode[] = [];
  for (let i = 0; i < n; i++) {
    last_empty[i] = new LastSlotNode(i, i);
  }

  let scheduled: Task[] = [];
  for (let i = 0; i < n; i++) {
    //  deadline is 1-based, d is 0-based index
    let d = T[i].deadline - 1;
    let last = last_empty[d].findSet().last;
    if (last === null) {
      last = last_empty[n - 1].findSet().last as number;
    }
    fill(last_empty, last);
    scheduled[last] = T[i];
  }

  return scheduled.filter((t, i) => t.deadline >= i + 1);
}
