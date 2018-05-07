export {
  spBellmanFord,
  spDijkstra,
  spReport,
  spDag,
  spGabow,
  dijkstraCheck,
  minimumMeanWeightCycle,
};

import { DList, DNode } from "../collection/dlist";
import { AbstractFHeap, FHeapNode } from "../structure/fibonacci-heap";
import { Color, DFS, Edge, EdgeType, Graph, PlainGraph, topologicalSort, Vertex } from "./directed-graph";
import { WeightedEdge } from "./weighted-graph";

interface SPAttrs<V, E> {
  d: number;
  p: V | null;
  e: E | null;
}

function initialize<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>, s: Vertex): Array<SPAttrs<V, E>> {
  let attrs: Array<SPAttrs<V, E>> = [];
  for (let v of G.vertices()) {
    attrs[v.key] = {
      d: +Infinity,
      p: null,
      e: null,
    };
  }
  attrs[s.key].d = 0;
  return attrs;
}

//  return true if v.d is updated
function relax<V extends Vertex, E extends WeightedEdge<V>>(
  e: E, attrs: Array<SPAttrs<V, E>>, w?: number[],
): boolean {
  let { from: u, to: v, weight } = e;
  if (w) {
    weight = w[e.key];
  }
  let ua = attrs[u.key];
  let va = attrs[v.key];
  if (va.d > ua.d + weight) {
    va.d = ua.d + weight;
    va.p = u;
    va.e = e;
    return true;
  } else {
    return false;
  }
}

function spBellmanFord<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>, s: V): Array<SPAttrs<V, E>> {
  let attrs = initialize(G, s);
  for (let i = 0, size = G.size(); i < size - 1; i++) {
    let updated = false;
    for (let e of G.edges()) {
      //  relax here must be on lhs of || operator
      //  otherwise it will not be executed when updated == true
      updated = relax(e, attrs) || updated;
    }
  }

  let negative_cycle = false;
  for (let i = 0, size = G.size(); i < size; i++) {
    for (let { from: u, to: v, weight: w } of G.edges()) {
      let ua = attrs[u.key];
      let va = attrs[v.key];
      if (va.d > ua.d + w) {
        va.d = -Infinity;
        ua.d = -Infinity;
        negative_cycle = true;
      }
    }
  }

  if (negative_cycle) {
    console.warn("Warning: graph contains a negative cycle");
  }

  return attrs;
}

function spReport(G: Graph<Vertex, WeightedEdge<Vertex>>, attrs: Array<SPAttrs<Vertex, WeightedEdge<Vertex>>>) {
  for (let v of G.vertices()) {
    let va = attrs[v.key];
    let name = v.name;
    let parent = va.p ? va.p.name : "NIL";
    console.log(`Vertex ${name}: d = ${va.d}, π = ${parent}`);
  }
}

function spDag<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>, s: V): Array<SPAttrs<V, E>> {
  let attrs = initialize(G, s);
  for (let u of topologicalSort(G)) {
    for (let e of G.edgeFrom(u)) {
      relax(e, attrs);
    }
  }

  return attrs;
}

class SPNode<V extends Vertex> extends FHeapNode<number, V> {
  protected nodeStringify(): string {
    let name = this.value.name;
    let key = this.key === +Infinity ? "∞" : this.key;
    return `${name}:${key}`;
  }
}

class SPHeap<V extends Vertex> extends AbstractFHeap<number, V, SPNode<V>> {
  protected cmp(a: number, b: number) {
    return a < b;
  }

  protected factory(k: number, v: V): SPNode<V> {
    return new SPNode(k, v);
  }
}

function spDijkstra<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>, s: V): Array<SPAttrs<V, E>> {
  let attrs = initialize(G, s);
  let Q = new SPHeap<V>();
  // let S: V[] = [];
  let N: Array<SPNode<V>> = [];
  for (let v of G.vertices()) {
    let node = Q.insert(attrs[v.key].d, v);
    N[v.key] = node;
  }

  while (Q.n > 1) {
    let u = (Q.extractMin() as SPNode<V>).value;
    // S.push(u);
    // console.log(`{${S.map(v => v.name).join(", ")}}`);
    for (let e of G.edgeFrom(u)) {
      if (relax(e, attrs)) {
        let v = e.to;
        let node = N[v.key];
        Q.decreaseKey(node, attrs[v.key].d);
      }
    }
  }

  return attrs;
}

type PlainSPAttrs = SPAttrs<Vertex, WeightedEdge<Vertex>>;

function treenessCheck(G: Graph<Vertex, Edge<Vertex>>, s: Vertex, attrs: PlainSPAttrs[]) {
  let H = new PlainGraph();
  let [reachable] = (new DFS(G)).runFrom(s);
  for (let v of G.vertices()) {
    if (reachable[v.key].color === Color.BLACK) {
      //  H contains only vertices reachable from s in G
      H.createVertex(v.name);
    }
  }
  let V = H.vertexMap();
  for (let v of G.vertices()) {
    if (attrs[v.key].d === +Infinity) {
      console.assert(V[v.name] === undefined, "Unreachable vertices have to be unreachable in original graph");
    }
    let u = attrs[v.key].p;
    if (u) {
      console.assert(V[v.name] !== undefined, "Only vertices reachable from s may have parent in predecessor graph");
      console.assert(V[u.name] !== undefined, "Only vertices reachable from s may be parent in predecessor graph");
      H.createEdge(V[u.name], V[v.name]);
    }
  }
  let dfs = new DFS(H);
  let [v_attr, e_attr] = dfs.runFrom(V[s.name]);
  console.assert(v_attr.every(a => a.color === Color.BLACK), "All vertices must be reachable from s");
  console.assert(e_attr.every(t => t === EdgeType.TREE), "The predecessor graph must be a tree");
}

function dijkstraCheck(G: Graph<Vertex, WeightedEdge<Vertex>>, s: Vertex, attrs: PlainSPAttrs[]) {
  treenessCheck(G, s, attrs);
  for (let v of G.vertices()) {
    let va = attrs[v.key];
    if (v !== s && va.d !== +Infinity) {
      let { p, e } = va;
      console.assert(p !== null && e !== null, "Non-root vertices with finite estimate must have a parent");
      let w = attrs[(p as Vertex).key].d + (e as WeightedEdge<Vertex>).weight;
      console.assert(va.d === w, "estimates should correspond to real path weights");
    }
  }

  let updated = false;
  for (let e of G.edges()) {
    updated = relax(e, attrs) || updated;
  }

  console.assert(!updated, "One pass of BELLMAN-FORD should not update an optimal result");
}

function linearInitialize<V extends Vertex, E extends WeightedEdge<V>>(
  G: Graph<V, E>, s: Vertex,
): Array<SPAttrs<V, E>> {
  let attrs: Array<SPAttrs<V, E>> = [];
  let e = G.edgeSize();
  for (let v of G.vertices()) {
    attrs[v.key] = {
      d: e + 1,
      p: null,
      e: null,
    };
  }
  attrs[s.key].d = 0;
  return attrs;
}

//  assuming δ(s, v) <= |G.E| for all v ∈ G.V
function linearDijkstra<V extends Vertex, E extends WeightedEdge<V>>(
  G: Graph<V, E>, s: V, w: number[],
): Array<SPAttrs<V, E>> {
  let attrs = linearInitialize(G, s);
  let N: Array<DNode<V>> = [];
  let Q: Array<DList<V>> = [];
  for (let i = 0, e = G.edgeSize(); i <= e + 1; i++) {
    Q[i] = new DList();
  }
  for (let v of G.vertices()) {
    let node = new DNode(v);
    N[v.key] = node;
    let d = attrs[v.key].d;
    Q[d].append(node);
  }

  for (let i = 0, min_d = 0, size = G.size(); i < size; i++) {
    while (Q[min_d].isEmpty()) {
      min_d++;
    }

    let u_node = Q[min_d].head as DNode<V>;
    Q[min_d].delete(u_node);
    let u = u_node.key;
    for (let e of G.edgeFrom(u)) {
      let v = e.to;
      let v_node = N[v.key];
      let old_d = attrs[v.key].d;
      if (relax(e, attrs, w)) {
        let new_d = attrs[v.key].d;
        Q[old_d].delete(v_node);
        Q[new_d].append(v_node);
      }
    }
  }

  return attrs;
}

function initWeightFunction(G: Graph<Vertex, WeightedEdge<Vertex>>, k: number, i: number, D: number[]): number[] {
  let w: number[] = [];
  for (let { key, weight, from: u, to: v } of G.edges()) {
    w[key] = Math.floor(weight / (2 ** (k - i))) + 2 * D[u.key] - 2 * D[v.key];
  }
  return w;
}

function spGabow<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>, s: V): Array<SPAttrs<V, E>> {
  let W = Math.max(...Array.from(G.edges()).map(e => e.weight));
  let k = Math.ceil(Math.log2(W + 1));
  //  shortest path weights according to weight function in the last iteration
  let D = new Array(G.size());
  D.fill(0);
  let attrs: Array<SPAttrs<V, E>> = [];

  for (let i = 1; i <= k; i++) {
    let w = initWeightFunction(G, k, i, D);
    attrs = linearDijkstra(G, s, w);
    for (let v of G.vertices()) {
      D[v.key] = attrs[v.key].d + 2 * D[v.key];
    }
  }

  for (let v of G.vertices()) {
    attrs[v.key].d = D[v.key];
  }

  return attrs;
}

function minimumMeanWeightCycle(G: Graph<Vertex, WeightedEdge<Vertex>>): number {
  let s!: Vertex;
  for (let v of G.vertices()) {
    let dfs = new DFS(G);
    let [v_attr] = dfs.runFrom(v);
    if (v_attr.every(a => a.color === Color.BLACK)) {
      s = v;
      break;
    }
  }

  console.assert(s !== undefined, "No proper source");

  let A: number[][] = [];
  for (let k = 0, size = G.size(); k <= size; k++) {
    A[k] = new Array(size);
    A[k].fill(+Infinity);
  }
  A[0][s.key] = 0;

  for (let k = 1, size = G.size(); k <= size; k++) {
    for (let u of G.vertices()) {
      for (let { weight: w, to: v } of G.edgeFrom(u)) {
        A[k][u.key] = Math.min(A[k][u.key], A[k - 1][v.key] + w);
      }
    }
  }

  let n = G.size();
  //  the first row is reused as the temporary storage for column maximum
  for (let v of G.vertices()) {
    A[0][v.key] = -Infinity;
  }
  for (let { key } of G.vertices()) {
    let dn = A[n][key];
    for (let k = 1; k < n; k++) {
      A[k][key] = (dn - A[k][key]) / (n - k);
      A[0][key] = Math.max(A[0][key], A[k][key]);
    }
  }

  return Math.min(...A[0]);
}
