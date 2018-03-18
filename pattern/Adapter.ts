export {}

class Point {
  constructor(x: Coord, y: Coord) {}

  initialize(x: Coord, y: Coord) {}
}

type Coord = number

abstract class Manipulator {}

class TextManipulator extends Manipulator {
  constructor(textShape: Shape) {
    super();
  }
}

abstract class Shape {
  constructor() {}

  abstract boundingBox(bottomLeft: Point, topRight: Point): void
  abstract createManipulator(): Manipulator
}

abstract class TextView {
  constructor() {}

  abstract getOrigin(x: Coord, y: Coord): void
  abstract getExtent(width: Coord, height: Coord): void
  abstract isEmpty(): boolean
}

abstract class TextShape extends TextView implements Shape {
  constructor() {
    super();
  }

  boundingBox(bottomLeft: Point, topRight: Point) {
    let bottom: Coord, left: Coord, width: Coord, height: Coord;

    this.getOrigin(bottom, left);
    this.getExtent(width, height);

    bottomLeft.initialize(bottom, left);
    topRight.initialize(bottom + height, left + width);
  }

  createManipulator(): Manipulator {
    return new TextManipulator(this);
  }
} 

abstract class TextShapeObject extends Shape {
  private _text: TextView

  constructor(text: TextView) {
    super();
    this._text = text;
  }

  boundingBox(bottomLeft: Point, topRight: Point) {
    let bottom: Coord, left: Coord, width: Coord, height: Coord;

    this._text.getOrigin(bottom, left);
    this._text.getExtent(width, height);

    bottomLeft.initialize(bottom, left);
    topRight.initialize(bottom + height, left + width);
  }

  isEmpty() {
    return this._text.isEmpty()
  }

  createManipulator(): Manipulator {
    return new TextManipulator(this);
  }
}