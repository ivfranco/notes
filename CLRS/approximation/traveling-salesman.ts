import { WeightedGraph, mstPrim } from "../graph/weighted-graph"
import { Vertex } from "../graph/directed-graph"
import { NAryTree } from "./vertex-cover"

export function approxTspTour(G: WeightedGraph): Vertex[] {
  let iter = G.vertices();
  let r = iter.next().value;

  let [P, _] = mstPrim(G, r);
  let T = treeify(r, G, P);

  let C: Vertex[] = [];
  let seen: boolean[] = [];
  for (let v of T.preorder()) {
    if (seen[v.key] !== undefined) {
      C.push(v);
    }
    seen[v.key] = true;
  }

  return C;
}

function treeify(r: Vertex, G: WeightedGraph, P: Array<Vertex | null>): NAryTree<Vertex> {
  let trees: NAryTree<Vertex>[] = [];

  for (let v of G.vertices()) {
    trees[v.key] = new NAryTree(v, null, []);
  }

  for (let v of G.vertices()) {
    let p = P[v.key];
    if (p !== null) {
      trees[p.key].children.push(trees[v.key]);
    }
  }

  return trees[r.key];
}
