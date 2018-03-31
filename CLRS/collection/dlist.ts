export {
  DList,
  DNode
};

class DList<T> {
  head: DNode<T> | null;
  tail: DNode<T> | null;

  constructor() {
    this.head = null;
    this.tail = null;
  }

  search(k: T): DNode<T> | null {
    let x = this.head;
    while (x !== null && x.key !== k) {
      x = x.next;
    }
    return x;
  }

  insert(x: T) {
    let node = new DNode(x);
    node.next = this.head;
    if (this.head !== null) {
      this.head.prev = node;
    } else {
      // DList is empty, set the tail
      this.tail = node;
    }
    this.head = node;
  }

  delete(node: DNode<T>) {
    if (node.prev !== null) {
      node.prev.next = node.next;
    } else {
      // node is head
      this.head = node.next;
    }

    if (node.next !== null) {
      node.next.prev = node.prev;
    } else {
      // node is tail
      this.tail = node.prev;
    }
  }

  concat(other: DList<T>) {
    if (this.head === null) {
      // this DList is empty, copy the other DList to this one
      this.head = other.head;
      this.tail = other.tail;
    } else if (this.tail !== null && other.head !== null) {
      this.tail.next = other.head;
      other.head.prev = this.tail;
      this.tail = other.tail;
    }
    // if the other DList is empty, no operation should be performed
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

  static fromArray<T>(A: T[]): DList<T> {
    let dlist: DList<T> = new DList();
    for (let i = A.length - 1; i >= 0; i--) {
      dlist.insert(A[i]);
    }
    return dlist;
  }
}

class DNode<T> {
  key: T;
  prev: DNode<T> | null;
  next: DNode<T> | null;

  constructor(k: T) {
    this.key = k;
    this.prev = null;
    this.next = null;
  }
}