export {
  optimalBST,
  constructOptimalBST,
  quadraticOptimalBST,
};

import { TreeNode } from "../collection/tree";

function optimalBST(p: number[], q: number[], n: number): [number, number[][]] {
  let e: number[][] = new Array(n + 2);
  let w: number[][] = new Array(n + 2);
  let root: number[][] = [];
  for (let i = 1; i <= n + 1; i++) {
    e[i] = [];
    w[i] = [];
    e[i][i - 1] = q[i - 1];
    w[i][i - 1] = q[i - 1];
  }
  for (let i = 1; i <= n; i++) {
    root[i] = [];
    root[i][i] = i;
  }

  for (let l = 1; l <= n; l++) {
    for (let i = 1; i + l - 1 <= n; i++) {
      let j = i + l - 1;
      e[i][j] = +Infinity;
      w[i][j] = w[i][j - 1] + p[j] + q[j];
      for (let r = i; r <= j; r++) {
        let t = e[i][r - 1] + e[r + 1][j] + w[i][j];
        if (t < e[i][j]) {
          e[i][j] = t;
          root[i][j] = r;
        }
      }
    }
  }

  return [e[1][n], root];
}

function constructOptimalBST(root: number[][], n: number): TreeNode<string> {
  function aux(i: number, j: number): TreeNode<string> | null {
    if (i > n) {
      return null;
    }

    let r = root[i][j];
    if (r == null) {
      return null;
    } else {
      let left = aux(i, r - 1);
      if (!left) {
        left = new TreeNode(`d${r - 1}`);
      }
      let right = aux(r + 1, j);
      if (!right) {
        right = new TreeNode(`d${r}`);
      }
      let node = new TreeNode(`k${r}`);
      node.left = left;
      node.right = right;
      return node;
    }
  }

  return aux(1, n) as TreeNode<string>;
}

function quadraticOptimalBST(p: number[], q: number[], n: number): [number, number[][]] {
  let e: number[][] = new Array(n + 2);
  let w: number[][] = new Array(n + 2);
  let root: number[][] = [];
  for (let i = 1; i <= n + 1; i++) {
    e[i] = [];
    w[i] = [];
    e[i][i - 1] = q[i - 1];
    w[i][i - 1] = q[i - 1];
  }
  for (let i = 1; i <= n; i++) {
    root[i] = [];
    root[i][i] = i;
  }

  for (let l = 1; l <= n; l++) {
    for (let i = 1; i + l - 1 <= n; i++) {
      let j = i + l - 1;
      w[i][j] = w[i][j - 1] + p[j] + q[j];
      if (i === j) {
        e[i][j] = e[i][i - 1] + e[i + 1][i] + w[i][i];
        root[i][i] = i;
      } else {
        e[i][j] = +Infinity;
        for (let r = root[i][j - 1]; r <= root[i + 1][j]; r++) {
          let t = e[i][r - 1] + e[r + 1][j] + w[i][j];
          if (t < e[i][j]) {
            e[i][j] = t;
            root[i][j] = r;
          }
        }
      }
    }
  }

  return [e[1][n], root];
}
