export {
  PlainGraph,
  bfs,
  Graph,
  Vertex,
};

import { Queue } from "../collection/queue";

interface Vertex {
  //  key is what used to access the adjacent list
  //  therefore vertices must have distinct keys
  readonly key: number;
  readonly name: string;
}

interface Edge<V> {
  //  key of an edge is not necessary but useful
  //  e.g. when simulating attributes of edges in an external array
  //  therefore edges must also have distinct keys
  readonly key: number;
  readonly from: V;
  readonly to: V;
}

interface WeightedEdge<V> extends Edge<V> {
  weight: number;
}

abstract class Graph<V extends Vertex, E extends Edge<V>> {
  protected V: V[];
  protected Adj: E[][];
  protected v_counter: number;
  protected e_counter: number;

  protected abstract vertexFactory(name: string, k: number): V;
  protected abstract edgeFactory(u: V, v: V, k: number): E;

  constructor() {
    this.V = [];
    this.Adj = [];
    this.v_counter = 0;
    this.e_counter = 0;
  }

  public size(): number {
    return this.v_counter;
  }

  public createVertex(name: string): V {
    let v = this.vertexFactory(name, this.v_counter);
    this.v_counter++;
    this.V.push(v);
    this.Adj[v.key] = [];
    return v;
  }

  public createEdge(u: V, v: V): E {
    let e = this.edgeFactory(u, v, this.e_counter);
    this.e_counter++;
    this.Adj[u.key].push(e);
    return e;
  }

  public *vertices(): IterableIterator<V> {
    yield* this.V;
  }

  public *edges(): IterableIterator<E> {
    for (let adj of this.Adj) {
      yield* adj;
    }
  }

  public *edgeFrom(u: V): IterableIterator<E> {
    yield* this.Adj[u.key];
  }

  public mapFrom<U extends Vertex, F extends Edge<U>, G extends Graph<U, F>>(G: G, f: (u: U) => V, g: (f: F) => E) {
    this.Adj = G.Adj.map(adj => {
      return adj.map(g);
    });
    this.V = G.V.map(f);
    this.v_counter = G.v_counter;
    this.e_counter = G.e_counter;
  }
}

class PlainGraph extends Graph<Vertex, Edge<Vertex>> {
  protected vertexFactory(name: string, k: number): Vertex {
    return {
      name,
      key: k,
    };
  }

  protected edgeFactory(u: Vertex, v: Vertex, k: number): Edge<Vertex> {
    return {
      key: k,
      from: u,
      to: v,
    };
  }
}

enum Color {
  WHITE,
  GRAY,
  BLACK,
}

function bfs<V extends Vertex, E extends Edge<V>>(G: Graph<V, E>, s: V): [number[], Array<V | null>] {
  let color: Color[] = [];
  let d: number[] = [];
  let p: Array<V | null> = [];

  for (let v of G.vertices()) {
    let k = v.key;
    color[k] = Color.WHITE;
    d[k] = +Infinity;
    p[k] = null;
  }

  color[s.key] = Color.GRAY;
  d[s.key] = 0;
  p[s.key] = null;

  let Q: Queue<V> = new Queue(G.size());
  Q.enqueue(s);
  while (!Q.isEmpty()) {
    let u = Q.dequeue();
    for (let e of G.edgeFrom(u)) {
      let v = e.to;
      if (color[v.key] === Color.WHITE) {
        color[v.key] = Color.GRAY;
        d[v.key] = d[u.key] + 1;
        p[v.key] = u;
        Q.enqueue(v);
      }
    }
    color[u.key] = Color.BLACK;
  }

  return [d, p];
}

function dfs<V extends Vertex, E extends Edge<V>>(G: Graph<V, E>): [number[], number[], Array<V | null>] {
  function visit(u: V) {
    time++;
    d[u.key] = time;
    color[u.key] = Color.GRAY;
    for (let { to: v } of G.edgeFrom(u)) {
      if (color[v.key] === Color.WHITE) {
        p[v.key] = u;
        visit(v);
      }
    }
    color[u.key] = Color.BLACK;
    time++;
    f[u.key] = time;
  }

  let color: Color[] = [];
  let d: number[] = [];
  let f: number[] = [];
  let p: Array<V | null> = [];
  let time = 0;

  for (let u of G.vertices()) {
    color[u.key] = Color.WHITE;
    p[u.key] = null;
  }

  for (let u of G.vertices()) {
    if (color[u.key] === Color.WHITE) {
      visit(u);
    }
  }

  return [d, f, p];
}
