export class Huge {
  A: number[];
  S: number[];
  top: number;

  constructor() {
    this.A = [];
    this.S = [];
    this.top = 0;
  }

  search(k: number): boolean {
    let A = this.A;
    let S = this.S;
    let top = this.top;

    return A[k] < top && S[A[k]] === k;
  }

  insert(k: number) {
    let A = this.A;
    let S = this.S;

    if (!this.search(k)) {
      A[k] = this.top;
      S[this.top] = k;
      this.top++;
    }
  }

  delete(k: number) {
    let A = this.A;
    let S = this.S;

    if (this.search(k)) {
      this.top--;
      S[A[k]] = S[this.top];
      A[S[this.top]] = A[k];
    }
  }

  list(): number[] {
    let ret = [];
    for (let i = 0; i < this.top; i++) {
      ret.push(this.S[i]);
    }
    return ret;
  }
}