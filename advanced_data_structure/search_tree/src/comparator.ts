export { Ordering, Comparator, native_comparator, array_comparator, min, max };

enum Ordering {
  EQ,
  LT,
  GT,
}

type Comparator<T> = (lhs: T, rhs: T) => Ordering;

function native_comparator<T>(lhs: T, rhs: T): Ordering {
  if (lhs === rhs) {
    return Ordering.EQ;
  } else if (lhs < rhs) {
    return Ordering.LT;
  } else {
    return Ordering.GT;
  }
}

// an alphabetical comparator for arrays
function array_comparator<T>(cmp: Comparator<T>): Comparator<T[]> {
  return function (lhs: T[], rhs: T[]): Ordering {
    let len = Math.min(lhs.length, rhs.length);

    for (let i = 0; i < len; i++) {
      let ord = cmp(lhs[i], rhs[i]);
      if (ord != Ordering.EQ) {
        return ord;
      }
    }

    return native_comparator(lhs.length, rhs.length);
  };
}

function min<T>(lhs: T, rhs: T, cmp: Comparator<T>): T {
  if (cmp(lhs, rhs) == Ordering.LT) {
    return lhs;
  } else {
    return rhs;
  }
}

function max<T>(lhs: T, rhs: T, cmp: Comparator<T>): T {
  if (cmp(lhs, rhs) == Ordering.LT) {
    return rhs;
  } else {
    return lhs;
  }
}
