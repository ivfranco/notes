import { Ident, ty, Ty } from '../ODL';
import { describe, it } from 'mocha';
import { assert } from 'chai';

describe('type parser', function () {
  it('parses primitive types', function () {
    for (const primitive of ['string', 'integer', 'float', 'boolean']) {
      assert.hasAllKeys(ty.parse(primitive), {
        status: true,
        value: primitive,
      });
    }
  });

  it('parses Set type', function () {
    const result = ty.parse('   Set< string>');
    assert.deepStrictEqual(result, {
      status: true,
      value: { kind: 'Set', inner: 'string' },
    });
  });

  it('parses List type', function () {
    const result = ty.parse('   List<  Set<   integer  >  >');
    assert.deepStrictEqual(result, {
      status: true,
      value: { kind: 'List', inner: { kind: 'Set', inner: 'integer' } },
    });
  });

  it('parses Array type', function () {
    const result = ty.parse(' Array<Set<boolean>, 10>');
    assert.deepStrictEqual(result, {
      status: true,
      value: {
        kind: 'Array',
        len: 10,
        inner: {
          kind: 'Set',
          inner: 'boolean',
        },
      },
    });
  });

  it('parses enum type', function () {
    const result = ty.parse('enum   Triple { variant0, variant1,   variant2 }');
    assert.deepEqual(result, {
      status: true,
      value: {
        kind: 'Enum',
        ident: 'Triple',
        variants: ['variant0', 'variant1', 'variant2'],
      },
    });
  });

  it('parses Struct type', function () {
    const result = ty.parse(`Struct Movie {
      string street,
      Date    release
    }`);

    const properties: Map<Ident, Ty> = new Map();
    properties.set('street', 'string');
    properties.set('release', {
      kind: 'Class',
      ident: 'Date',
    });

    assert.deepEqual(result, {
      status: true,
      value: {
        kind: 'Struct',
        ident: 'Movie',
        properties,
      },
    });
  });
});
