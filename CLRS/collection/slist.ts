export {
  SList,
  SNode
};

class SList<T> {
  head: SNode<T> | null;

  constructor() {
    this.head = null;
  }

  insert(x: T) {
    let node = new SNode(x);
    node.next = this.head;
    this.head = node;
  }

  delete(node: SNode<T>) {
    let prev = this.head;
    while (prev !== null && prev.next !== node) {
      prev = prev.next;
    }
    if (prev !== null && prev.next !== null) {
      prev.next = prev.next.next;
    }
  }

  search(k: T): SNode<T> | null {
    let x = this.head;
    while (x !== null && x.key !== k) {
      x = x.next;
    }
    return x;
  }

  reverse() {
    let x = this.head;
    if (x !== null) {
      let next = x.next;
      x.next = null;
      x = next;
    }
    while (x !== null) {
      let next = x.next;
      x.next = this.head;
      this.head = x;
      x = next;
    }
  }

  show(): string {
    let s = "List: ";
    let x = this.head;
    while (x !== null) {
      s += `${x.key} --> `;
      x = x.next;
    }
    s += "NIL";
    return s;
  }
}

class SNode<T> {
  key: T;
  next: SNode<T> | null;

  constructor(k: T) {
    this.key = k;
    this.next = null;
  }
}