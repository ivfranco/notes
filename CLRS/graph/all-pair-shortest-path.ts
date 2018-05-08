export {
  slowAllPairsShortestPaths,
  fasterAllPairsShortestPaths,
  fromDirectedGraph,
  predecessorMatrix,
  floydWarshall,
  showShortestPath,
};

import { Graph, Vertex } from "./directed-graph";
import { WeightedEdge } from "./weighted-graph";

type Vtx = number;
type Weight = number;
type Matrix<V> = V[][];
type PredMatrix = Matrix<Vtx | null>;

function fromDirectedGraph(G: Graph<Vertex, WeightedEdge<Vertex>>): Matrix<Weight> {
  let W: Matrix<Weight> = [];
  let n = G.size();
  for (let i = 0; i < n; i++) {
    W[i] = [];
    for (let j = 0; j < n; j++) {
      W[i][j] = +Infinity;
    }
  }

  let Idx: number[] = new Array(n);
  let idx = 0;
  for (let v of G.vertices()) {
    Idx[v.key] = idx;
    idx++;
  }

  for (let { from: u, to: v, weight: w } of G.edges()) {
    let i = Idx[u.key];
    let j = Idx[v.key];
    W[i][j] = w;
  }

  return W;
}

function initLP(n: number): [Matrix<Weight>, PredMatrix] {
  let L: Matrix<Weight> = [];
  let P: Matrix<Vtx | null> = [];
  for (let i = 0; i < n; i++) {
    L[i] = [];
    P[i] = [];
    for (let j = 0; j < n; j++) {
      if (i === j) {
        L[i][j] = 0;
      } else {
        L[i][j] = +Infinity;
      }
      P[i][j] = null;
    }
  }
  return [L, P];
}

function extendShortestPaths(
  L: Matrix<Weight>, P: Matrix<Vtx | null>, W: Matrix<Weight>,
): [Matrix<Weight>, PredMatrix] {
  let n = L.length;
  let [ret_L, ret_P] = initLP(n);
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
      ret_P[i][j] = P[i][j];
      for (let k = 0; k < n; k++) {
        let l = L[i][k] + W[k][j];
        if (ret_L[i][j] > l) {
          ret_L[i][j] = l;
          ret_P[i][j] = k;
        }
      }
    }
  }
  return [ret_L, ret_P];
}

function slowAllPairsShortestPaths(W: Matrix<Weight>): [Matrix<Weight>, PredMatrix] {
  let n = W.length;
  let [L, P] = initLP(n);
  for (let m = 1; m < n; m++) {
    [L, P] = extendShortestPaths(L, P, W);
  }
  return [L, P];
}

function fasterAllPairsShortestPaths(W: Matrix<Weight>): Matrix<Weight> {
  let n = W.length;
  let [L, P] = initLP(n);
  [L] = extendShortestPaths(L, P, W);
  for (let m = 1; m < n - 1; m *= 2) {
    [L] = extendShortestPaths(L, P, L);
  }

  let negative_cycle = false;
  let [T] = extendShortestPaths(L, P, L);
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
      if (T[i][j] !== L[i][j]) {
        negative_cycle = true;
      }
    }
  }

  if (negative_cycle) {
    console.warn("Warning: negative cycle detected");
  }

  return L;
}

//  return true if j is an ancestor of k on Gπ,i
function ancestorCheck(P: PredMatrix, i: Vtx, j: Vtx, k: Vtx | null): boolean {
  //  P[i][k] may also be undefined, != null checks both null and undefined
  while (k != null) {
    if (k === j) {
      return true;
    } else {
      k = P[i][k];
    }
  }
  return false;
}

function updateOnCycle(P: Matrix<Vtx | null>, i: Vtx, j: Vtx, k: Vtx, onCycle: boolean[]) {
  while (k !== j) {
    onCycle[k] = true;
    k = P[i][k] as Vtx;
  }
}

function predecessorMatrix(L: Matrix<Weight>, W: Matrix<Weight>): PredMatrix {
  let n = L.length;
  let P: Matrix<Vtx | null> = [];
  for (let i = 0; i < n; i++) {
    P[i] = [];
    for (let j = 0; j < n; j++) {
      if (i === j || L[i][j] === +Infinity) {
        P[i][j] = null;
      } else {
        let onCycle: boolean[] = new Array(n);
        onCycle.fill(false);
        onCycle[j] = true;
        for (let k = 0; k < n; k++) {
          if (!onCycle[k] && L[i][k] + W[k][j] === L[i][j]) {
            if (ancestorCheck(P, i, j, k)) {
              updateOnCycle(P, i, j, k, onCycle);
            } else {
              P[i][j] = k;
              break;
            }
          }
        }
      }
    }
  }
  return P;
}

function floydWarshall(W: Matrix<Weight>): [Matrix<Weight>, PredMatrix] {
  let n = W.length;
  let [D, P] = initLP(n);
  [D, P] = extendShortestPaths(D, P, W);
  // console.log("Before first iteration, D:");
  // console.log(D);
  for (let k = 0; k < n; k++) {
    let Q: PredMatrix = [];
    for (let i = 0; i < n; i++) {
      Q[i] = [];
      for (let j = 0; j < n; j++) {
        let d = D[i][k] + D[k][j];
        if (D[i][j] >= d) {
          D[i][j] = d;
          Q[i][j] = P[k][j];
        } else {
          D[i][j] = D[i][j];
          Q[i][j] = P[i][j];
        }
      }
    }
    P = Q;
    // console.log(`Iteration ${k}, D${k}:`);
    // console.log(D);
  }

  return [D, P];
}

function showShortestPath(P: PredMatrix, i: Vtx, j: Vtx): string {
  if (i === j) {
    return `0-length edge ${i}`;
  } else if (P[i][j] === null) {
    return `No path from ${i} to ${j} in the graph`;
  } else {
    let path = "";
    while (j !== i) {
      path = ` -> ${j}` + path;
    }
    path = `${i}` + path;
    return path;
  }
}
