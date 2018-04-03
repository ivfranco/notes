export {
  LinearProbing,
  QuadraticProbing,
  DoubleHashing
};

import { DList, DNode } from "./dlist";

abstract class Chaining<T> {
  protected m: number;
  protected T: DList<T>[];

  constructor(m: number) {
    this.m = m;
    let T: DList<T>[] = [];

    for (let i = 0; i < m; i++) {
      T[i] = new DList();
    }
    this.T = T;
  }

  abstract hash(a: T): number;

  search(a: T): DNode<T> | null {
    let T = this.T;
    let h = this.hash(a);

    return T[h].search(a);
  }

  insert(a: T) {
    let T = this.T;
    let h = this.hash(a);

    T[h].insert(a);
  }

  delete(x: DNode<T>) {
    let T = this.T;
    let h = this.hash(x.key);

    T[h].delete(x);
  }
}

const DELETED = Symbol("DELETED");

abstract class OpenAddressing<T> {
  protected m: number;
  protected T: (T | null | typeof DELETED)[];

  constructor(m: number) {
    this.m = m;
    this.T = new Array(m);
    this.T.fill(null);
  }

  abstract hash(k: T, i: number): number;

  search(a: T): number | null {
    let T = this.T;
    let m = this.m;

    let i = 0;
    while (i < m) {
      let h = this.hash(a, i);
      if (T[h] === a) {
        return h;
      }
      if (T[h] === null) {
        return null;
      }
      i++;
    }
    return null;
  }

  insert(a: T): number {
    let T = this.T;
    let m = this.m;

    let i = 0;
    while (i < m) {
      let h = this.hash(a, i);
      if (T[h] === a) {
        return h;
      }
      if (T[h] === DELETED || T[h] === null) {
        T[h] = a;
        return h;
      }
      i++;
    }

    throw "Error: Hashtable overflow";
  }

  delete(i: number) {
    let T = this.T;
    let m = this.m;

    if (i < 0 || i >= m) {
      throw "Error: Out of bound access";
    }

    T[i] = DELETED;
  }

  report() {
    console.log(this.T);
  }
}

class LinearProbing extends OpenAddressing<number> {
  hash(k: number, i: number): number {
    return (k + i) % this.m;
  }
}

class QuadraticProbing extends OpenAddressing<number> {
  hash(k: number, i: number): number {
    return (k + i + 3 * (i ** 2)) % this.m;
  }
}

class DoubleHashing extends OpenAddressing<number> {
  h1(k: number): number {
    return k;
  }
  h2(k: number): number {
    return 1 + (k % (this.m - 1));
  }

  hash(k: number, i: number): number {
    return (this.h1(k) + i * this.h2(k)) % this.m;
  }
}