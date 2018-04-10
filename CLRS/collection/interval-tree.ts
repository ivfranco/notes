export {
  IntTree,
};

import { ARBNode, ARBTree } from "./augmented-redblack-tree";

interface Interval {
  low: number;
  high: number;
}

//  assuming intervals are closed
function overlap(a: Interval, b: Interval): boolean {
  return !(a.high < b.low || b.high < a.low);
}

class IntTree extends ARBTree<Interval, number, IntNode> {
  public factory(k: Interval) {
    return new IntNode(k);
  }

  protected le(a: Interval, b: Interval): boolean {
    if (a.low === b.low) {
      return a.high <= b.high;
    } else {
      return a.low <= b.low;
    }
  }

  protected eq(a: Interval, b: Interval): boolean {
    return a.low === b.low && a.high === b.high;
  }

  public intervalSearch(i: Interval): IntNode | null {
    let x: IntNode | null = this.root;

    while (x !== null && !overlap(i, x.key)) {
      if (x.left !== null && x.left.max() >= i.low) {
        x = x.left;
      } else {
        x = x.right;
      }
    }

    return x;
  }
}

class IntNode extends ARBNode<Interval, number> {
  public max(): number {
    return this.f;
  }

  public calcAugment(): number {
    let max = this.key.high;
    if (this.left) {
      max = Math.max(this.left.max(), max);
    }
    if (this.right) {
      max = Math.max(this.right.max(), max);
    }

    return max;
  }
}

function minimalOverlappingInterval(i: Interval, x: IntNode | null): IntNode | null {
  let m: IntNode | null = null;

  while (x !== null) {
    if (overlap(x.key, i)) {
      if (m && m.key.low > x.key.low) {
        m = x;
      }
    }

    if (x.left !== null && x.left.max() >= i.low) {
      x = x.left;
    } else {
      x = x.right;
    }
  }

  return m;
}
