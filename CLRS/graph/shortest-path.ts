export {
  spBellmanFord,
  spDijkstra,
  spReport,
  spDag,
};

import { AbstractFHeap, FHeapNode } from "../structure/fibonacci-heap";
import { Edge, Graph, topologicalSort, Vertex } from "./directed-graph";
import { WeightedEdge } from "./weighted-graph";

interface SPAttrs<V> {
  d: number;
  p: V | null;
}

function initialize<V extends Vertex>(G: Graph<V, WeightedEdge<V>>, s: Vertex): Array<SPAttrs<V>> {
  let attrs: Array<SPAttrs<V>> = [];
  for (let v of G.vertices()) {
    attrs[v.key] = {
      d: +Infinity,
      p: null,
    };
  }
  attrs[s.key].d = 0;
  return attrs;
}

function relax<V extends Vertex>(e: WeightedEdge<V>, attrs: Array<SPAttrs<V>>): boolean {
  let { from: u, to: v, weight: w } = e;
  let ua = attrs[u.key];
  let va = attrs[v.key];
  if (va.d > ua.d + w) {
    va.d = ua.d + w;
    va.p = u;
    return true;
  } else {
    return false;
  }
}

function spBellmanFord<V extends Vertex>(G: Graph<V, WeightedEdge<V>>, s: V): Array<SPAttrs<V>> {
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

function spReport(G: Graph<Vertex, WeightedEdge<Vertex>>, attrs: Array<SPAttrs<Vertex>>) {
  for (let v of G.vertices()) {
    let va = attrs[v.key];
    let name = v.name;
    let parent = va.p ? va.p.name : "NIL";
    console.log(`Vertex ${name}: d = ${va.d}, π = ${parent}`);
  }
}

function spDag<V extends Vertex>(G: Graph<V, WeightedEdge<V>>, s: V): Array<SPAttrs<V>> {
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

function spDijkstra<V extends Vertex>(G: Graph<V, WeightedEdge<V>>, s: V): Array<SPAttrs<V>> {
  let attrs = initialize(G, s);
  let Q = new SPHeap<V>();
  // let S: V[] = [];
  let N: Array<SPNode<V>> = [];
  for (let v of G.vertices()) {
    let node = Q.insert(attrs[v.key].d, v);
    N[v.key] = node;
  }

  while (!Q.isEmpty()) {
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
