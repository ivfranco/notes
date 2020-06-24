import { expect } from "chai";
import { Interval } from "../src/lib";
import { native_comparator } from "../src/comparator";

function contain_test(a: number | null, b: number | null, c: number | null, d: number | null): Chai.Assertion {
  return expect(new Interval(a, b).contain(new Interval(c, d), native_comparator), `[${c}, ${d}) ⊆ [${a}, ${b})`);
}

function overlap_test(a: number | null, b: number | null, c: number | null, d: number | null): Chai.Assertion {
  return expect(new Interval(a, b).overlap(new Interval(c, d), native_comparator), `[${a}, ${b}) ∩ [${c}, ${d}) != ∅`);
}

describe("Interval operations", function () {
  it("contain operations", function () {
    contain_test(1, 3, 2, 3).true;
    contain_test(1, 3, 2, 4).false;
    contain_test(null, null, null, 4).true;
    contain_test(null, 4, null, 5).false;
  });

  it("overlap operations", function () {
    overlap_test(1, 3, 2, 3).true;
    overlap_test(1, 3, 2, 4).true;
    overlap_test(4, 10, 5, 6).true;
    overlap_test(1, 2, 3, 4).false;
    overlap_test(null, 3, 4, null).false;
  });
});
