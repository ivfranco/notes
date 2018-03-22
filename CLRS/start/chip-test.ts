export {
  Chip,
  GoodChip,
  BadChip,
  testChips
};

interface Testable {
  test(other: Testable): boolean;
}

interface Chip extends Testable {
  isGood(): boolean;
  test(other: Chip): boolean;
}

class GoodChip implements Chip {
  constructor() { }

  isGood(): boolean {
    return true;
  }

  test(other: Chip): boolean {
    return other.isGood();
  }
}

class BadChip implements Chip {
  constructor() { }

  isGood(): boolean {
    return false;
  }

  test(other: Chip): boolean {
    return Math.random() >= 0.5;
  }
}

function findGood(cs: Testable[]): Testable {
  let n = cs.length;

  if (n == 1) {
    return cs[0];
  } else {
    let pick = [];
    let both = 0;
    for (let i = 0; i < (n & ~0x1); i += 2) {
      let a = cs[i];
      let b = cs[i + 1];

      if (a.test(b) == true && b.test(a) == true) {
        pick.push(a);
        both++;
      }
    }

    if (n % 2 == 1 && both % 2 == 0) {
      pick.push(cs[n - 1]);
    }

    return findGood(pick);
  }
}

function testChips<T extends Testable>(cs: T[]): T[] {
  let good = findGood(cs);
  return cs.filter((c) => good.test(c));
}