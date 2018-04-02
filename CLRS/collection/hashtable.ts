import { DList, DNode } from "./dlist";

function dHash(m: number, k: number): number {
  return k % m;
}

function mHash(m: number, k: number): number {
  const A = (Math.sqrt(5) - 1) / 2;
  return Math.floor(m * ((A * k) % 1));
}

abstract class Chaining<T> {
  m: number;
  T: DList<T>[];

  constructor(m: number) {
    this.m = m;
    let T: DList<T>[] = [];

    for (let i = 0; i < m; i++) {
      T[i] = new DList();
    }
    this.T = T;
  }

  abstract mapping(a: T): number;

  hash(a: T): number {
    let m = this.m;
    let key = this.mapping(a);

    return mHash(m, key);
  }

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