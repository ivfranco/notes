export {
  alterTopologicalSort,
  bfs,
  dfs,
  dfsReport,
  Graph,
  numberOfPaths,
  PlainGraph,
  scc,
  singlyConnected,
  topologicalSort,
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

  public vertexMap(): { [index: string]: V } {
    let map: { [index: string]: V } = Object.create(null);
    for (let v of this.vertices()) {
      map[v.name] = v;
    }
    return map;
  }

  public *edges(): IterableIterator<E> {
    for (let adj of this.Adj) {
      yield* adj;
    }
  }

  public *edgeFrom(u: V): IterableIterator<E> {
    yield* this.Adj[u.key];
  }

  public outDegree(u: V): number {
    return this.Adj[u.key].length;
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
  public static fromDirected(vertices: string, edges: string[]): PlainGraph {
    let G = new PlainGraph();
    let V: { [index: string]: Vertex } = Object.create(null);
    vertices
      .split(" ")
      .map(name => G.createVertex(name))
      .forEach(v => V[v.name] = v);

    edges.forEach(pair => {
      let [u, v] = pair.split(" ");
      G.createEdge(V[u], V[v]);
    });

    return G;
  }

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
  WHITE = "WHITE",
  GRAY = "GRAY",
  BLACK = "BLACK",
}

function bfs<V extends Vertex>(G: Graph<V, Edge<V>>, s: V): [number[], Array<V | null>] {
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

enum EdgeType {
  TREE = "tree",
  BACK = "back",
  FORWARD = "forward",
  CROSS = "cross",
}

interface DFSVertexAttr<V> {
  color: Color;
  //  visit time
  d: number;
  //  finish time
  f: number;
  //  index of or df tree or connected component
  cc: number;
  //  parent in df tree
  p: V | null;
}

type DFSEdgeAttr = EdgeType;

type OnFinish<V> = (u: V, ua: DFSVertexAttr<V>) => void;

function dfs<V extends Vertex>(
  G: Graph<V, Edge<V>>,
  s?: V | null,
  onFinish?: OnFinish<V>,
): [Array<DFSVertexAttr<V>>, DFSEdgeAttr[]] {
  // function visit(u: V) {
  //   time++;
  //   v_attr[u.key].d = time;
  //   v_attr[u.key].color = Color.GRAY;
  //   parens += "(" + u.name;
  //   for (let e of G.edgeFrom(u)) {
  //     let v = e.to;
  //     let v_color = v_attr[v.key].color;
  //     if (v_color === Color.WHITE) {
  //       v_attr[v.key].p = u;
  //       e_attr[e.key] = EdgeType.TREE;
  //       visit(v);
  //     } else if (v_color === Color.GRAY) {
  //       e_attr[e.key] = EdgeType.BACK;
  //     } else {
  //       if (v_attr[u.key].d < v_attr[v.key].d) {
  //         e_attr[e.key] = EdgeType.FORWARD;
  //       } else {
  //         e_attr[e.key] = EdgeType.CROSS;
  //       }
  //     }
  //   }
  //   v_attr[u.key].color = Color.BLACK;
  //   time++;
  //   v_attr[u.key].f = time;
  //   parens += u.name + ")";
  // }

  function stackVisit(u: V, cc: number) {
    let stack: Array<[V, Edge<V> | null]> = [[u, null]];
    //  records the edge from last gray vertex on the stack to the stack top
    let last_edge: Edge<V> | null;
    while (stack.length !== 0) {
      [u, last_edge] = stack.pop() as [V, Edge<V> | null];
      let ua = v_attr[u.key];
      if (ua.color === Color.WHITE) {
        stack.push([u, last_edge]);
        time++;
        ua.d = time;
        ua.cc = cc;
        ua.color = Color.GRAY;
        parens += "(" + u.name;
        if (last_edge) {
          //  u is chosen as the next gray vertex
          //  the edge between u and the last gray vertex thus is a tree edge
          e_attr[last_edge.key] = EdgeType.TREE;
        }
        let adj = Array.from(G.edgeFrom(u));
        //  so the vertices are visited in the same order as recursive visit, easier to debug
        adj.reverse();
        for (let e of adj) {
          let v = e.to;
          let va = v_attr[v.key];
          if (va.color === Color.WHITE) {
            va.p = u;
            last_edge = e;
            //  all edges starts as forward, may be updated later
            e_attr[e.key] = EdgeType.FORWARD;
            stack.push([v, e]);
          } else if (va.color === Color.GRAY) {
            //  v is colored gray before u, must be a back edge
            e_attr[e.key] = EdgeType.BACK;
          } else if (va.d > ua.d) {
            //  v is black and 22.3-5
            e_attr[e.key] = EdgeType.FORWARD;
          } else {
            e_attr[e.key] = EdgeType.CROSS;
          }
        }
      } else if (ua.color === Color.GRAY) {
        time++;
        ua.f = time;
        ua.color = Color.BLACK;
        if (onFinish) {
          onFinish(u, ua);
        }
        parens += u.name + ")";
      }
    }
  }

  let v_attr: Array<DFSVertexAttr<V>> = [];
  let e_attr: DFSEdgeAttr[] = [];
  let time = 0;

  let parens = "";

  for (let u of G.vertices()) {
    v_attr[u.key] = {
      color: Color.WHITE,
      d: +Infinity,
      f: +Infinity,
      cc: +Infinity,
      p: null,
    };
  }

  if (s) {
    stackVisit(s, 0);
  } else {
    let cc = 0;
    for (let u of G.vertices()) {
      if (v_attr[u.key].color === Color.WHITE) {
        stackVisit(u, cc);
        cc++;
      }
    }
  }

  // console.log(parens);
  return [v_attr, e_attr];
}

function showEdge(e: Edge<Vertex>): string {
  return `(${e.from.name}, ${e.to.name})`;
}

function dfsReport(G: Graph<Vertex, Edge<Vertex>>, v_attr: Array<DFSVertexAttr<Vertex>>, e_attr: DFSEdgeAttr[]) {
  for (let v of G.vertices()) {
    let { color, d, f, p, cc } = v_attr[v.key];
    let name = v.name;
    let parent = p ? p.name : "NIL";
    console.log(`vertex ${name}: ${d}/${f}, π = ${parent}, cc = ${cc}`);
  }
  for (let e of G.edges()) {
    console.log(`${showEdge(e)} is a ${e_attr[e.key]} edge`);
  }
}

function singlyConnected(G: Graph<Vertex, Edge<Vertex>>): boolean {
  for (let s of G.vertices()) {
    let [v_attr, e_attr] = dfs(G, s);
    dfsReport(G, v_attr, e_attr);
    if (!e_attr.every(t => t !== EdgeType.FORWARD && t !== EdgeType.CROSS)) {
      return false;
    }
  }
  return true;
}

function topologicalSort<V extends Vertex>(G: Graph<V, Edge<V>>): V[] {
  let sorted: V[] = [];
  let [v_attr, e_attr] = dfs(G, null, (u, ua) => sorted.push(u));
  // console.assert(e_attr.every(t => t !== EdgeType.BACK), "graph is not acyclic");
  // for (let v of G.vertices()) {
  //   console.log(`${v.name}.f = ${v_attr[v.key].f}`);
  // }
  return sorted.reverse();
}

function numberOfPaths(G: Graph<Vertex, Edge<Vertex>>, s: Vertex, t: Vertex): number {
  let sorted = topologicalSort(G);
  let p: number[] = [];
  let s_idx = sorted.indexOf(s);
  let t_idx = sorted.indexOf(t);
  for (let i = sorted.length - 1; i >= 0; i--) {
    let u = sorted[i];
    if (i < s_idx || i > t_idx) {
      p[u.key] = 0;
    } else if (i === t_idx) {
      p[u.key] = 1;
    } else {
      let n = 0;
      for (let { to: v } of G.edgeFrom(u)) {
        n += p[v.key];
      }
      p[u.key] = n;
    }
  }

  return p[s.key];
}

function alterTopologicalSort<V extends Vertex>(G: Graph<V, Edge<V>>): V[] {
  let d: number[] = [];
  for (let u of G.vertices()) {
    d[u.key] = 0;
  }
  for (let { to: v } of G.edges()) {
    d[v.key]++;
  }
  let stack = Array.from(G.vertices()).filter(u => d[u.key] === 0);

  console.assert(stack.length !== 0, "graph is not acyclic");

  let sorted: V[] = [];
  for (let i = G.size(); i > 0; i--) {
    let u = stack.pop() as V;
    sorted.push(u);
    for (let { to: v } of G.edgeFrom(u)) {
      d[v.key]--;
      if (d[v.key] === 0) {
        stack.push(v);
      }
    }
  }

  return sorted;
}

function scc<V extends Vertex>(G: Graph<V, Edge<V>>): Array<DFSVertexAttr<V>> {
  let sorted = topologicalSort(G);
  let T = new PlainGraph();
  for (let v of sorted) {
    T.createVertex(v.name);
  }
  let V = T.vertexMap();
  for (let { from, to } of G.edges()) {
    T.createEdge(V[to.name], V[from.name]);
  }
  let [v_attr, e_attr] = dfs(T);

  //  transform v_attr so it's indexed by u.key from vertices u of G
  let map = G.vertexMap();
  let g_attr: Array<DFSVertexAttr<V>> = [];
  for (let v of G.vertices()) {
    let u = map[v.name];
    let { color, d, f, cc, p } = v_attr[v.key];
    g_attr[u.key] = {
      color,
      d,
      f,
      cc,
      p: p ? map[p.name] : null,
    };
  }
  return g_attr;
}

function componentGraph(G: Graph<Vertex, Edge<Vertex>>): PlainGraph {
  let v_attr = scc(G);
  let SCC = new PlainGraph();

  let components = Math.max(...v_attr.map(a => a.cc)) + 1;
  for (let i = 0; i < components; i++) {
    SCC.createVertex("" + i);
  }
  let C = SCC.vertexMap();

  let sets: Vertex[][] = [];
  for (let u of G.vertices()) {
    let cc = v_attr[u.key].cc;
    if (sets[cc]) {
      sets[cc].push(u);
    } else {
      sets[cc] = [u];
    }
  }

  let connected: boolean[] = new Array(components);
  connected.fill(false);
  for (let set of sets) {
    for (let u of set) {
      let u_cc = v_attr[u.key].cc;
      for (let v of G.edgeFrom(u)) {
        let v_cc = v_attr[v.key].cc;
        if (!connected[v_cc] && u_cc !== v_cc) {
          SCC.createEdge(C[u_cc], C[v_cc]);
          connected[v_cc] = true;
        }
      }
    }
    for (let u of set) {
      for (let v of G.edgeFrom(u)) {
        let v_cc = v_attr[v.key].cc;
        connected[v_cc] = false;
      }
    }
  }

  return SCC;
}
