export {
  polarSort,
};

import { Point } from "./line";

function polarSort(origin: Point, points: Point[]) {
  points.sort((p1, p2) => compareAngle(origin, p1, p2));
}

function compareAngle(origin: Point, p1: Point, p2: Point): number {
  p1 = p1.sub(origin);
  p2 = p2.sub(origin);
  let q1 = p1.quadrant();
  let q2 = p2.quadrant();
  if (q1 !== q2) {
    return q1 - q2;
  } else {
    return -p1.crossProduct(p2);
  }
}
