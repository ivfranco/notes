import { Queue } from "./queue";
import { SList, SNode } from "./slist";
import { DList, DNode } from "./dlist";

function main() {
  problem_10_2_7();
}

function problem_10_1_1() {
  let S = [];
  S.push(4);
  console.log(S);
  S.push(1);
  console.log(S);
  S.push(3);
  console.log(S);
  console.log("Pop: ", S.pop());
  console.log(S);
  S.push(8);
  console.log(S);
  console.log("Pop: ", S.pop());
  console.log(S);
}

function problem_10_1_3() {
  let Q = new Queue(6);

  Q.enqueue(4);
  Q.report();
  Q.enqueue(1);
  Q.report();
  Q.enqueue(3);
  Q.report();
  console.log("Dequeue: ", Q.dequeue());
  Q.report();
  Q.enqueue(8);
  Q.report();
  console.log("Dequeue: ", Q.dequeue());
  Q.report();
}

function problem_10_2_6() {
  let dlist = new DList();
  for (let i = 0; i < 10; i++) {
    dlist.insert(i);
  }
  console.log(dlist.show());
  for (let i = 0; i < 10; i++) {
    let node = dlist.search(i);
    if (node !== null) {
      dlist.delete(node);
    }
  }
  console.log(dlist.show());

  let dlist_a = DList.fromArray([1, 2, 3, 4, 5]);
  let dlist_b = DList.fromArray([6, 7, 8, 9, 10]);
  dlist_a.concat(dlist_b);
  console.log(dlist_a.show());
}

function problem_10_2_7() {
  let A = [1, 2, 3, 4, 5];
  let slist = new SList();
  for (let a of A) {
    slist.insert(a);
  }
  slist.reverse();
  console.log(slist.show());
}

main();