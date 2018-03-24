export {
  YoungTableau
};

import { minimumOn, maximumOn } from "../util";

const EMPTY = +Infinity;

class YoungTableau {
  private _matrix: number[][];
  private _m: number;
  private _n: number;

  constructor(m: number, n: number) {
    this._m = m;
    this._n = n;

    let Y: number[][] = [];
    for (let i = 0; i < m; i++) {
      Y[i] = [];
      for (let j = 0; j < n; j++) {
        Y[i][j] = EMPTY;
      }
    }

    this._matrix = Y;
  }

  isEmpty(): boolean {
    return this._matrix[0][0] == EMPTY;
  }

  isFull(): boolean {
    return this._matrix[this._m - 1][this._n - 1] != EMPTY;
  }

  inBound(i: number, j: number): boolean {
    return i >= 0 && i < this._m && j >= 0 && j < this._n;
  }

  swap(i: number, j: number, l: number, k: number) {
    if (!this.inBound(i, j)) {
      throw `Error: out of bound access Y[${i}][${j}]`;
    }
    if (!this.inBound(l, k)) {
      throw `Error: out of bound access Y[${l}][${k}]`;
    }

    let Y = this._matrix;
    let temp = Y[i][j];
    Y[i][j] = Y[l][k];
    Y[l][k] = temp;
  }

  extractMin(): number {
    if (this.isEmpty()) {
      throw "Error: Tableau underflow";
    }

    let Y = this._matrix;
    let min = Y[0][0];
    Y[0][0] = EMPTY;

    let i = 0, j = 0;
    let new_i = 0, new_j = 0;
    do {
      this.swap(i, j, new_i, new_j);
      i = new_i;
      j = new_j;
      let candidates = [[i, j], [i + 1, j], [i, j + 1]]
        .filter(([i, j]) => this.inBound(i, j));
      [new_i, new_j] = minimumOn(candidates, ([i, j]) => Y[i][j]);
    } while (i != new_i || j != new_j);

    return min;
  }

  insert(key: number) {
    if (this.isFull()) {
      throw "Error: Tableau overflow";
    }

    let Y = this._matrix;
    let [m, n] = [this._m, this._n];

    let i = m - 1;
    let j = n - 1;
    Y[i][j] = key;
    let new_i = i;
    let new_j = j;
    do {
      this.swap(i, j, new_i, new_j);
      i = new_i;
      j = new_j;
      let candidates = [[i, j], [i - 1, j], [i, j - 1]]
        .filter(([i, j]) => this.inBound(i, j));
      [new_i, new_j] = maximumOn(candidates, ([i, j]) => Y[i][j]);
    } while (i != new_i || j != new_j);
  }

  find(key: number): [number, number] | null {
    let Y = this._matrix;
    let m = this._m;
    let n = this._n;

    let i = 0, j = n - 1;
    while (this.inBound(i, j) && Y[i][j] !== key) {
      if (Y[i][j] > key) {
        j--;
      } else {
        i++;
      }
    }

    if (this.inBound(i, j)) {
      return [i, j];
    } else {
      return null;
    }
  }

  diagnose() {
    let Y = this._matrix;
    let m = this._m;
    let n = this._n;

    console.log("Self diagnosing...");

    for (let i = 0; i < m; i++) {
      for (let j = 0; j < n; j++) {
        if (this.inBound(i + 1, j) && Y[i + 1][j] < Y[i][j]) {
          console.error(`Error: Y[${i}][${j}] = ${Y[i][j]} is greater than Y[${i + 1}][${j}] = ${Y[i + 1][j]}`);
        }
        if (this.inBound(i, j + 1) && Y[i][j + 1] < Y[i][j]) {
          console.error(`Error: Y[${i}][${j}] = ${Y[i][j]} is greater than Y[${i}][${j + 1}] = ${Y[i][j + 1]}`);
        }
      }
    }

    console.log("End of self diagnosis");
  }
}