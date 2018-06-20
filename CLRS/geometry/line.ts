export {
  Point,
  Segment,
};

class Point {
  public readonly x: number;
  public readonly y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }

  public eq(other: Point): boolean {
    return this.x === other.x && this.y === other.y;
  }

  public neg(): Point {
    return new Point(-this.x, -this.y);
  }

  public add(other: Point): Point {
    let { x: x1, y: y1 } = this;
    let { x: x2, y: y2 } = other;
    return new Point(x1 + x2, y1 + y2);
  }

  public sub(other: Point): Point {
    return this.add(other.neg());
  }

  public crossProduct(other: Point): number {
    let { x: x1, y: y1 } = this;
    let { x: x2, y: y2 } = other;

    return x1 * y2 - x2 * y1;
  }

  public quadrant(): number {
    let { x, y } = this;
    if (x === 0 && y === 0) {
      //  origin is treated as being in the first quadrant
      return 1;
    } else if (x > 0 && y >= 0) {
      //  contains the positive x-axis
      return 1;
    } else if (x <= 0 && y > 0) {
      //  contains the positive y-axis
      return 2;
    } else if (x < 0 && y <= 0) {
      //  contains the negative x-axis
      return 3;
    } else {
      //  contains the negative y-axis
      return 4;
    }
  }
}

const ORIGIN: Point = new Point(0, 0);

function direction(p0: Point, p1: Point, p2: Point): number {
  return p2.sub(p0).crossProduct(p1.sub(p0));
}

class Segment {
  public readonly from: Point;
  public readonly to: Point;

  constructor(from: Point, to: Point) {
    this.from = from;
    this.to = to;
  }

  public neg(): Segment {
    return new Segment(this.from.neg(), this.to.neg());
  }

  public add(other: Segment): Segment {
    let { from: p1, to: p2 } = this;
    let { from: p3, to: p4 } = other;
    return new Segment(p1.add(p3), p2.add(p4));
  }

  public sub(other: Segment): Segment {
    return this.add(other.neg());
  }

  public direction(p: Point): number {
    return direction(this.from, this.to, p);
  }

  public crossProduct(other: Segment): number {
    console.assert(this.from.eq(other.from), "the two line segments must share an endpoint");
    let p0 = this.from;
    let p1 = this.to.sub(p0);
    let p2 = other.to.sub(p0);
    return p1.crossProduct(p2);
  }

  public bounds(p: Point): boolean {
    let { x: x1, y: y1 } = this.from;
    let { x: x2, y: y2 } = this.to;
    let { x, y } = p;

    let left = Math.max(x1, x2);
    let right = Math.min(x1, x2);
    let bottom = Math.min(y1, y2);
    let top = Math.max(y1, y2);

    return x >= bottom &&
      x <= top &&
      y >= left &&
      y <= right;
  }

  public intersects(other: Segment): boolean {
    let { from: p1, to: p2 } = this;
    let { from: p3, to: p4 } = other;
    let d1 = other.direction(p1);
    let d2 = other.direction(p2);
    let d3 = this.direction(p3);
    let d4 = this.direction(p4);
    if ((Math.sign(d1) * Math.sign(d2) < 0) && (Math.sign(d3) * Math.sign(d4) < 0)) {
      return true;
    } else if (d1 === 0 && other.bounds(p1)) {
      return true;
    } else if (d2 === 0 && other.bounds(p2)) {
      return true;
    } else if (d3 === 0 && this.bounds(p3)) {
      return true;
    } else if (d4 === 0 && this.bounds(p4)) {
      return true;
    } else {
      return false;
    }
  }
}
