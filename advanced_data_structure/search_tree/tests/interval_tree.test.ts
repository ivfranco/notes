import { IntervalTree, ClosedInterval } from "../src/interval_tree";
import { native_comparator } from "../src/comparator";
import { expect } from "chai";
import { random_int } from "./test_utils";

describe("Interval tree query", function () {
  it("construct a correct interval tree", function () {
    for (let i = 0; i < 10; i++) {
      let intervals = [];
      for (let j = 0; j < 10; j++) {
        intervals.push(new ClosedInterval(j, j + 5));
      }

      let search_key = random_int(0, 10);
      let found = intervals.filter((i) => i.close_close(search_key, native_comparator));

      let tree = new IntervalTree<number>(native_comparator, intervals);

      let tree_found = tree.find_intervals(search_key);
      expect(tree_found, "should find all intervals containing the key").includes.members(found);
      expect(tree_found.length, "should find only intervals containing the key").equals(found.length);
    }
  });
});
