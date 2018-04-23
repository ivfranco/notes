export {
  ProtoVEBTree,
};

const unit = Symbol("Unit");
type Unit = typeof unit;
type ProtoVEBSet = ProtoVEBTree<Unit>;

abstract class ProtoVEBTree<V> {
  protected abstract u: number;
  protected abstract n: number;

  public static factory<V>(u: number): ProtoVEBTree<V> {
    let d = Math.ceil(Math.log2(Math.log2(u)));
    return factory(d);
  }

  public size(): number {
    return this.n;
  }

  public isEmpty(): boolean {
    return this.n === 0;
  }

  //  return true if the key doesn't exist before insertion; return false if an existing value is overwritten
  public abstract insert(k: number, v: V): boolean;
  //  return true if an existing key is deleted; return false if k is not in this tree
  public abstract delete(k: number): boolean;
  public abstract search(k: number): V | null;
  public abstract minimum(): number | null;
  public abstract maximum(): number | null;
  public abstract successor(k: number): number | null;
  public abstract predecessor(k: number): number | null;

  public abstract diagnose(): void;
}

function factory<V>(d: number): ProtoVEBTree<V> {
  console.assert(d >= 0);
  if (d === 0) {
    return new ProtoVEBBase();
  } else {
    return new ProtoVEBNode(d);
  }
}

function high(x: number, d: number): number {
  let sqrt = 2 ** (2 ** (d - 1));
  return Math.floor(x / sqrt);
}

function low(x: number, d: number): number {
  let sqrt = 2 ** (2 ** (d - 1));
  return Math.floor(x % sqrt);
}

function index(h: number, l: number, k: number): number {
  let sqrt = 2 ** (2 ** (k - 1));
  return h * sqrt + l;
}

class ProtoVEBNode<V> extends ProtoVEBTree<V> {
  protected u: number;
  protected n: number;
  private d: number;
  private summary: ProtoVEBSet;
  //  a particular slot is not initialized until the first time it's accessed
  //  should always be accessed through this.get
  private _cluster: Array<ProtoVEBTree<V>>;

  constructor(d: number) {
    super();
    this.u = 2 ** (2 ** d);
    this.n = 0;
    this.d = d;
    this.summary = factory(d - 1) as ProtoVEBSet;
    this._cluster = [];
  }

  private makeChild(): ProtoVEBTree<V> {
    return factory(this.d - 1);
  }

  private get(i: number): ProtoVEBTree<V> {
    let cluster = this._cluster;
    let child = cluster[i];
    if (child) {
      return child;
    } else {
      child = this.makeChild();
      this._cluster[i] = child;
      return child;
    }
  }

  public insert(k: number, v: V): boolean {
    let d = this.d;
    this.summary.insert(high(k, d), unit);
    let inserted = this.get(high(k, d)).insert(low(k, d), v);
    if (inserted) {
      this.n++;
    }
    return inserted;
  }

  public delete(k: number): boolean {
    let d = this.d;
    let child = this.get(high(k, d));
    let deleted = child.delete(low(k, d));
    if (deleted) {
      this.n--;
      if (child.isEmpty()) {
        this.summary.delete(high(k, d));
      }
    }
    return deleted;
  }

  public search(k: number): V | null {
    let d = this.d;
    return this.get(high(k, d)).search(low(k, d));
  }

  public minimum(): number | null {
    let min_cluster = this.summary.minimum();
    if (min_cluster === null) {
      return null;
    } else {
      let d = this.d;
      let offset = this.get(min_cluster).minimum() as number;
      return index(min_cluster, offset, d);
    }
  }

  public maximum(): number | null {
    let max_cluster = this.summary.maximum();
    if (max_cluster === null) {
      return null;
    } else {
      let d = this.d;
      let offset = this.get(max_cluster).maximum() as number;
      return index(max_cluster, offset, d);
    }
  }

  public successor(k: number): number | null {
    let d = this.d;
    let offset = this.get(high(k, d)).successor(low(k, d));
    if (offset !== null) {
      return index(high(k, d), offset, d);
    } else {
      let succ_cluster = this.summary.successor(high(k, d));
      if (succ_cluster === null) {
        return null;
      } else {
        offset = this.get(succ_cluster).minimum() as number;
        return index(succ_cluster, offset, d);
      }
    }
  }

  public predecessor(k: number): number | null {
    let d = this.d;
    let offset = this.get(high(k, d)).predecessor(low(k, d));
    if (offset !== null) {
      return index(high(k, d), offset, d);
    } else {
      let pred_cluster = this.summary.predecessor(high(k, d));
      if (pred_cluster === null) {
        return null;
      } else {
        offset = this.get(pred_cluster).maximum() as number;
        return index(pred_cluster, offset, d);
      }
    }
  }

  public diagnose() {
    let n = 0;
    let sqrt = 2 ** (2 ** (this.d - 1));
    let summary = this.summary;

    summary.diagnose();
    for (let i = 0; i < sqrt; i++) {
      let c = this._cluster[i];
      if (c) {
        c.diagnose();
      }
      if (c && c.size() > 0) {
        console.assert(
          summary.search(i) !== null,
          "Search key i in summary should return non-null if cluster i is filled",
        );
        n += c.size();
      } else {
        console.assert(
          summary.search(i) === null,
          "Search key i in summary should return null if cluster i is empty",
        );
      }
    }

    console.assert(this.n === n, "n should match the number of keys stored in the vEB-tree");
  }
}

class ProtoVEBBase<V> extends ProtoVEBTree<V> {
  protected u: number;
  protected n: number;
  private A: Array<V | null>;

  constructor() {
    super();
    this.u = 2;
    this.n = 0;
    this.A = [null, null];
  }

  public insert(k: number, v: V): boolean {
    let A = this.A;
    if (A[k] === null) {
      A[k] = v;
      this.n++;
      return true;
    } else {
      A[k] = v;
      return false;
    }
  }

  public delete(k: number): boolean {
    let A = this.A;
    if (A[k] !== null) {
      this.n--;
      A[k] = null;
      return true;
    } else {
      return false;
    }
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
    let A = this.A;
    let n = 0;
    if (A[0] !== null) {
      n++;
    }
    if (A[1] !== null) {
      n++;
    }

    console.assert(this.n === n, "n should match the number of keys stored in the vEB-tree");
  }
}
