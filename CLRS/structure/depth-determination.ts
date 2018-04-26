class DepthTreeNode<T> {
  public readonly key: T;
  private parent: this;
  private rank: number;
  private d: number;

  constructor(k: T) {
    this.key = k;
    this.parent = this;
    this.rank = 0;
    this.d = 0;
  }

  private pathCompression(): [number, this] {
    if (this.parent === this) {
      return [0, this];
    } else {
      let [depth, root] = this.parent.pathCompression();
      this.d += depth;
      return [this.d, root];
    }
  }

  public findDepth(): number {
    return this.pathCompression()[0];
  }

  public graft(v: this) {
    let r = this;
    let [r_depth, r_root] = r.pathCompression();
    let [v_depth, v_root] = v.pathCompression();
    r_root.d += v_depth + 1;
    if (r_root.rank < v_root.rank) {
      r_root.parent = v_root;
      r_root.d -= v_root.d;
    } else {
      v_root.parent = r_root;
      v_root.d -= r_root.d;
      if (r_root.rank === v_root.rank) {
        r_root.rank++;
      }
    }
  }
}
