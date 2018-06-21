export {
  Polygon,
};

import { direction, Direction, Point, Segment } from "./line";

//  a polygon represented by a set of vertices in clockwise order
class Polygon {
  public readonly vertices: Point[];

  constructor(vertices: Point[]) {
    console.assert(vertices.length >= 3, "a polygon must have at least three vertices");

    this.vertices = vertices;
  }

  public size(): number {
    return this.vertices.length;
  }

  public isConvex(): boolean {
    let n = this.size();
    let vertices = this.vertices;

    for (let i = 0, d = 0; i < n; i++) {
      let p0 = vertices[i % n];
      let p1 = vertices[i + 1 % n];
      let p2 = vertices[i + 2 % n];
      let sign = Math.sign(direction(p0, p1, p2));
      if (i !== 0 && d !== sign) {
        return false;
      } else {
        d = sign;
      }
    }

    return true;
  }

  public interior(p: Point): boolean {
    let vertices = this.vertices;
    let xm = vertices.map(v => v.x).reduce((x1, x2) => Math.max(x1, x2));
    let ray = new Segment(p, new Point(xm, p.y));
    let n = this.size();
    let cnt = 0;
    for (let i = 0; i < n; i++) {
      let p0 = vertices[i % n];
      let p1 = vertices[i + 1 % n];
      let side = new Segment(p0, p1);

      if (side.direction(p) === 0) {
        //  the ray is colinear with a side, not treated as an intersection
        if (side.bounds(p)) {
          //  p is on the boundary of the polygon
          return false;
        }
      } else if (onRightRay(p, p0)) {
        //  the ray intersects the boundary at a vertex
        //  only p1 is treated in the current iteration
        //  intersection with p0 is treated in the latest iteration
      } else if (onRightRay(p, p1)) {
        //  if the ray intersects on the vertex p1
        //  the ray crosses the boundary iff pp1p0 and pp1p2 take different turns
        let p2 = vertices[i + 2 % n];
        let d1 = direction(p, p1, p0);
        let d2 = direction(p, p1, p2);
        if (d1 !== d2) {
          cnt++;
        }
      } else if (ray.intersects(side)) {
        cnt++;
      }
    }

    return cnt % 2 !== 0;
  }

  public area(): number {
    let reversed = this.vertices.slice();
    reversed.reverse();

    let area = 0;
    while (reversed.length >= 3) {
      let p0 = reversed.pop() as Point;
      let p1 = reversed.pop() as Point;
      let p2 = reversed.pop() as Point;

      let side1 = new Segment(p1, p0);
      let side2 = new Segment(p1, p2);
      let triangle = Math.abs(side1.crossProduct(side2)) / 2;
      switch (direction(p0, p1, p2)) {
        case Direction.COLINEAR:
          break;
        case Direction.LEFT:
          area -= triangle;
          break;
        case Direction.RIGHT:
          area += triangle;
          break;
      }

      reversed.push(p2, p0);
    }

    return area;
  }
}

function onRightRay(p0: Point, p1: Point): boolean {
  return p0.y === p1.y && p0.x <= p1.x;
}
