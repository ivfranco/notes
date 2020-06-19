export { Ordering, Comparator, WithBottom, cmp_with_bottom, native_comparator, array_comparator, min, max, sorted };

enum Ordering {
  EQ,
  LT,
  GT,
}

type Comparator<T> = (lhs: T, rhs: T) => Ordering;

function native_comparator<T extends number | string>(lhs: T, rhs: T): Ordering {
  if (lhs === rhs) {
    return Ordering.EQ;
  } else if (lhs < rhs) {
    return Ordering.LT;
  } else {
    return Ordering.GT;
  }
}

function sorted<T>(arr: Array<T>, cmp: Comparator<T>): boolean {
  for (let i = 0; i < arr.length - 1; i += 1) {
    if (cmp(arr[i], arr[i + 1]) == Ordering.GT) {
      return false;
    }
  }

  return true;
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

const Bottom = Symbol("Bottom");

type WithBottom<T> = T | typeof Bottom;

function cmp_with_bottom<T>(cmp: Comparator<T>): Comparator<WithBottom<T>> {
  return function (lhs, rhs) {
    if (lhs == Bottom && rhs == Bottom) {
      return Ordering.EQ;
    } else if (lhs == Bottom) {
      return Ordering.LT;
    } else if (rhs == Bottom) {
      return Ordering.GT;
    } else {
      return cmp(lhs, rhs);
    }
  };
}
