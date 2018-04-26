export {
  offlineMinimum,
};

import { DList, DNode } from "../collection/dlist";
import { DSTreeNode } from "./disjoint-set-forest";

function preprocess(input: string): number[][] {
  return input
    .split("E")
    .map(I => I.split(" ").filter(s => s !== "").map(i => parseInt(i, 10)));
}

type Set = DSTreeNode<DNode<number>>;

function merge(i: Set, j: Set, dlist: DList<number>) {
  let i_root = i.findSet();
  let j_root = j.findSet();
  i.union(j);
  //  the root linked to the other is deleted from the linked root list
  if (i.findSet() === i_root) {
    dlist.delete(j_root.key);
  } else {
    dlist.delete(i_root.key);
  }
  //  set the number at the root to the maximum between the two
  //  each node x in Ki ∪ Kj now has x.findSet().key.key == j assuming i <= j
  i.findSet().key.key = Math.max(i_root.key.key, j_root.key.key);
}

function offlineMinimum(input: string, n: number, m: number): number[] {
  //  a reference array, maps numerical keys to nodes in disjoint set forest
  let elems: Set[] = new Array(n);
  //  a list of roots described by their set number defined below
  let root_list: DList<number> = new DList();
  //  a reference array, maps set number (indices) to a node in that disjoint set tree
  //  sets[i].findSet().key.key == i as long as extracted[i] is not assigned
  let sets: Set[] = new Array(m);

  preprocess(input).forEach((I, j) => {
    if (I.length !== 0) {
      let root_node = new DNode(j);
      //  O(1)
      root_list.append(root_node);
      let head = new DSTreeNode(root_node);
      elems[I[0]] = head;
      for (let i = 1; i < I.length; i++) {
        let elem = new DSTreeNode(root_node);
        //  all element in sequence Ij is unioned into a single set at the beginning
        //  O(α(|Ij|)|Ij|) for each sequence, O(α(n) * n) in total
        head.union(elem);
        elems[I[i]] = elem;
      }
      sets[j] = head;
    } else {
      let root_node = new DNode(j);
      //  O(1)
      root_list.append(root_node);
      //  when Ij is empty, a dummy set is put in sets[j]
      //  initially no element in elems is in the same tree with sets[j]
      //  it still can be merged with other sets, empty or not
      let dummy_set = new DSTreeNode(root_node);
      sets[j] = dummy_set;
    }
  });

  let extracted: number[] = new Array(m);

  for (let i = 0; i < n; i++) {
    //  O(n) FIND-SET, O(α(n) * n) in total
    let root_node = elems[i].findSet().key;
    let j = root_node.key;
    if (j !== m) {
      extracted[j] = i;
      let l = (root_node.next as DNode<number>).key;
      //  each call to merge destroys a set
      //  at most m times assuming the input is valid
      merge(elems[i], sets[l], root_list);
    }
  }

  return extracted;
}
