export {}

interface List<T> {
  count(): number
  get(idx: number): T
  createIterator(): MyIterator<T>
}

interface MyIterator<T> {
  first(): void
  next(): void
  isDone(): boolean
  currentItem(): T
}

class IteratorOutOfBounds implements Error {
  name: string = "IteratorOutOfBounds"
  message: string

  constructor(message: string) {
    this.message = message;
  }

  toString() {
    return `${this.name}: ${this.message}`;
  }
}

class ListIterator<T> implements MyIterator<T> {
  private _list: List<T>
  private _current: number

  constructor(list: List<T>) {
    this._list = list;
    this._current = 0;
  }

  first() {
    this._current = 0;
  }

  next(): void {
    this._current++;
  }

  isDone() {
    return this._current >= this._list.count();
  }

  currentItem() {
    if (this.isDone) {
      throw new IteratorOutOfBounds("Beyond the last item");
    } else {
      return this._list.get(this._current);
    }
  }
}

class ListTraverser<T> {
  private _iterator: ListIterator<T>

  constructor(list: List<T>) {
    this._iterator = new ListIterator(list);
  }

  traverse(op: (t: T) => boolean): boolean {
    let advance = false;
    let iter = this._iterator;

    for (iter.first(); !iter.isDone(); iter.next()) {
      advance = op(iter.currentItem());
      if (advance === false) break;
    }

    return advance;
  }
}