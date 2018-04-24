export {
  VEBTree,
};

import { Unit, unit } from "./proto-veb-tree";

type VEBSet = VEBTree<Unit>;

function lowSqrt(d: number): number {
  return 2 ** (Math.floor(d / 2));
}

function highSqrt(d: number): number {
  return 2 ** (Math.ceil(d / 2));
}

function high(x: number, d: number): number {
  return Math.floor(x / lowSqrt(d));
}

function low(x: number, d: number): number {
  return x % lowSqrt(d);
}

function index(h: number, l: number, d: number): number {
  return lowSqrt(d) * h + l;
}

function factory<V>(d: number): VEBTree<V> {
  //  u = 2^d
  console.assert(d >= 1, "Minimum universe if 2^1");
  if (d === 1) {
    return new VEBBase();
  } else {
    return new VEBNode(d);
  }
}

abstract class VEBTree<V> {
  protected abstract u: number;

  public static factory<V>(u: number): VEBTree<V> {
    let d = Math.ceil(Math.log2(u));
    return factory(d);
  }

  public abstract isEmpty(): boolean;
  public abstract insert(k: number, v: V): void;
  public abstract delete(k: number): void;
  public abstract search(k: number): V | null;
  public abstract minimum(): number | null;
  public abstract maximum(): number | null;
  public abstract successor(k: number): number | null;
  public abstract predecessor(k: number): number | null;
  public abstract diagnose(): void;
}

class VEBNode<V> extends VEBTree<V> {
  protected u: number;
  private d: number;
  private min: number | null;
  private min_value: V | null;
  private max: number | null;
  private summary: VEBSet;
  //  a particular slot is not initialized until the first time a key is inserted into it
  //  should always be accessed through this.get or this.touch
  private cluster: Array<VEBTree<V> | null>;

  constructor(d: number) {
    super();
    this.u = 2 ** d;
    this.d = d;
    this.min = null;
    this.min_value = null;
    this.max = null;
    this.summary = factory(Math.ceil(d / 2)) as VEBSet;
    this.cluster = [];
  }

  private makeChild(): VEBTree<V> {
    return factory(Math.floor(this.d / 2));
  }

  //  will create a new cluster if this.cluster[k] === null
  private touch(k: number): VEBTree<V> {
    console.assert(k >= 0 && k < highSqrt(this.d), "Out of boundary access");

    let child = this.cluster[k];
    if (!child) {
      child = this.makeChild();
      this.cluster[k] = child;
    }
    return child;
  }

  //  will not create a new cluster, return null instead
  private get(k: number): VEBTree<V> | null {
    console.assert(k >= 0 && k < highSqrt(this.d), "Out of boundary access");

    let child = this.cluster[k];
    if (child) {
      return child;
    } else {
      return null;
    }
  }

  public isEmpty(): boolean {
    return this.min === null;
  }

  public insert(k: number, v: V) {
    let d = this.d;

    if (this.min === null) {
      this.min = k;
      this.max = k;
      this.min_value = v;
    } else {
      if (this.min > k) {
        let temp_k = this.min;
        this.min = k;
        k = temp_k;
        let temp_v = this.min_value as V;
        this.min_value = v;
        v = temp_v;
      }
      if (this.touch(high(k, d)).isEmpty()) {
        this.summary.insert(high(k, d), unit);
      }
      //  if the insertion into summary above happened
      //  the insertion below is guarenteed to be an insertion into an empty tree
      this.touch(high(k, d)).insert(low(k, d), v);
    }

    if (!this.max || k > this.max) {
      this.max = k;
    }
  }

  //  update the current min with the minimum among clusters, if any
  //  the old min is deleted, the new min is duplicated
  private updateMin() {
    let d = this.d;

    let first_cluster = this.summary.minimum();
    if (first_cluster !== null) {
      let offset = this.touch(first_cluster).minimum() as number;
      //  searching the minimum value is O(1)
      let min_value = this.touch(first_cluster).search(offset) as V;
      this.min = index(first_cluster, offset, d);
      this.min_value = min_value;
    } else {
      this.min = null;
      this.max = null;
      this.min_value = null;
    }
  }

  //  update the current max with the maximum among clusters or min
  private updateMax() {
    let d = this.d;

    let summary_max = this.summary.maximum();
    if (summary_max === null) {
      this.max = this.min;
    } else {
      let offset = this.touch(summary_max).maximum() as number;
      this.max = index(summary_max, offset, d);
    }
  }

  public delete(k: number) {
    let d = this.d;

    if (this.min === this.max) {
      if (k === this.min) {
        this.min = null;
        this.max = null;
        this.min_value = null;
        return true;
      } else {
        return false;
      }
    } else {
      if (k === this.min) {
        this.updateMin();
        //  k === this.min !== this.max, the tree contains at least one more key alongside min
        k = this.min as number;
      }
      this.touch(high(k, d)).delete(low(k, d));
      if (this.touch(high(k, d)).isEmpty()) {
        this.summary.delete(high(k, d));
        this.cluster[high(k, d)] = null;
      }
      if (k === this.max) {
        this.updateMax();
      }
    }
  }

  public minimum(): number | null {
    return this.min;
  }

  public maximum(): number | null {
    return this.max;
  }

  public search(k: number): V | null {
    let d = this.d;
    if (this.min === k) {
      return this.min_value as V;
    } else {
      let child = this.get(high(k, d));
      if (child) {
        return child.search(low(k, d));
      } else {
        return null;
      }
    }
  }

  public successor(k: number): number | null {
    let d = this.d;

    if (this.min !== null && k < this.min) {
      return this.min;
    } else {
      let child = this.get(high(k, d));
      let max_low = child ? child.maximum() : null;
      if (max_low !== null && low(k, d) < max_low) {
        //  max_low > low(k, d), cluster high(k, d) must contain a successor of k
        let offset = this.touch(high(k, d)).successor(low(k, d)) as number;
        return index(high(k, d), offset, d);
      } else {
        let succ_cluster = this.summary.successor(high(k, d));
        if (succ_cluster === null) {
          return null;
        } else {
          //  succ_cluster is in the summary, must contain at least one key
          let offset = this.touch(succ_cluster).minimum() as number;
          return index(succ_cluster, offset, d);
        }
      }
    }
  }

  public predecessor(k: number): number | null {
    let d = this.d;

    if (this.max !== null && k > this.max) {
      return this.max;
    } else {
      let child = this.get(high(k, d));
      let min_low = child ? child.minimum() : null;
      if (min_low !== null && low(k, d) > min_low) {
        //  min_low < low(k, d), cluster high(k, d) must contain a predecessor of k
        let offset = this.touch(high(k, d)).predecessor(low(k, d)) as number;
        return index(high(k, d), offset, d);
      } else {
        let pred_cluster = this.summary.predecessor(high(k, d));
        if (pred_cluster === null) {
          if (this.min !== null && k > this.min) {
            return this.min;
          } else {
            return null;
          }
        } else {
          //  pred_cluster is in the summary, must contain at least one key
          let offset = this.touch(pred_cluster).maximum() as number;
          return index(pred_cluster, offset, d);
        }
      }
    }
  }

  public diagnose() {
    let d = this.d;
    let summary = this.summary;
    summary.diagnose();
    for (let i = 0; i < highSqrt(d); i++) {
      let c = this.cluster[i];
      if (c) {
        c.diagnose();
      }
      if (c && !c.isEmpty()) {
        console.assert(
          summary.search(i) !== null,
          "Search key i in summary should return non-null if cluster i is filled",
        );
      } else {
        console.assert(
          summary.search(i) === null,
          "Search key i in summary should return null if cluster i is empty",
        );
      }
    }
  }
}

//  copied ProtoVEBBase to here
//  each operation in base case can not take longer than O(1)
//  the implementation will not affect asymptotic running time as long as it's correct
class VEBBase<V> extends VEBTree<V> {
  protected u: number;
  private A: [V | null, V | null];

  constructor() {
    super();
    this.u = 2;
    this.A = [null, null];
  }

  private size(): number {
    let A = this.A;
    let n = 0;

    if (A[0] !== null) {
      n++;
    }
    if (A[1] !== null) {
      n++;
    }

    return n;
  }

  public isEmpty(): boolean {
    return this.size() === 0;
  }

  public insert(k: number, v: V) {
    console.assert(k >= 0 && k <= 1, "Out of boundary access");
    this.A[k] = v;
  }

  public delete(k: number) {
    console.assert(k >= 0 && k <= 1, "Out of boundary access");
    this.A[k] = null;
  }

  public search(k: number): V | null {
    console.assert(k >= 0 && k <= 1);
    return this.A[k];
  }

  public minimum(): number | null {
    let A = this.A;
    if (A[0] !== null) {
      return 0;
    } else if (A[1] !== null) {
      return 1;
    } else {
      return null;
    }
  }

  public maximum(): number | null {
    let A = this.A;
    if (A[1] !== null) {
      return 1;
    } else if (A[0] !== null) {
      return 0;
    } else {
      return null;
    }
  }

  public successor(k: number): number | null {
    let A = this.A;
    if (k === 0 && A[1] !== null) {
      return 1;
    } else {
      return null;
    }
  }

  public predecessor(k: number): number | null {
    let A = this.A;
    if (k === 1 && A[0] !== null) {
      return 0;
    } else {
      return null;
    }
  }

  public diagnose() {
    //  nothing to test
  }
}
