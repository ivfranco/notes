export {
  pMergeSort,
  pPartition,
  pRandomizedSelect,
  pRandomPivoter,
  pSelect,
};

import { TreeNode } from "../collection/tree";
import { randomPivoter } from "../sort/quicksort";
import { insertionSortSlice } from "../start/insertion-sort";
import { SWAP } from "../util";
import { parallelFor } from "./p-matrix-multiply";

function binarySearch<T>(x: T, T: T[], p: number, r: number): number {
  let low = p;
  let high = Math.max(p, r + 1);

  while (low < high) {
    let mid = Math.floor((low + high) / 2);
    if (x <= T[mid]) {
      high = mid;
    } else {
      low = mid + 1;
    }
  }

  return high;
}

async function pMerge<T>(T: T[], p1: number, r1: number, p2: number, r2: number, A: T[], p3: number): Promise<void> {
  let n1 = r1 - p1 + 1;
  let n2 = r2 - p2 + 1;
  if (n1 < n2) {
    return pMerge(T, p2, r2, p1, r1, A, p3);
  }

  if (n1 === 0) {
    return;
  } else {
    let q1 = Math.floor((p1 + r1) / 2);
    let q2 = binarySearch(T[q1], T, p2, r2);
    let q3 = p3 + (q1 - p1) + (q2 - p2);
    A[q3] = T[q1];
    let handle = pMerge(T, p1, q1 - 1, p2, q2 - 1, A, p3);
    await pMerge(T, q1 + 1, r1, q2, r2, A, q3 + 1);
    await handle;
  }
}

async function pMergeSort<T>(A: T[], p: number, r: number, B: T[], s: number) {
  let n = r - p + 1;
  if (n === 1) {
    B[s] = A[p];
  } else {
    let C: T[] = [];
    let q = Math.floor((p + r) / 2);
    let t = q - p;
    let handle = pMergeSort(A, p, q, C, 0);
    await pMergeSort(A, q + 1, r, C, t + 1);
    await handle;
    await pMerge(C, 0, t, t + 1, n - 1, B, s);
  }
}

type Pair = [number, number];

function pairSum(a: Pair, b: Pair): Pair {
  let [a1, a2] = a;
  let [b1, b2] = b;
  return [a1 + b1, a2 + b2];
}

async function constrTrie<T>(
  A: T[], p: number, r: number, prefix: number, height: number, pivot: T,
): Promise<TreeNode<Pair>> {
  if (height === 0) {
    if (prefix + p <= r) {
      let key: Pair = A[prefix + p] <= pivot ? [1, 0] : [0, 1];
      return new TreeNode(key);
    } else {
      return new TreeNode([0, 0] as Pair);
    }
  } else {
    let handle = constrTrie(A, p, r, prefix * 2, height - 1, pivot);
    let right_child = await constrTrie(A, p, r, prefix * 2 + 1, height - 1, pivot);
    let left_child = await handle;
    //  the key of an internal node is the number of items less or equal to and greater than the pivot in the subtree
    let node = new TreeNode(pairSum(left_child.key, right_child.key));
    node.left = left_child;
    node.right = right_child;
    return node;
  }
}

async function pAssign<T>(trie: TreeNode<Pair>, A: T[], p: number, r: number, prefix: number, sum: Pair, T: T[]) {
  if (trie.isLeaf()) {
    let [le, gt] = trie.key;
    if (le === 1) {
      //  the current item is smaller or equal to the pivot
      //  assigned from left to right into the target array T
      //  sum[0] items before A[prefix] is smaller or equal to pivot, A[prefix] is placed at T[sum[0]]
      T[sum[0]] = A[prefix + p];
    } else if (gt === 1) {
      //  the current item is greater than the pivot
      //  assigned from right to left into the target array T
      //  sum[1] items before A[prefix] is greater than pivot, A[prefix] is placed at T[last - sum[1]]
      let last = r - p;
      T[last - sum[1]] = A[prefix + p];
    }
    //  otherwise prefix is greater than A.length - 1, no item in A corresponds to the node
  } else {
    //  the trie is complete, any node that's not a leaf has two children
    let left_child = trie.left as TreeNode<Pair>;
    let right_child = trie.right as TreeNode<Pair>;
    let handle = pAssign(left_child, A, p, r, prefix * 2, sum, T);
    await pAssign(right_child, A, p, r, prefix * 2 + 1, pairSum(sum, left_child.key), T);
    await handle;
  }
}

function trieHeight(n: number): number {
  return Math.ceil(Math.log2(Math.max(n - 1, 1))) + 1;
}

type PPivoter<T> = (A: T[], p: number, r: number) => Promise<number>;

async function pPartition<T>(A: T[], p: number, r: number, pivoter: PPivoter<T>, T: T[]): Promise<number> {
  let pivot_idx = await pivoter(A, p, r);
  let pivot = A[pivot_idx];
  SWAP(A, pivot_idx, r);
  let trie = await constrTrie(A, p, r, 0, trieHeight(r - p + 1), pivot);
  await pAssign(trie, A, p, r, 0, [0, 0], T);
  let [le, gt] = trie.key;
  return le - 1;
}

async function pRandomPivoter<T>(A: T[], p: number, r: number): Promise<number> {
  return randomPivoter(A, p, r);
}

async function pRandomizedSelect<T>(A: T[], p: number, r: number, i: number): Promise<T> {
  if (p === r) {
    return A[p];
  } else {
    let B: T[] = [];
    let q = await pPartition(A, p, r, pRandomPivoter, B);
    let k = q + 1;
    if (i === k) {
      return B[q];
    } else if (i < k) {
      return pRandomizedSelect(B, 0, q - 1, i);
    } else {
      return pRandomizedSelect(B, q + 1, B.length - 1, i - k);
    }
  }
}

const SLICE_LEN = 5;
const SLICE_MEDIAN = Math.floor(SLICE_LEN / 2);

async function pIndexOf<T>(A: T[], x: T, p: number, r: number): Promise<number> {
  let idx: number | null = null;
  await parallelFor(p, r, async i => {
    if (A[i] === x) {
      idx = i;
    }
  });
  return idx === null ? -1 : idx;
}

async function pmmPivoter<T>(A: T[], p: number, r: number): Promise<number> {
  let iters = Math.floor((r - p + 1) / SLICE_LEN);
  let medians: T[] = [];
  await parallelFor(0, iters - 1, async i => {
    let off = SLICE_LEN * i + p;
    insertionSortSlice(A, off, off + SLICE_LEN - 1);
    medians[i] = A[off + SLICE_MEDIAN];
  });
  let j = iters * SLICE_LEN + p;
  if (j < r) {
    insertionSortSlice(A, j, r);
    medians.push(A[Math.floor((j + r) / 2)]);
  }

  let n = medians.length;
  let x = await pSelect(medians, 0, n - 1, Math.floor(n / 2));

  let idx = await pIndexOf(A, x, p, r);
  if (idx < 0) {
    throw Error("Error: supposed median of medians does not exist in original array");
  } else {
    return idx;
  }
}

async function pSelect<T>(A: T[], p: number, r: number, i: number): Promise<T> {
  if (p === r) {
    return A[p];
  } else {
    let B: T[] = [];
    let q = await pPartition(A, p, r, pmmPivoter, B);
    let k = q + 1;
    if (i === k) {
      return B[q];
    } else if (i < k) {
      return pSelect(B, 0, q - 1, i);
    } else {
      return pSelect(B, q + 1, B.length - 1, i - k);
    }
  }
}
