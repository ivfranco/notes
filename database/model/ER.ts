import graphviz from 'graphviz';

export {
  Arrow, Entity, Relation, ERModel, RelationKind, binary_relation, isa
};

interface Entity {
  // entities in the same model must have distinct names
  label: string;
  // entities may have identical attributes
  attrs?: Array<string>;
}

enum Arrow {
  Many,
  One,
}

function arrow_style(arrow: Arrow, text?: string): graphviz.Options {
  const style = {};

  switch (arrow) {
    case Arrow.Many:
      Object.assign(style, { arrowhead: 'none' });
      break;
    case Arrow.One:
      Object.assign(style, { dir: 'forward', arrowhead: 'normal' });
      break;
  }

  if (text) {
    Object.assign(style, { label: text });
  }

  return style;
}

interface Relation extends Entity {
  arrows: Array<[Entity, Arrow, string?]>;
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

function binary_relation(
  label: string,
  from: Entity,
  to: Entity,
  kind: RelationKind = RelationKind.ManyMany
): Relation {
  const from_arrow = kind === RelationKind.ManyMany || kind === RelationKind.ManyOne ? Arrow.Many : Arrow.One;
  const to_arrow = kind === RelationKind.ManyMany || kind == RelationKind.OneMany ? Arrow.Many : Arrow.One;
  return {
    label,
    arrows: [
      [from, from_arrow],
      [to, to_arrow],
    ],
  };
}

function isa(base: Entity, child: Entity): ISA {
  return {
    base,
    child
  };
}

// http://www.graphviz.org/Gallery/undirected/ER.html
class ERModel {
  inner: graphviz.Graph;

  constructor(label: string) {
    const GRAPH_STYLE = {
      dpi: 240,
      layout: 'neato',
      overlap: 'scale',
    };

    this.inner = graphviz.graph(label);
    for (const [key, value] of Object.entries(GRAPH_STYLE)) {
      this.inner.set(key, value);
    }
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
      cluster.addEdge(entry, attr, { len: 0.5 });
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

    for (const [target, arrow, text] of relation.arrows) {
      g.addEdge(entry, target.label, arrow_style(arrow, text));
    }

    return entry;
  }

  add_isa(isa: ISA): void {
    const g = this.inner;
    const { base, child } = isa;

    const entry = g.addNode(`${base.label}-${child.label}-ISA`, { label: 'isa', shape: 'triangle', margin: 0 });
    g.addEdge(entry, base.label, { dir: 'forward', arrowhead: 'normal' });
    g.addEdge(entry, child.label, { arrowhead: 'none' });
  }

  output(path: string): void {
    this.inner.render;
    this.inner.output('png', path, console.error);
  }
}
