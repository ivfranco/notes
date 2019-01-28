export {
  Task,
  taskScheduling,
};

interface Task {
  time: number,
  profit: number,
  deadline: number,
}

function taskScheduling(tasks: Task[]): number {
  tasks.sort((a, b) => a.deadline - b.deadline);

  let n = tasks.length;
  let profits: number[][] = [];

  for (let i = 0; i <= n; i++) {
    profits[i] = [];
  }

  function aux(i: number, t: number): number {
    if (profits[i][t] !== undefined) {
      return profits[i][t];
    }

    let {time, profit, deadline} = tasks[i];
    let ret;

    if (i >= n) {
      ret = 0;
    } else if (t + time > deadline) {
      ret = aux(i + 1, t);
    } else {
      let taken = profit + aux(i + 1, t + time);
      let notTaken = aux(i + 1, t);
      ret = Math.max(taken, notTaken);
    }

    profits[i][t] = ret;
    return ret;
  }

  return aux(0, 0);
}