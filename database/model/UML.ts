import plantuml from 'node-plantuml';

import { strict as assert } from 'assert';
import fs from 'fs';
import { Arrow, kind_to_arrows, Relation, RelationKind } from './ER';

export {
  Class,
  Multiplicity,
  Association,
  UML,
  mul,
  aggregation,
  composition,
  association,
  association_from_self_relation,
};

interface Class {
  label: string;
  attrs?: Array<string>;
  keys?: Array<string>;
}

function class_to_uml(cls: Class): string {
  return [
    `class ${cls.label} {`,
    ...(cls.attrs ?? []).map((attr) =>
      cls.keys?.includes(attr) ? `${attr} PK` : attr
    ),
    '}',
  ].join('\n');
}

type Notation = Aggregation | Composition | Multiplicity;

interface Aggregation {
  kind: 'aggregation';
}

interface Composition {
  kind: 'composition';
}

const aggregation = {
  kind: 'aggregation',
};

const composition = {
  kind: 'composition',
};

interface Multiplicity {
  kind: 'multiplicity';
  min: number;
  max?: number;
}

function mul(min?: number, max?: number): Multiplicity {
  return {
    kind: 'multiplicity',
    min: min ?? 0,
    max,
  };
}

function arrowhead(notation: Notation): string {
  switch (notation.kind) {
    case 'aggregation':
      return 'o';
    case 'composition':
      return '*';
    default:
      return '';
  }
}

function head_label(notation: Notation, text?: string): string {
  const tokens = [];
  if (
    notation.kind === 'multiplicity' &&
    (notation.min != 1 || notation.max != 1)
  ) {
    tokens.push(`${notation.min}..${notation.max ?? '*'}`);
  }

  if (text) {
    tokens.push(text);
  }

  if (tokens.length > 0) {
    return '"' + tokens.join(' ') + '"';
  } else {
    return '';
  }
}

interface Association {
  label: string;
  from: [Class, Notation, string?];
  to: [Class, Notation, string?];
  class?: Class;
}

function association_from_self_relation(rel: Relation): Association {
  assert(rel.arrows.length === 2);

  const [from_entity, from_arrow, from_text] = rel.arrows[0];
  const [to_entity, to_arrow, to_text] = rel.arrows[1];

  assert(from_entity === to_entity);

  return {
    label: rel.label,
    from: [from_entity, arrow_to_mul(from_arrow), from_text],
    to: [to_entity, arrow_to_mul(to_arrow), to_text],
  };
}

function arrow_to_mul(arrow: Arrow): Multiplicity {
  switch (arrow) {
    case Arrow.Many:
      return mul(0);
    case Arrow.One:
      return mul(0, 1);
    case Arrow.RI:
      return mul(1, 1);
  }
}

function association(
  label: string,
  from: Class,
  to: Class,
  kind: RelationKind = RelationKind.ManyMany
): Association {
  const [from_arrow, to_arrow] = kind_to_arrows(kind);
  return {
    label,
    from: [from, arrow_to_mul(from_arrow)],
    to: [to, arrow_to_mul(to_arrow)],
  };
}

function association_to_uml(assoc: Association): string {
  if (assoc.class) {
    return association_with_class(assoc as Required<Association>);
  } else {
    return plain_association(assoc);
  }
}

function association_with_class(assoc: Association & { class: Class }): string {
  const label = assoc.label.replace('-', '');
  const entry = `circle ${label}`;
  const cls = class_to_uml(assoc.class);
  const cls_arrow: [Class, Notation] = [assoc.class, mul(1, 1)];
  const arrows = [assoc.from, assoc.to, cls_arrow].map(
    ([cls, notation, text]) =>
      `${cls.label} ${head_label(notation, text)} ${arrowhead(
        notation
      )}-- ${label}`
  );

  return [entry, cls, ...arrows].join('\n');
}

function plain_association(assoc: Association): string {
  const [from_class, from_notation, from_text] = assoc.from;
  const [to_class, to_notation, to_text] = assoc.to;

  return `${from_class.label} ${head_label(
    from_notation,
    from_text
  )} ${arrowhead(from_notation)}--${arrowhead(to_notation)} ${head_label(
    to_notation,
    to_text
  )} ${to_class.label}`;
}

interface Subclass {
  base: Class;
  child: Class;
}

function subclass_to_uml(subclass: Subclass): string {
  const { base, child: sub } = subclass;
  return `${base.label} <|-- ${sub.label}`;
}

class UML {
  private id: string;
  private classes!: Set<Class>;
  private associations!: Set<Association>;
  private subclasses: Set<Subclass>;

  constructor(id: string) {
    this.id = id;
    this.classes = new Set();
    this.associations = new Set();
    this.subclasses = new Set();
  }

  add_entity(cls: Class): void {
    this.add_class(cls);
  }

  add_class(cls: Class): void {
    this.classes.add(cls);
  }

  add_association(assoc: Association): void {
    assert(this.classes.has(assoc.from[0]));
    assert(this.classes.has(assoc.to[0]));

    this.associations.add(assoc);
  }

  add_relation(assoc: Association): void {
    this.add_association(assoc);
  }

  add_subclass(subclass: Subclass): void {
    this.subclasses.add(subclass);
  }

  add_isa(subclass: Subclass): void {
    this.subclasses.add(subclass);
  }

  output(path: string): void {
    const PARAMS = ['skinparam linetype ortho', 'allow_mixing'];

    const uml = [
      '@startuml',
      ...PARAMS,
      ...Array.from(this.classes).map(class_to_uml),
      ...Array.from(this.subclasses).map(subclass_to_uml),
      ...Array.from(this.associations).map(association_to_uml),
      '@enduml',
    ].join('\n');

    const gen = plantuml.generate(uml, { format: 'png' });
    gen.out.pipe(fs.createWriteStream(path));
  }
}
