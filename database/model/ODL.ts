import * as p from 'parsimmon';
import { Parser } from 'parsimmon';

export { ty, Ty, Ident };

function lexeme<T>(parser: Parser<T>): Parser<T> {
  return p.regexp(/\s*/).then(parser);
}

function lexstr(str: string): Parser<string> {
  return lexeme(p.string(str));
}

const ident: Parser<Ident> = lexeme(
  p
    .seq(p.letter, p.alt(p.letter, p.digit).many())
    .map(([f, s]) => [f, ...s].join(''))
);

const integer = lexeme(p.digits.map((digits) => Number.parseInt(digits)));

type Ident = string;
type ClassIdent = string;

const ty: Parser<Ty> = lexeme(
  p.lazy(() =>
    p.alt<Ty>(primitive_t, set_t, list_t, array_t, enum_t, struct_t, class_t)
  )
);

const primitive_t = p.alt(
  p.string('boolean'),
  p.string('float'),
  p.string('integer'),
  p.string('string')
);

type Ty = PrimitiveT | SetT | ListT | ArrayT | EnumT | StructT | ClassT;

type PrimitiveT = 'string' | 'float' | 'integer' | 'boolean';

interface SetT {
  kind: 'Set';
  inner: Ty;
}

interface ListT {
  kind: 'List';
  inner: Ty;
}

interface ArrayT {
  kind: 'Array';
  inner: Ty;
  len: number;
}

interface EnumT {
  kind: 'Enum';
  ident: Ident;
  variants: Array<Ident>;
}

interface StructT {
  kind: 'Struct';
  ident: Ident;
  properties: Map<Ident, Ty>;
}

interface ClassT {
  kind: 'Class';
  ident: ClassIdent;
}

const set_t: Parser<SetT> = ty.wrap(p.string('Set<'), lexstr('>')).map((ty) => {
  return { kind: 'Set', inner: ty };
});

const list_t: Parser<ListT> = ty
  .wrap(p.string('List<'), lexstr('>'))
  .map((ty) => {
    return { kind: 'List', inner: ty };
  });

const array_t: Parser<ArrayT> = p
  .seq(ty, lexstr(','), integer)
  .wrap(p.string('Array<'), lexstr('>'))
  .map(([inner, _, len]) => ({ kind: 'Array', inner, len }));

const enum_t: Parser<EnumT> = p
  .seq(
    p.string('enum'),
    ident,
    p.sepBy(ident, lexstr(',')).wrap(lexstr('{'), lexstr('}'))
  )
  .map(([_, ident, variants]) => ({ kind: 'Enum', ident, variants }));

const struct_t: Parser<StructT> = p
  .seq(
    p.string('Struct'),
    ident,
    p.sepBy(p.seq(ty, ident), lexstr(',')).wrap(lexstr('{'), lexstr('}'))
  )
  .map(([_, ident, pairs]) => ({
    kind: 'Struct',
    ident,
    properties: new Map(pairs.map(([ty, ident]) => [ident, ty])),
  }));

const class_t: Parser<ClassT> = p
  .seq(p.range('A', 'Z'), p.alt(p.letter, p.digit).many())
  .map(([h, t]) => ({ kind: 'Class', ident: [h, ...t].join('') }));
