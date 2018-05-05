export {
  spBellmanFord,
  spDijkstra,
  spReport,
  spDag,
  dijkstraCheck,
};

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
function relax<V extends Vertex, E extends WeightedEdge<V>>(e: E, attrs: Array<SPAttrs<V, E>>): boolean {
  let { from: u, to: v, weight: w } = e;
  let ua = attrs[u.key];
  let va = attrs[v.key];
  if (va.d > ua.d + w) {
    va.d = ua.d + w;
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
