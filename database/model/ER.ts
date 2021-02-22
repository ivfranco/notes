import graphviz from 'graphviz';

export {
  Arrow, Entity, Relation, ERModel, RelationKind, binary_relation, isa
};

const DEFAULT_EDGE_LENGTH = 1.0;

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

const UNDIRECTIONAL: graphviz.Options = {
  arrowhead: 'none'
};

const DIRECTIONAL: graphviz.Options = {
  dir: 'forward',
  arrowhead: 'normal',
};

function arrow_style(arrow: Arrow, text?: string): graphviz.Options {
  const style = {};

  switch (arrow) {
    case Arrow.Many:
      Object.assign(style, UNDIRECTIONAL);
      break;
    case Arrow.One:
      Object.assign(style, DIRECTIONAL);
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
  ManyMany = 0b00,
  ManyOne = 0b01,
  OneMany = 0b10,
  OneOne = 0b11,
}

function binary_relation(
  label: string,
  from: Entity,
  to: Entity,
  kind: RelationKind = RelationKind.ManyMany
): Relation {
  const from_arrow = kind & 0b10 ? Arrow.One : Arrow.Many;
  const to_arrow = kind & 0b01 ? Arrow.One : Arrow.Many;
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
  private inner: graphviz.Graph;

  constructor(label: string) {
    const GRAPH_OPTIONS: graphviz.Options = {
      dpi: 240,
      layout: 'neato',
      overlap: 'scale',
    };

    // id of a graph may not contain `-`, '.' or space.
    this.inner = graphviz.graph(label.replace(/[-.\s]/, '_'));
    for (const [key, value] of Object.entries(GRAPH_OPTIONS)) {
      this.inner.set(key, value);
    }
  }

  private add_cluster(entity: Entity, shape: string): graphviz.Node {
    const g = this.inner;

    // id of a graph may not contain `-`, '.' or space.
    const cluster = g.addCluster(`Cluster_${entity.label.replace(/[-.\s]/, '_')}`);
    // remove subgraph borders in dot mode, ignored by neato
    cluster.set('style', 'filled');
    cluster.set('color', 'none');

    const entry = cluster.addNode(entity.label, { shape: shape });

    entity.attrs?.forEach(attr_name => {
      const attr = cluster.addNode(`${entity.label}.${attr_name}`, { label: attr_name });
      cluster.addEdge(entry, attr, { len: DEFAULT_EDGE_LENGTH / 2 });
    });

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
    const ISA_OPTIONS: graphviz.Options = {
      label: 'isa',
      shape: 'triangle',
      margin: 0,
    };

    const g = this.inner;
    const { base, child } = isa;

    const entry = g.addNode(`${base.label}-${child.label}-ISA`, ISA_OPTIONS);
    g.addEdge(entry, base.label, Object.assign(DIRECTIONAL, { len: DEFAULT_EDGE_LENGTH / 2 }));
    g.addEdge(entry, child.label, Object.assign(UNDIRECTIONAL, { len: DEFAULT_EDGE_LENGTH / 2 }));
  }

  output(path: string): void {
    this.inner.render;
    this.inner.output('png', path, console.error);
  }
}
