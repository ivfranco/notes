import graphviz from "graphviz";

export { Arrow, Entity, Relation, ERModel, RelationKind, binary_relation };

interface Entity {
  label: string;
  attrs?: Array<string>;
}

enum Arrow {
  Many,
  One,
}

function arrow_style(arrow: Arrow): graphviz.Options {
  switch (arrow) {
    case Arrow.Many:
      return { arrowhead: "none" };
    case Arrow.One:
      return { arrowhead: "normal" };
  }
}

interface Relation extends Entity {
  arrows: Array<[Entity, Arrow]>;
}

enum RelationKind {
  ManyMany,
  ManyOne,
  OneMany,
  OneOne,
}

function binary_relation(label: string, from: Entity, to: Entity, kind?: RelationKind): Relation {
  kind = kind ?? RelationKind.ManyMany;
  let from_arrow = kind === RelationKind.ManyMany || kind == RelationKind.ManyOne ? Arrow.Many : Arrow.One;
  let to_arrow = kind === RelationKind.ManyMany || kind == RelationKind.OneMany ? Arrow.Many : Arrow.One;
  return {
    label: label,
    arrows: [
      [from, from_arrow],
      [to, to_arrow],
    ],
  };
}

class ERModel {
  inner: graphviz.Graph;

  constructor(label: string) {
    this.inner = graphviz.digraph(label);
  }

  private add_cluster(entity: Entity, shape: string): graphviz.Node {
    let g = this.inner;

    // name of a subgraph cannot contain `-` or `.`
    let cluster = g.addCluster(`Cluster_${entity.label.replace("-", "_")}`);
    cluster.set("style", "filled");
    cluster.set("color", "none");

    let entry = cluster.addNode(entity.label, { shape: shape });

    for (let attr_name of entity.attrs ?? []) {
      let attr = cluster.addNode(`${entity.label}.${attr_name}`, { label: attr_name });
      cluster.addEdge(entry, attr, { arrowhead: "none" });
    }

    return entry;
  }

  add_entity(entity: Entity): graphviz.Node {
    return this.add_cluster(entity, "box");
  }

  add_relation(relation: Relation): graphviz.Node {
    let g = this.inner;
    let entry = this.add_cluster(relation, "diamond");

    for (let [target, arrow] of relation.arrows) {
      g.addEdge(entry, target.label, arrow_style(arrow));
    }

    return entry;
  }

  output(path: string) {
    this.inner.output("png", path, console.error);
  }
}
