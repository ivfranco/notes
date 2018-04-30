import { Edge, Graph, Vertex } from "./directed-graph";

interface WeightedEdge<V> extends Edge<V> {
  weight: number;
}

class WeightedGraph extends Graph<Vertex, WeightedEdge<Vertex>> {
  protected vertexFactory(name: string, k: number): Vertex {
    return {
      name,
      key: k,
    };
  }

  protected edgeFactory(u: Vertex, v: Vertex, k: number): WeightedEdge<Vertex> {
    return {
      key: k,
      from: u,
      to: v,
      weight: 0,
    };
  }

  public createEdge(u: Vertex, v: Vertex, w?: number): WeightedEdge<Vertex> {
    let e = this.edgeFactory(u, v, this.e_counter);
    if (w !== undefined) {
      e.weight = w;
    }
    this.e_counter++;
    this.Adj[u.key].push(e);
    return e;
  }
}
