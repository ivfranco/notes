export {
  toSSLinearProgram,
  toFlowLinearProgram,
  toCompactFlowLinearProgram,
};

import { Graph, Vertex } from "../graph/directed-graph";
import { WeightedEdge } from "../graph/weighted-graph";

function toSSLinearProgram(G: Graph<Vertex, WeightedEdge<Vertex>>, s: Vertex, t: Vertex): string {
  let rep: string[] = [];
  rep.push(`maximize d${t.name}`);
  rep.push(`subject to`);
  let W: number[][] = [];
  for (let { from: u, to: v, weight: w } of G.edges()) {
    rep.push(`d${v.name} <= d${u.name} + ${w}`);
  }
  rep.push(`d${s.name} = 0`);
  return rep.join("\n");
}

function flow(u: Vertex, v: Vertex): string {
  return `f${u.name}${v.name}`;
}

function outFlow(u: Vertex, vs: Vertex[]): string {
  let sum = vs.map(v => flow(u, v)).join(" + ");
  return sum === "" ? "0" : sum;
}

function inFlow(v: Vertex, us: Vertex[]): string {
  let sum = us.map(u => flow(u, v)).join(" + ");
  return sum === "" ? "0" : sum;
}

function toFlowLinearProgram(G: Graph<Vertex, WeightedEdge<Vertex>>, s: Vertex, t: Vertex): string {
  let C: number[][] = [];
  for (let u of G.vertices()) {
    C[u.key] = [];
    for (let v of G.vertices()) {
      C[u.key][v.key] = 0;
    }
  }
  for (let { from: u, to: v, weight: w } of G.edges()) {
    C[u.key][v.key] = w;
  }
  let V = Array.from(G.vertices());

  let header: string[] = [];
  header.push(`maximize ${outFlow(s, V)} - (${inFlow(s, V)})`);
  header.push("subject to");
  let capacity_constraints: string[] = [];
  let conservation_constraints: string[] = [];
  let nonneg_constraints: string[] = [];
  for (let u of G.vertices()) {
    if (u !== s && u !== t) {
      conservation_constraints.push(`${inFlow(u, V)} = ${outFlow(u, V)}`);
    }
    for (let v of G.vertices()) {
      capacity_constraints.push(`${flow(u, v)} <= ${C[u.key][v.key]}`);
      nonneg_constraints.push(`${flow(u, v)} >= 0`);
    }
  }

  return [
    ...header,
    ...capacity_constraints,
    ...conservation_constraints,
    ...nonneg_constraints,
  ].join("\n");
}

function toCompactFlowLinearProgram(G: Graph<Vertex, WeightedEdge<Vertex>>, s: Vertex, t: Vertex): string {
  let in_vertices: Vertex[][] = [];
  let out_vertices: Vertex[][] = [];
  for (let v of G.vertices()) {
    in_vertices[v.key] = [];
    out_vertices[v.key] = [];
  }
  for (let e of G.edges()) {
    let { from: u, to: v } = e;
    in_vertices[v.key].push(u);
    out_vertices[u.key].push(v);
  }

  let header: string[] = [];
  header.push(`maximize ${outFlow(s, out_vertices[s.key])} - (${inFlow(s, in_vertices[s.key])})`);
  header.push("subject to");
  let capacity_constraints: string[] = [];
  let conservation_constraints: string[] = [];
  let nonneg_constraints: string[] = [];
  for (let u of G.vertices()) {
    if (u !== s && u !== t) {
      conservation_constraints.push(`${inFlow(u, in_vertices[u.key])} = ${outFlow(u, out_vertices[u.key])}`);
    }
  }
  for (let { from: u, to: v, weight: c } of G.edges()) {
    capacity_constraints.push(`${flow(u, v)} <= c`);
    nonneg_constraints.push(`${flow(u, v)} >= 0`);
  }

  return [
    ...header,
    ...capacity_constraints,
    ...conservation_constraints,
    ...nonneg_constraints,
  ].join("\n");
}
