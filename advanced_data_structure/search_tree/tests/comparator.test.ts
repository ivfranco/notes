import { dedup_sorted, native_comparator } from "../src/comparator";
import { expect } from "chai";
import { random_int } from "./test_utils";

describe("dedup_sorted", function () {
  it("should dedup a sorted array", function () {
    for (let i = 0; i < 10; i++) {
      let numbers = [];
      let SIZE = random_int(0, 100);
      for (let i = 0; i < SIZE; i++) {
        numbers.push(random_int(0, 20));
      }

      let set = new Set(numbers);
      numbers.sort((a, b) => a - b);
      dedup_sorted(native_comparator, numbers);

      expect(numbers.length).equals(set.size);
    }
  });
});
