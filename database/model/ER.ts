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
  keys?: Array<string>;
  is_weak?: boolean;
}

enum Arrow {
  Many,
  One,
  RI,
}

const UNDIRECTIONAL: graphviz.Options = {
  arrowhead: 'none'
};

const DIRECTIONAL: graphviz.Options = {
  dir: 'forward',
  arrowhead: 'normal',
};

const RI: graphviz.Options = {
  dir: 'forward',
  arrowhead: 'curve',
  arrowsize: 1.5,
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
    case Arrow.RI:
      Object.assign(style, RI);
  }

  if (text) {
    Object.assign(style, { label: text });
  }

  return style;
}

interface Relation {
  label: string,
  attrs?: Array<string>,
  arrows: Array<[Entity, Arrow, string?]>;
  is_support?: boolean;
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
      // node.js graphviz has special undocumented syntax for html-like labels
      // https://github.com/glejeune/node-graphviz/blob/f552e1fd2c363c95efd518b1eae1167020b01d2d/lib/deps/attributs.js#L195
      const label = entity.keys?.includes(attr_name) ? `!<U>${attr_name}</U>` : attr_name;
      const attr = cluster.addNode(`${entity.label}.${attr_name}`, { label });
      cluster.addEdge(entry, attr, { len: DEFAULT_EDGE_LENGTH / 2 });
    });

    return entry;
  }

  add_entity(entity: Entity): graphviz.Node {
    const entry = this.add_cluster(entity, 'box');
    if (entity.is_weak) {
      entry.set('peripheries', 2);
    }
    return entry;
  }

  add_relation(relation: Relation): graphviz.Node {
    const g = this.inner;

    let node;
    if (relation.attrs) {
      node = this.add_cluster(relation, 'diamond');
    } else {
      node = g.addNode(relation.label, { shape: 'diamond' });
    }

    if (relation.is_support) {
      node.set('peripheries', 2);
    }

    for (const [target, arrow, text] of relation.arrows) {
      g.addEdge(node, target.label, arrow_style(arrow, text));
    }

    return node;
  }

  add_isa(isa: ISA): void {
    const ISA_OPTIONS: graphviz.Options = {
      label: 'isa',
      shape: 'triangle',
      margin: 0,
    };

    const g = this.inner;
    const { base, child } = isa;

    const node = g.addNode(`${base.label}-ISA-${child.label}`, ISA_OPTIONS);
    g.addEdge(node, base.label, Object.assign(DIRECTIONAL, { len: DEFAULT_EDGE_LENGTH / 2 }));
    g.addEdge(node, child.label, Object.assign(UNDIRECTIONAL, { len: DEFAULT_EDGE_LENGTH / 2 }));
  }

  output(path: string): void {
    this.inner.render;
    this.inner.output('png', path, console.error);
  }
}
