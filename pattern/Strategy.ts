interface Coord {}
interface Component {}

class Composition {
  private _compositor: Compositor
  private _components: Component
  private _componentCount: number
  private _lineWidth: number
  private _lineBreaks: number[]
  private _lineCount: number

  repair(): void {
    let natural: Coord[];
    let stretchability: Coord[];
    let shrinkability: Coord[];
    let componentCount: number;
    let breaks: number[];

    // ...

    let breakCount = this._compositor.compose(
      natural,
      stretchability,
      shrinkability,
      componentCount,
      this._lineWidth,
      breaks
    );
  }
}

class Compositor {
  constructor() {}

  compose(
    natural: Coord[], 
    stretch: Coord[], 
    shrink: Coord[],
    componentCount: number,
    lineWidth: number,
    breaks: number[]
  ): number { return null; }
}

class SimpleCompositor extends Compositor {}
class TexCompositor extends Compositor {}
class ArrayCompositor extends Compositor {}