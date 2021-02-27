import {
  Ident,
  KEYWORDS,
  ty,
  Ty,
  PRIMITIVES,
  cls,
  odl_parser,
  VKEYWORDS,
} from '../ODL';
import { describe, it } from 'mocha';
import { assert } from 'chai';

describe('type parser', function () {
  it('parses primitive types', function () {
    for (const primitive of Object.values(PRIMITIVES)) {
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
      value: { kind: KEYWORDS.SET, inner: KEYWORDS.STRING },
    });
  });

  it('parses Bag type', function () {
    const result = ty.parse('   Bag<   boolean  >');
    assert.deepStrictEqual(result, {
      status: true,
      value: { kind: KEYWORDS.BAG, inner: KEYWORDS.BOOLEAN },
    });
  });

  it('parses List type', function () {
    const result = ty.parse('   List<  Set<   integer  >  >');
    assert.deepStrictEqual(result, {
      status: true,
      value: {
        kind: KEYWORDS.LIST,
        inner: { kind: KEYWORDS.SET, inner: KEYWORDS.INTEGER },
      },
    });
  });

  it('parses Array type', function () {
    const result = ty.parse(' Array<Set<boolean>, 10>');
    assert.deepStrictEqual(result, {
      status: true,
      value: {
        kind: KEYWORDS.ARRAY,
        len: 10,
        inner: {
          kind: KEYWORDS.SET,
          inner: KEYWORDS.BOOLEAN,
        },
      },
    });
  });

  it('parses Dictionary type', function () {
    const result = ty.parse('   Dictionary<integer, Array<boolean, 20>>');
    assert.deepStrictEqual(result, {
      status: true,
      value: {
        kind: KEYWORDS.DICTIONARY,
        key: KEYWORDS.INTEGER,
        value: {
          kind: KEYWORDS.ARRAY,
          len: 20,
          inner: KEYWORDS.BOOLEAN,
        },
      },
    });
  });

  it('parses enum type', function () {
    const result = ty.parse('enum   Triple { variant0, variant1,   variant2 }');
    assert.deepStrictEqual(result, {
      status: true,
      value: {
        kind: KEYWORDS.ENUM,
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
      kind: VKEYWORDS.CONTEXTUAL,
      class: 'Date',
    });

    assert.deepStrictEqual(result, {
      status: true,
      value: {
        kind: KEYWORDS.STRUCT,
        ident: 'Movie',
        fields: properties,
      },
    });
  });
});

describe('class parser', function () {
  it('parses keys', function () {
    const result = cls.parse(`class Movie (key (title, year)) {
      }`);

    assert.deepStrictEqual(result, {
      status: true,
      value: {
        ident: 'Movie',
        keys: ['title', 'year'],
        properties: [],
      },
    });
  });

  it('parses base class', function () {
    const result = cls.parse('class Cartoon extends Movie {}');
    assert.deepStrictEqual(result, {
      status: true,
      value: {
        ident: 'Cartoon',
        properties: [],
        base: 'Movie',
      },
    });
  });

  it('parses attributes', function () {
    const result = cls.parse(`class Movie {
      attribute string title;
      attribute integer year;
      attribute enum Genre { drama, comedy, sciFi, teen } genre;
      attribute Set<Star> stars;
    }`);

    assert.deepStrictEqual(result, {
      status: true,
      value: {
        ident: 'Movie',
        properties: [
          {
            kind: KEYWORDS.ATTRIBUTE,
            type: KEYWORDS.STRING,
            ident: 'title',
          },
          {
            kind: KEYWORDS.ATTRIBUTE,
            type: KEYWORDS.INTEGER,
            ident: 'year',
          },
          {
            kind: KEYWORDS.ATTRIBUTE,
            type: {
              kind: KEYWORDS.ENUM,
              ident: 'Genre',
              variants: ['drama', 'comedy', 'sciFi', 'teen'],
            },
            ident: 'genre',
          },
          {
            kind: KEYWORDS.ATTRIBUTE,
            type: {
              kind: KEYWORDS.SET,
              inner: {
                kind: VKEYWORDS.CONTEXTUAL,
                class: 'Star',
              },
            },
            ident: 'stars',
          },
        ],
      },
    });
  });

  it('parses relationships', function () {
    const result = cls.parse(`class Movie {
      relationship Set<Star> stars;
      relationship Studio studio inverse Studio::produced;
    }`);

    assert.deepStrictEqual(result, {
      status: true,
      value: {
        ident: 'Movie',
        properties: [
          {
            kind: KEYWORDS.RELATIONSHIP,
            type: {
              kind: KEYWORDS.SET,
              inner: {
                kind: VKEYWORDS.CONTEXTUAL,
                class: 'Star',
              },
            },
            ident: 'stars',
          },
          {
            kind: KEYWORDS.RELATIONSHIP,
            type: {
              kind: VKEYWORDS.CONTEXTUAL,
              class: 'Studio',
            },
            ident: 'studio',
            inverse: {
              class: 'Studio',
              relationship: 'produced',
            },
          },
        ],
      },
    });
  });
});

describe('ODL parser', function () {
  it('parses textbook example', function () {
    const text = `
    class Movie {
      attribute string title;
      attribute integer year;
      attribute integer length;
      attribute enum Genres
        {drama, comedy, sciFi, teen} genre;
      relationship Set<Star> stars
        inverse Star::starredIn;
        // test
      relationship Studio ownedBy
        inverse Studio::owns;
    };

    // test
    class Star {
      attribute string name;
      attribute Struct Addr
        {string street, string city} address;
      relationship Set<Movie> starredIn
        inverse Movie::stars;
    };

    class Studio {
      attribute string name;
      attribute Star::Addr address;
      relationship Set<Movie> owns
        inverse Movie::ownedBy;
    };
    `;

    odl_parser.tryParse(text);
  });
});
