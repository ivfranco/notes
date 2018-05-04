export {
  WeightedEdge,
  WeightedGraph,
  showWeighted,
  mstKruskal,
  mstPrim,
};

import { DSTreeNode } from "../structure/disjoint-set-forest";
import { AbstractFHeap, FHeapNode } from "../structure/fibonacci-heap";
import { Edge, Graph, Vertex } from "./directed-graph";

interface WeightedEdge<V> extends Edge<V> {
  readonly weight: number;
}

function showWeighted(e: WeightedEdge<Vertex>): string {
  let { from, to, weight } = e;
  return `${from.name} -> ${to.name}: ${weight}`;
}

class WeightedGraph extends Graph<Vertex, WeightedEdge<Vertex>> {
  public static fromDirected(vertices: string, edges: string[]): WeightedGraph {
    return WeightedGraph.from(vertices, edges, true);
  }

  public static fromUndirected(vertices: string, edges: string[]): WeightedGraph {
    return WeightedGraph.from(vertices, edges, false);
  }

  private static from(vertices: string, edges: string[], directed: boolean): WeightedGraph {
    let G = new WeightedGraph();
    let V: { [index: string]: Vertex } = Object.create(null);
    vertices
      .split(" ")
      .map(name => G.createVertex(name))
      .forEach(v => V[v.name] = v);

    edges.forEach(pair => {
      let [u, v, w] = pair.split(" ");
      if (directed) {
        G.createEdge(V[u], V[v], parseFloat(w));
      } else {
        G.createUndirectedEdge(V[u], V[v], parseFloat(w));
      }
    });

    return G;
  }

  protected vertexFactory(name: string, k: number): Vertex {
    return {
      name,
      key: k,
    };
  }

  protected edgeFactory(u: Vertex, v: Vertex, k: number, w?: number): WeightedEdge<Vertex> {
    return {
      key: k,
      from: u,
      to: v,
      weight: w ? w : 0,
    };
  }

  public createEdge(u: Vertex, v: Vertex, w?: number): WeightedEdge<Vertex> {
    let e = this.edgeFactory(u, v, this.e_counter, w);
    this.e_counter++;
    this.Adj[u.key].push(e);
    return e;
  }

  //  an undirected edge is a pair of WeightedEdge objects with the same key
  public createUndirectedEdge(u: Vertex, v: Vertex, w?: number): [WeightedEdge<Vertex>, WeightedEdge<Vertex>] {
    let e = this.edgeFactory(u, v, this.e_counter, w);
    let f = this.edgeFactory(v, u, this.e_counter, w);
    this.e_counter++;
    this.Adj[u.key].push(e);
    this.Adj[v.key].push(f);
    return [e, f];
  }
}

//  return type of MST functions
//  an array of parents indexed by key of vertices, and a set of edges in the MST
type MST<V, E> = [Array<V | null>, E[]];

function mstKruskal<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>): MST<V, E> {
  let A = [];
  let E = Array.from(G.edges());
  E.sort((e, f) => e.weight - f.weight);
  let V: Array<DSTreeNode<Vertex>> = [];
  for (let v of G.vertices()) {
    V[v.key] = new DSTreeNode(v);
  }

  for (let e of E) {
    let { from, to } = e;
    let u = V[from.key];
    let v = V[to.key];
    if (u.findSet() !== v.findSet()) {
      A.push(e);
      u.union(v);
    }
  }

  let p: V[] = [];
  for (let { from: u, to: v } of A) {
    if (u.key < v.key) {
      p[v.key] = u;
    } else {
      p[u.key] = v;
    }
  }

  return [p, A];
}

class MSTNode<V extends Vertex> extends FHeapNode<number, V> {
  protected nodeStringify(): string {
    let name = this.value.name;
    let key = this.key === +Infinity ? "∞" : this.key;
    return `${name}:${key}`;
  }
}

class MSTHeap<V extends Vertex> extends AbstractFHeap<number, V, MSTNode<V>> {
  protected cmp(a: number, b: number) {
    return a < b;
  }

  protected factory(k: number, v: V): MSTNode<V> {
    return new MSTNode(k, v);
  }
}

function mstPrim<V extends Vertex, E extends WeightedEdge<V>>(G: Graph<V, E>, r: V): MST<V, E> {
  let A: E[] = [];
  let Q = new MSTHeap<V>();
  //  an array of heap nodes, indexed by vertex key
  let N: Array<MSTNode<V> | null> = [];
  for (let u of G.vertices()) {
    let node: MSTNode<V>;
    if (u === r) {
      node = Q.insert(0, u);
    } else {
      node = Q.insert(+Infinity, u);
    }
    N[u.key] = node;
  }

  let p: V[] = [];
  while (!Q.isEmpty()) {
    let node = Q.extractMin() as MSTNode<V>;
    let u = node.value;
    N[u.key] = null;
    for (let e of G.edgeFrom(u)) {
      let v = e.to;
      let v_node = N[v.key];
      if (v_node && e.weight < v_node.key) {
        p[v.key] = u;
        Q.decreaseKey(v_node, e.weight);
      }
    }
  }

  for (let e of G.edges()) {
    let { from, to } = e;
    if (p[to.key] === from) {
      A.push(e);
    }
  }

  return [p, A];
}
