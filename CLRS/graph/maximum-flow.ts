export {
  edmondsKarp,
  flowReport,
  flowCheck,
  maximumMatching,
  pushRelabel,
  relabelToFront,
  relabelFIFO,
};

import { DList, DNode } from "../collection/dlist";
import { Queue } from "../collection/queue";
import { bfs, BFSAttrs, Color, Edge, Graph, showEdge, Vertex } from "./directed-graph";
import { WeightedEdge, WeightedGraph } from "./weighted-graph";

type ResidualGraph = WeightedGraph;
type EdgeMap<E> = Array<[E, boolean]>;
type Flow = number[];

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
  G: Graph<V, E>, f: Flow,
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

function augmentPath<V extends Vertex, E extends WeightedEdge<V>>(f: Flow, path: E[], e_map: EdgeMap<E>) {
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

function edmondsKarp<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>, s: V, t: V): Flow {
  let f: number[] = [];
  for (let e of G.edges()) {
    f[e.key] = 0;
  }

  let [Gf, v_map, e_map] = residualGraph(G, f);
  let sf = v_map[s.key];
  let tf = v_map[t.key];
  let attrs = bfs(Gf, sf);
  let iter = 0;
  while (attrs[tf.key].color === Color.BLACK) {
    console.log(`Before iteration ${iter}`);
    console.log(Gf.show());
    iter++;
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

function flowCheck(G: Graph<Vertex, WeightedEdge<Vertex>>, s: Vertex, t: Vertex, f: Flow) {
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

function flowReport(G: Graph<Vertex, WeightedEdge<Vertex>>, s: Vertex, f: Flow) {
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

function maximumMatching<V extends Vertex, E extends Edge<V>>(G: Graph<V, E>, L: V[], R: V[]): E[] {
  let H = new WeightedGraph();
  H.mapFrom(G, v => v, e => Object.assign({}, e, { weight: 1 }));
  let s = H.createVertex("s");
  let t = H.createVertex("t");
  for (let v of L) {
    H.createEdge(s, v, 1);
  }
  for (let v of R) {
    H.createEdge(v, t, 1);
  }

  let f = edmondsKarp(H, s, t);
  return Array.from(G.edges()).filter(e => f[e.key] > 0);
}

interface PRVertex extends Vertex {
  h: number;
  e: number;
}

interface PREdge extends WeightedEdge<PRVertex> {
  f: number;
  cf: number;
  forward: boolean;
  reverse: this;
}

class PRGraph extends Graph<PRVertex, PREdge> {
  //  a dummy edge, should immediately be replaced by a real edge upon edge construction
  private readonly nullEdge!: PREdge;
  private _s!: PRVertex;
  private _t!: PRVertex;

  protected vertexFactory(name: string, k: number): PRVertex {
    return {
      name,
      key: k,
      h: 0,
      e: 0,
    };
  }

  protected edgeFactory(u: PRVertex, v: PRVertex, k: number, w?: number): PREdge {
    return {
      key: k,
      from: u,
      to: v,
      weight: w ? w : 0,
      f: 0,
      cf: 0,
      forward: true,
      reverse: this.nullEdge,
    };
  }

  public s(): PRVertex {
    return this._s;
  }

  public t(): PRVertex {
    return this._t;
  }

  public createEdge(u: PRVertex, v: PRVertex, w?: number): PREdge {
    let e = this.edgeFactory(u, v, this.e_counter, w);
    this.e_counter++;
    this.Adj[u.key].push(e);
    return e;
  }

  public fromWeighted<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>, s: V, t: V): E[] {
    let V: PRVertex[] = [];
    let e_map: E[] = [];
    for (let v of G.vertices()) {
      V[v.key] = this.createVertex(v.name);
    }
    this._s = V[s.key];
    this._t = V[t.key];
    for (let e of G.edges()) {
      let { from: u, to: v, weight: c } = e;
      //  the forward edge that's also in G.E
      let f = this.createEdge(V[u.key], V[v.key], c);
      //  the backward edge that only exists in Gf
      let b = this.createEdge(V[v.key], V[u.key], 0);
      f.reverse = b;
      b.reverse = f;
      b.forward = false;
      updateFlow(f, 0);
      e_map[f.key] = e;
      e_map[b.key] = e;
    }

    return e_map;
  }

  public report(): void {
    for (let v of this.vertices()) {
      console.log(`${v.name}: e = ${v.e}, h = ${v.h}`);
    }
    for (let e of this.edges()) {
      console.log(`${showEdge(e)}: f = ${e.f}, c = ${e.weight}, cf = ${e.cf}, ${e.forward ? "forward" : "backward"}`);
    }
  }

  public diagnose() {
    let balance: number[] = new Array(this.size());
    balance.fill(0);
    for (let v of this.vertices()) {
      balance[v.key] -= v.e;
    }
    for (let e of this.edges()) {
      let { from: u, to: v } = e;
      if (e.forward) {
        balance[u.key] -= e.f;
        balance[v.key] += e.f;
      }
      //  capacity constraints
      console.assert(e.f <= e.weight, `capacity constraints on edge ${showEdge(e)} not satisfied`);
      //  validity of residual graph
      if (e.forward) {
        console.assert(e.cf === e.weight - e.f, `residual capacity on forward edge ${showEdge(e)} is incorrect`);
      } else {
        console.assert(e.cf === e.reverse.f, `residual capacity on backward edge ${showEdge(e)} is incorrect`);
        console.assert(e.f === 0 && e.weight === 0, `backward edge ${showEdge(e)} has non-zero flow or capacity`);
      }
      //  validity of height function
      if (e.cf > 0) {
        console.assert(u.h <= v.h + 1, `${showEdge(e)} ∈ Ef but h(${u.name}) > h(${v.name}) + 1`);
      }
    }

    //  preflow conservation
    for (let v of this.vertices()) {
      console.assert(balance[v.key] === 0, `Preflow conservation on ${v.name} not satisfied`);
    }
  }
}

//  updates everything may be changed by a new flow
//  namely u.e, v.e, (u, v).f, (u, v).cf, (v, u).cf
function updateFlow(e: PREdge, f: number) {
  console.assert(e.forward === true, "only the flow of an edge in E can be updated");
  console.assert(f >= 0 && f <= e.weight, "flow of an edge must be in the range 0 <= f(u, v) <= c(u, v)");

  let d = f - e.f;
  let { from: u, to: v } = e;
  u.e -= d;
  v.e += d;

  e.f = f;
  e.cf = e.weight - e.f;
  e.reverse.cf = f;
}

function initializePreflow<V extends Vertex, E extends WeightedEdge<V>>(
  G: Graph<V, E>, s: V, t: V,
): [PRGraph, E[]] {
  let H = new PRGraph();
  let e_map = H.fromWeighted(G, s, t);

  let hs = H.s();
  hs.h = H.size();

  for (let e of H.edgeFrom(hs)) {
    let { to: v, weight: c } = e;
    updateFlow(e, c);
  }

  return [H, e_map];
}

function push(e: PREdge) {
  let { from: u, to: v } = e;
  console.assert(u.e > 0, `${u.name} on push must be overflowing`);
  console.assert(e.cf > 0, `capacity of ${showEdge(e)} in residual graph must be positive`);
  console.assert(u.h === v.h + 1, `${u.name} must have height one higher than ${v.name}`);

  let d = Math.min(u.e, e.cf);
  if (e.forward) {
    updateFlow(e, e.f + d);
  } else {
    updateFlow(e.reverse, e.reverse.f - d);
  }
}

function relabel(G: PRGraph, u: PRVertex) {
  //  only edges in Ef (i.e. with positive cf) are considered
  let adj = Array.from(G.edgeFrom(u)).filter(e => e.cf > 0);
  let min_h = Math.min(...adj.map(e => e.to.h));
  console.assert(u.h <= min_h, `${u.name} must have height no higher than any adjacent vertices`);
  u.h = min_h + 1;
}

//  returns
//    1. a list of new pushable edges (from u or to u)
//    2. a list of edges no longer pushable to u
function relabelAndScan(G: PRGraph, u: PRVertex): [PREdge[], PREdge[]] {
  console.assert(u.e > 0, `${u.name} must be overflowing for relabel to apply`);
  //  u.h is increased by at least 1, these edges originally pushable to u are no longer pushable
  //  as u.h <= min{v.h | (u, v) ∈ Ef}, no edge from u is pushable before relabel
  let adj = Array.from(G.edgeFrom(u));
  let unpushable = adj
    .map(e => e.reverse)
    .filter(e => e.cf > 0 && e.from.h === u.h + 1);

  relabel(G, u);

  let pushable = adj.filter(e => e.cf > 0 && e.to.h === u.h - 1);
  for (let e of G.edgeFrom(u)) {
    let b = e.reverse;
    if (b.cf > 0 && b.from.h === u.h + 1) {
      pushable.push(b);
    }
  }

  return [pushable, unpushable];
}

type Pushable = Array<DList<PREdge>>;

function pushRelabel<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>, s: V, t: V): Flow {
  let [H, e_map] = initializePreflow(G, s, t);
  let V = H.vertexMap();
  let hs = V[s.name];
  let ht = V[t.name];
  let pushable: Pushable = [];
  //  initially s = |V| >= 2, other vertices have u.h == 0, no edge is pushable
  for (let i = 0, size = H.size(); i < size; i++) {
    pushable[i] = new DList();
  }
  let NE: Array<DNode<PREdge>> = [];
  for (let e of H.edges()) {
    NE[e.key] = new DNode(e);
  }

  let exceeding: DList<PRVertex> = new DList();
  let NV: Array<DNode<PRVertex>> = [];
  for (let v of H.vertices()) {
    let node = new DNode(v);
    NV[v.key] = node;
    if (v.e > 0 && v !== hs && v !== ht) {
      exceeding.append(node);
    }
  }

  while (!exceeding.isEmpty()) {
    // console.log("");
    // H.report();
    // H.diagnose();
    let u_node = exceeding.head as DNode<PRVertex>;
    let u = u_node.key;
    let u_pushable = pushable[u.key];
    if (u_pushable.isEmpty()) {
      let [edge_added, edge_deleted] = relabelAndScan(H, u);
      for (let e of edge_added) {
        let v = e.from;
        pushable[v.key].append(NE[e.key]);
      }
      for (let e of edge_deleted) {
        let v = e.from;
        pushable[v.key].delete(NE[e.key]);
      }
    } else {
      let e_node = u_pushable.head as DNode<PREdge>;
      let e = e_node.key;
      let v = e.to;
      let v_may_overflow = v.e === 0 && v !== hs && v !== ht;
      push(e);
      if (u.e === 0) {
        exceeding.delete(u_node);
      }
      if (e.cf === 0) {
        u_pushable.delete(e_node);
      }
      if (v_may_overflow && v.e > 0) {
        exceeding.append(NV[v.key]);
      }
    }
  }

  return recoordFlow(H, e_map);
}

function recoordFlow(G: PRGraph, e_map: Array<Edge<Vertex>>): Flow {
  let f: Flow = [];
  for (let e of G.edges()) {
    if (e.forward) {
      let k = e_map[e.key].key;
      f[k] = e.f;
    }
  }

  return f;
}

function discharge(G: PRGraph, u: PRVertex, N: PREdge[], current: number): [number, PRVertex[]] {
  let exceeding: PRVertex[] = [];

  while (u.e > 0) {
    if (current >= N.length) {
      relabel(G, u);
      current = 0;
    } else {
      let edge = N[current];
      let v = edge.to;
      if (edge.cf > 0 && u.h === v.h + 1) {
        if (v.e === 0) {
          exceeding.push(v);
        }
        push(edge);
      } else {
        current++;
      }
    }
  }

  return [current, exceeding];
}

function initializeNeighbourhood(G: PRGraph): [number[], PREdge[][]] {
  let currents = new Array(G.size());
  currents.fill(0);
  let neighbours: PREdge[][] = [];
  for (let v of G.vertices()) {
    neighbours[v.key] = [];
    for (let e of G.edgeFrom(v)) {
      neighbours[v.key].push(e);
    }
  }

  return [currents, neighbours];
}

function relabelToFront<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>, s: V, t: V): Flow {
  let [H, e_map] = initializePreflow(G, s, t);
  let [currents, neighbours] = initializeNeighbourhood(H);
  let L: DList<PRVertex> = new DList();
  for (let v of H.vertices()) {
    if (v.name !== s.name && v.name !== t.name) {
      L.insert(v);
    }
  }

  let node = L.head;
  while (node !== null) {
    let u = node.key;
    let old_height = u.h;
    currents[u.key] = discharge(H, u, neighbours[u.key], currents[u.key])[0];
    if (u.h > old_height) {
      L.delete(node);
      L.prepend(node);
    }
    node = node.next;
  }

  return recoordFlow(H, e_map);
}

function overflowing(G: PRGraph, u: PRVertex): boolean {
  return u.e > 0 && u !== G.s() && u !== G.t();
}

function relabelFIFO<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>, s: V, t: V): Flow {
  let [H, e_map] = initializePreflow(G, s, t);
  let [currents, neighbours] = initializeNeighbourhood(H);
  let Q: Queue<PRVertex> = new Queue(G.size());
  for (let v of H.vertices()) {
    if (overflowing(H, v)) {
      Q.enqueue(v);
    }
  }

  while (!Q.isEmpty()) {
    let u = Q.dequeue();
    let [current, exceeding] = discharge(H, u, neighbours[u.key], currents[u.key]);
    H.diagnose();
    currents[u.key] = current;
    for (let v of exceeding) {
      if (overflowing(H, v)) {
        Q.enqueue(v);
      }
    }
  }

  return recoordFlow(H, e_map);
}
