export {
  edmondsKarp,
  flowReport,
  flowCheck,
};

import { bfs, BFSAttrs, Color, Graph, showEdge, Vertex } from "./directed-graph";
import { WeightedEdge, WeightedGraph } from "./weighted-graph";

type ResidualGraph = WeightedGraph;
type EdgeMap<E> = Array<[E, boolean]>;

//  takes:
//    1.  a graph
//    2.  a flow defined on the graph (indexed by keys of edges)
//  returns:
//    1.  the residual graph
//    2.  map from (keys of) vetices in original graph to vertices in residual graph
//    3.  map from (keys of) edges in residual graph to edges in the original graph
//  the second mapping is not injective, two edges in residual graph may be mapped to the same edge in original graph
//  thus an additional boolean indicates whether (u, v) in Gf corresponds to (u, v) or (v, u) in G
function residualGraph<V extends Vertex, E extends WeightedEdge<V>>(
  G: Graph<V, E>, f: number[],
): [ResidualGraph, Vertex[], EdgeMap<E>] {
  let edge_map: EdgeMap<E> = [];
  let Gf = new WeightedGraph();
  let V: Vertex[] = [];

  for (let v of G.vertices()) {
    V[v.key] = Gf.createVertex(v.name);
  }

  for (let e of G.edges()) {
    let { key: k, weight: c, from: u, to: v } = e;
    let uf = V[u.key];
    let vf = V[v.key];
    if (c - f[k] > 0) {
      //  if c(u, v) - f(u, v) > 0, (u, v) still can be increased
      //  (u, v) is in the residual graph with capacity c(u, v) - f(u, v)
      let ef = Gf.createEdge(uf, vf, c - f[k]);
      edge_map[ef.key] = [e, true];
    }
    if (f[k] > 0) {
      //  if f(u, v) > 0, f(u, v) can be decreased
      //  (v, u) is in the residual graph with capacity f(u, v)
      let ef = Gf.createEdge(vf, uf, f[k]);
      edge_map[ef.key] = [e, false];
    }
  }

  return [Gf, V, edge_map];
}

function traverseBack<V extends Vertex, E>(attrs: Array<BFSAttrs<V, E>>, t: V): E[] {
  let path: E[] = [];
  let ta = attrs[t.key];
  while (ta.p !== null) {
    path.push(ta.e as E);
    t = ta.p;
    ta = attrs[t.key];
  }

  return path;
}

function augmentPath<V extends Vertex, E extends WeightedEdge<V>>(f: number[], path: E[], e_map: EdgeMap<E>) {
  let cfp = Math.min(...path.map(e => e.weight));
  for (let ef of path) {
    let [e, same_direction] = e_map[ef.key];
    if (same_direction) {
      f[e.key] += cfp;
    } else {
      f[e.key] -= cfp;
    }
  }
}

function edmondsKarp<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>, s: V, t: V): number[] {
  let f: number[] = [];
  for (let e of G.edges()) {
    f[e.key] = 0;
  }

  let [Gf, v_map, e_map] = residualGraph(G, f);
  let sf = v_map[s.key];
  let tf = v_map[t.key];
  let attrs = bfs(Gf, sf);
  // let iter = 0;
  while (attrs[tf.key].color === Color.BLACK) {
    // console.log(`Before iteration ${iter}`);
    // iter++;
    // flowReport(G, s, f);
    let path = traverseBack(attrs, tf);
    augmentPath(f, path, e_map);
    [Gf, v_map, e_map] = residualGraph(G, f);
    sf = v_map[s.key];
    tf = v_map[t.key];
    attrs = bfs(Gf, sf);
  }

  return f;
}

function flowCheck(G: Graph<Vertex, WeightedEdge<Vertex>>, s: Vertex, t: Vertex, f: number[]) {
  for (let e of G.edges()) {
    let { key: k, weight: c } = e;
    console.assert(c >= f[k], `Capacity constraints not satisfied on edge ${showEdge(e)}`);
  }

  let accum: number[] = [];
  for (let v of G.vertices()) {
    accum[v.key] = 0;
  }
  for (let { key: k, from: u, to: v } of G.edges()) {
    accum[u.key] -= f[k];
    accum[v.key] += f[k];
  }

  for (let v of G.vertices()) {
    if (v !== s && v !== t) {
      console.assert(accum[v.key] === 0, `Flow conservation not satisfied on vertex ${v.name}`);
    }
  }
}

function flowReport(G: Graph<Vertex, WeightedEdge<Vertex>>, s: Vertex, f: number[]) {
  for (let { key: k, from: u, to: v, weight: c } of G.edges()) {
    console.log(`${u.name} -> ${v.name}: ${f[k]}/${c}`);
  }

  let flow = 0;
  for (let { key: k, from: u, to: v } of G.edges()) {
    if (u === s) {
      flow += f[k];
    }
    if (v === s) {
      flow -= f[k];
    }
  }
  console.log(`The maximum flow is ${flow}`);
}
