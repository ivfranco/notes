import { simplex } from "../linear-programming/simplex";
import { PlainGraph, Vertex } from "../graph/directed-graph";

type Coff = number;

export function approxMinWeightVC(G: PlainGraph, w: number[]): Vertex[] {
  let [A, b, c] = toLinearProblem(G, w);
  let x = simplex(A, b, c) as Coff[];
  return Array.from(G.vertices()).filter(v => x[v.key] >= 1 / 2);
}

function toLinearProblem(G: PlainGraph, w: number[]): [Coff[][], Coff[], Coff[]] {
  let V = Array.from(G.vertices());

  let A: Coff[][] = [];
  for (let e of G.edges()) {
    let a: Coff[] = [];
    for (let i = 0; i < G.size(); i++) {
      a[i] = 0;
    }
    a[e.from.key] = -1;
    a[e.to.key] = -1;
    A.push(a);
  }

  let b: Coff[] = [];
  for (let i = 0; i < G.edgeSize(); i++) {
    b[i] = -1;
  }

  let c: Coff[] = w.map(c => -c);

  return [A, b, c];
}