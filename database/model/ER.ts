import graphviz from 'graphviz';

export {
  Arrow, Entity, Relation, ERModel, RelationKind, binary_relation,
};

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
      return { arrowhead: 'none' };
    case Arrow.One:
      return { dir: "forward", arrowhead: 'normal' };
    default:
      throw Error('unreachable');
  }
}

interface Relation extends Entity {
  arrows: Array<[Entity, Arrow]>;
}

interface ISA {
  base: Entity;
  child: Entity;
}

enum RelationKind {
  ManyMany,
  ManyOne,
  OneMany,
  OneOne,
}

function binary_relation(label: string, from: Entity, to: Entity, kind?: RelationKind): Relation {
  kind = kind ?? RelationKind.ManyMany;
  const from_arrow = kind === RelationKind.ManyMany || kind == RelationKind.ManyOne ? Arrow.Many : Arrow.One;
  const to_arrow = kind === RelationKind.ManyMany || kind == RelationKind.OneMany ? Arrow.Many : Arrow.One;
  return {
    label,
    arrows: [
      [from, from_arrow],
      [to, to_arrow],
    ],
  };
}

class ERModel {
  inner: graphviz.Graph;

  constructor(label: string) {
    this.inner = graphviz.graph(label);
  }

  private add_cluster(entity: Entity, shape: string): graphviz.Node {
    const g = this.inner;

    // name of a subgraph cannot contain `-` or `.`
    const cluster = g.addCluster(`Cluster_${entity.label.replace('-', '_')}`);
    cluster.set('style', 'filled');
    cluster.set('color', 'none');

    const entry = cluster.addNode(entity.label, { shape: shape });

    for (const attr_name of entity.attrs ?? []) {
      const attr = cluster.addNode(`${entity.label}.${attr_name}`, { label: attr_name });
      cluster.addEdge(entry, attr);
    }

    return entry;
  }

  add_entity(entity: Entity): graphviz.Node {
    return this.add_cluster(entity, 'box');
  }

  add_relation(relation: Relation): graphviz.Node {
    const g = this.inner;

    let entry;
    if (relation.attrs) {
      entry = this.add_cluster(relation, 'diamond');
    } else {
      entry = g.addNode(relation.label, { shape: 'diamond' });
    }

    for (const [target, arrow] of relation.arrows) {
      g.addEdge(entry, target.label, arrow_style(arrow));
    }

    return entry;
  }

  add_isa(isa: ISA): void {
    const g = this.inner;
    const { base, child } = isa;

    const entry = g.addNode(`${base.label}-${child.label}-ISA`, { label: 'isa', shape: 'triangle' });
    g.addEdge(entry, base.label, { arrowhead: 'normal' });
    g.addEdge(entry, child.label, { arrowhead: 'none' });
  }

  output(path: string): void {
    this.inner.render;
    this.inner.output({ type: 'png', use: 'dot' }, path, console.error);
  }
}
