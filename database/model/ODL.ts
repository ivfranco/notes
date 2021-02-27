/* eslint-disable @typescript-eslint/no-unused-vars */

import P from 'parsimmon';
import { Parser } from 'parsimmon';

export {
  cls,
  Ident,
  KEYWORDS,
  odl_parser,
  PRIMITIVES,
  ty,
  Ty,
  validate,
  VKEYWORDS,
};

interface Primitives {
  STRING: 'string';
  INTEGER: 'integer';
  FLOAT: 'float';
  BOOLEAN: 'boolean';
}

const PRIMITIVES: Primitives = {
  STRING: 'string',
  INTEGER: 'integer',
  FLOAT: 'float',
  BOOLEAN: 'boolean',
};

type ValueOf<T> = T[keyof T];
type PrimitiveT = ValueOf<Primitives>;

interface Keywords extends Primitives {
  SET: 'Set';
  BAG: 'Bag';
  LIST: 'List';
  ARRAY: 'Array';
  DICTIONARY: 'Dictionary';
  ENUM: 'enum';
  STRUCT: 'Struct';
  ATTRIBUTE: 'attribute';
  RELATIONSHIP: 'relationship';
  INVERSE: 'inverse';
  CLASS: 'class';
  KEY: 'key';
}

const KEYWORDS: Keywords = {
  ...PRIMITIVES,
  SET: 'Set',
  BAG: 'Bag',
  LIST: 'List',
  ARRAY: 'Array',
  DICTIONARY: 'Dictionary',
  ENUM: 'enum',
  STRUCT: 'Struct',
  ATTRIBUTE: 'attribute',
  RELATIONSHIP: 'relationship',
  INVERSE: 'inverse',
  CLASS: 'class',
  KEY: 'key',
};

// virtual keywords, some of them will not appear in the language
interface VKeywords extends Keywords {
  CONTEXTUAL: 'Contextual';
}

const VKEYWORDS: VKeywords = {
  ...KEYWORDS,
  CONTEXTUAL: 'Contextual',
};

function opt<T>(parser: Parser<T>): Parser<T | undefined> {
  return parser.fallback(undefined);
}

function lexeme<T>(parser: Parser<T>): Parser<T> {
  return P.optWhitespace.then(parser);
}

function lexstr<T extends string>(str: T): Parser<T> {
  return lexeme(P.string(str));
}

function sep_by_comma<T>(parser: Parser<T>): Parser<T[]> {
  return P.sepBy(parser, lexstr(','));
}

type Ident = string;
type TypeIdent = string;

const valid_char = P.alt(P.letter, P.digit, P.oneOf('-_'));

const raw_ident: Parser<Ident> = P.seqMap(P.letter, valid_char.many(), (f, s) =>
  [f, ...s].join('')
);

const ident: Parser<Ident> = lexeme(raw_ident);

const raw_type_ident: Parser<TypeIdent> = P.seqMap(
  P.range('A', 'Z'),
  valid_char.many(),
  (h, t) => [h, ...t].join('')
);

const type_ident: Parser<TypeIdent> = lexeme(raw_type_ident);

const array_len = lexeme(P.digits.map((digits) => Number.parseInt(digits)));

const ty: Parser<Ty> = lexeme(
  P.lazy(() =>
    P.alt<Ty>(
      primitive_t,
      set_t,
      bag_t,
      list_t,
      array_t,
      dictionary_t,
      enum_t,
      struct_t,
      contextual_t
    )
  )
);

const primitive_t: Parser<PrimitiveT> = P.alt(
  ...Object.values(PRIMITIVES).map((prim) => P.string(prim))
);

type Ty =
  | PrimitiveT
  | SetT
  | BagT
  | ListT
  | ArrayT
  | DictionaryT
  | EnumT
  | StructT
  | Contextual;

function is_primitive(type: Ty): type is PrimitiveT {
  return Object.values(PRIMITIVES).includes(type);
}

function has_inner(
  type: Exclude<Ty, PrimitiveT>
): type is SetT | ListT | ArrayT {
  return (
    type.kind === KEYWORDS.SET ||
    type.kind === KEYWORDS.LIST ||
    type.kind === KEYWORDS.ARRAY
  );
}

interface SetT {
  kind: typeof KEYWORDS.SET;
  inner: Ty;
}

interface BagT {
  kind: typeof KEYWORDS.BAG;
  inner: Ty;
}

interface ListT {
  kind: typeof KEYWORDS.LIST;
  inner: Ty;
}

interface ArrayT {
  kind: typeof KEYWORDS.ARRAY;
  inner: Ty;
  len: number;
}

interface DictionaryT {
  kind: typeof KEYWORDS.DICTIONARY;
  key: Ty;
  value: Ty;
}

interface EnumT {
  kind: typeof KEYWORDS.ENUM;
  ident: Ident;
  variants: Array<Ident>;
}

interface StructT {
  kind: typeof KEYWORDS.STRUCT;
  ident: Ident;
  fields: Map<Ident, Ty>;
}

interface Contextual {
  kind: typeof VKEYWORDS.CONTEXTUAL;
  class: TypeIdent;
  ident?: TypeIdent;
}

const set_t: Parser<SetT> = ty
  .wrap(P.string(`${KEYWORDS.SET}<`), lexstr('>'))
  .map((ty) => {
    return { kind: KEYWORDS.SET, inner: ty };
  });

const bag_t: Parser<BagT> = ty
  .wrap(P.string(`${KEYWORDS.BAG}<`), lexstr('>'))
  .map((ty) => {
    return { kind: KEYWORDS.BAG, inner: ty };
  });

const list_t: Parser<ListT> = ty
  .wrap(P.string(`${KEYWORDS.LIST}<`), lexstr('>'))
  .map((ty) => {
    return { kind: KEYWORDS.LIST, inner: ty };
  });

const array_t: Parser<ArrayT> = P.seqMap(
  ty,
  lexstr(','),
  array_len,
  (inner, _, len) => ({
    kind: KEYWORDS.ARRAY,
    inner,
    len,
  })
).wrap(P.string(`${KEYWORDS.ARRAY}<`), lexstr('>'));

const dictionary_t: Parser<DictionaryT> = P.seqMap(
  ty,
  lexstr(','),
  ty,
  (key, _, value) => ({
    kind: KEYWORDS.DICTIONARY,
    key,
    value,
  })
).wrap(P.string(`${KEYWORDS.DICTIONARY}<`), lexstr('>'));

const enum_t: Parser<EnumT> = P.seqMap(
  P.string(KEYWORDS.ENUM),
  ident,
  P.sepBy1(ident, lexstr(',')).wrap(lexstr('{'), lexstr('}')),
  (kind, ident, variants) => ({ kind, ident, variants })
);

const struct_t: Parser<StructT> = P.seqMap(
  P.string(KEYWORDS.STRUCT),
  ident,
  P.sepBy(P.seq(ty, ident), lexstr(',')).wrap(lexstr('{'), lexstr('}')),
  (kind, ident, pairs) => ({
    kind,
    ident,
    fields: new Map(pairs.map(([ty, ident]) => [ident, ty])),
  })
);

function scope<T>(parser: Parser<T>): Parser<T> {
  return P.seqMap(P.string('::'), parser, (_, r) => r);
}

const contextual_t: Parser<Contextual> = P.seqMap(
  type_ident,
  opt(scope(type_ident)),
  (cls, ident) => {
    const c: Contextual = {
      kind: VKEYWORDS.CONTEXTUAL,
      class: cls,
    };

    if (ident) {
      Object.assign(c, { ident });
    }

    return c;
  }
);

type Property = Attribute | Relationship;

interface Attribute {
  kind: typeof KEYWORDS.ATTRIBUTE;
  ident: Ident;
  type: Ty;
}

interface Relationship {
  kind: typeof KEYWORDS.RELATIONSHIP;
  ident: Ident;
  type: Ty;
  inverse?: {
    class: TypeIdent;
    relationship: Ident;
  };
}

interface Class {
  ident: TypeIdent;
  properties: Array<Property>;
  keys?: Array<Ident>;
  base?: TypeIdent;
}

const attribute = P.seqMap(
  lexstr(KEYWORDS.ATTRIBUTE),
  ty,
  ident,
  (_, type, ident) => ({
    kind: KEYWORDS.ATTRIBUTE,
    ident,
    type,
  })
);

const inverse = P.seqMap(
  lexstr(KEYWORDS.INVERSE),
  type_ident,
  scope(raw_ident),
  (_0, cls, relationship) => ({
    class: cls,
    relationship,
  })
);

function valid_rel_type(type: Ty): boolean {
  function is_bare_class(type: Ty): boolean {
    return (
      !is_primitive(type) &&
      type.kind === 'Contextual' &&
      type.ident === undefined
    );
  }

  if (is_primitive(type)) {
    return false;
  }

  if (type.kind === VKEYWORDS.CONTEXTUAL) {
    return is_bare_class(type);
  }

  if (type.kind === KEYWORDS.DICTIONARY) {
    return is_bare_class(type.key);
  }

  if (has_inner(type)) {
    return is_bare_class(type.inner);
  }

  return false;
}

const relationship: Parser<Relationship> = P.seqMap(
  lexstr(KEYWORDS.RELATIONSHIP),
  ty.assert(
    valid_rel_type,
    'Invalid relationship type, must be Class or Set|Bag|List|Array<Class>'
  ),
  ident,
  opt(inverse),
  (kind, type, ident, inverse) => {
    const r = {
      kind,
      type,
      ident,
    };

    if (inverse) {
      Object.assign(r, { inverse });
    }

    return r;
  }
);

const property: Parser<Property> = P.alt(attribute, relationship).skip(
  lexstr(';')
);

const keys: Parser<Ident[]> = P.seqMap(
  lexstr(KEYWORDS.KEY),
  sep_by_comma(ident).wrap(lexstr('('), lexstr(')')),
  (_, keys) => keys
).wrap(lexstr('('), lexstr(')'));

const ext = P.seqMap(lexstr('extends'), type_ident, (_, ty) => ty);

const comment = P.seq(
  lexstr('//'),
  P.takeWhile((c) => c != '\r' && c != '\n'),
  P.end
).result(null);

const cls: Parser<Class> = P.seqMap(
  lexstr(KEYWORDS.CLASS),
  ident,
  opt(keys),
  opt(ext),
  property.or(comment).many().wrap(lexstr('{'), lexstr('}')),
  (_, ident, keys, base, prop) => {
    const properties = prop.filter((p) => p !== null) as Array<Property>;
    const c = {
      ident,
      properties,
    };

    if (keys) {
      Object.assign(c, { keys });
    }

    if (base) {
      Object.assign(c, { base });
    }

    return c;
  }
);

class ODL {
  classes: Array<Class>;

  constructor(classes: Array<Class>) {
    this.classes = classes;
  }
}

const odl_parser: Parser<ODL> = cls
  .skip(lexstr(';'))
  .or(comment)
  .many()
  .trim(P.optWhitespace)
  .map((cls_or_null) => {
    const classes = cls_or_null.filter((c) => c !== null) as Array<Class>;
    return new ODL(classes);
  });

// TODO: add semantic checks (e.g. key must be a property)
function validate(odl: string): void {
  const _odl = odl_parser.tryParse(odl);
}
