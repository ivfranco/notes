export {
  NAryTree,
  treeVertexCover,
};

class NAryTree<T> {
  public key: T;
  public parent: NAryTree<T> | null;
  public children: NAryTree<T>[];

  constructor(key: T, parent: NAryTree<T> | null, children: NAryTree<T>[]) {
    this.key = key;
    this.parent = parent;
    this.children = children;
  }

  isLeaf(): boolean {
    return this.children.length === 0;
  }

  lastChild(): NAryTree<T> | null {
    if (this.isLeaf()) {
      return null;
    } else {
      return this.children[ this.children.length - 1 ];
    }
  }

  createChild(t: T): NAryTree<T> {
    let child = new NAryTree(t, this, []);
    this.children.push(child);
    return child;
  }
}

function treeVertexCover<T>(T: NAryTree<T>): T[] {
  let trees: NAryTree<T>[] = [ T ];
  let cover: T[] = [];

  while (trees.length > 0) {
    let t = trees.pop() as NAryTree<T>;

    if (!(t.isLeaf() && t.parent === null)) {
      while (!t.isLeaf()) {
        t = t.lastChild() as NAryTree<T>;
      }

      t = t.parent as NAryTree<T>;
      cover.push(t.key);

      for (let child of t.children) {
        for (let grand of child.children) {
          grand.parent = null;
          trees.push(grand);
        }
      }

      let parent = t.parent;
      if (parent !== null) {
        parent.children.pop();
        trees.push(parent);
      }
    }
  }

  return cover;
}