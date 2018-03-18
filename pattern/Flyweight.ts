export {}

interface Window {}
interface Font {}

abstract class GlyphContext {
  private _index: number
  private _fonts: Map<number, Font>

  constructor() {}

  abstract next(step: number): void
  abstract insert(quantity: number): void
  
  abstract getFont(): Font
  abstract setFont(font: Font, span: number): void
}

abstract class Glyph {
  constructor() {}

  abstract draw(window: Window, context: GlyphContext): void
  abstract setFont(font: Font, context: GlyphContext): void
  abstract getFont(context: GlyphContext): Font

  abstract first(context: GlyphContext): void
  abstract next(context: GlyphContext): void
  abstract isDone(context: GlyphContext): boolean
  abstract current(context: GlyphContext): Glyph

  abstract insert(glyph: Glyph, context: GlyphContext): void
  abstract remove(context: GlyphContext): void
}

abstract class Character extends Glyph {
  private _charcode: number

  constructor(char: number) {
    super();
  }
}

abstract class GlyphFactory {
  static NCHARCODES = 128
  private _characters: Array<Character>

  constructor() {
    this._characters.length = GlyphFactory.NCHARCODES;
    this._characters.fill(null);
  }

  abstract createCharacter(char: number): Character {
    if (!this._characters[char]) {
      this._characters[char] = new Character(c);
    }

    return this._characters[char];
  }

  abstract createGlyph(): Glyph
}