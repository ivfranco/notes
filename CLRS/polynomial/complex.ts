export {
  Complex,
  unityRoot,
};

class Complex {
  public readonly real: number;
  public readonly img: number;

  constructor(real: number, img: number) {
    this.real = real;
    this.img = img;
  }

  public mul(b: Complex | number): Complex {
    return complexMul(this, b);
  }

  public div(b: Complex | number): Complex {
    return complexDiv(this, b);
  }

  public add(b: Complex | number): Complex {
    return complexAdd(this, b);
  }

  public sub(b: Complex | number): Complex {
    return complexSub(this, b);
  }

  public neg(): Complex {
    return complexNeg(this);
  }

  public inverse(): Complex {
    return complexInverse(this);
  }

  public pow(n: number): Complex {
    return complexPow(this, n);
  }

  public norm(): number {
    return complexNorm(this);
  }

  public show(): string {
    let { real, img } = this;
    if (img === 0) {
      return "" + real;
    } else if (img > 0) {
      return `${real} + ${img}i`;
    } else {
      return `${real} - ${Math.abs(img)}i`;
    }
  }
}

function unityRoot(n: number): Complex {
  let u = 2 * Math.PI / n;
  return new Complex(Math.cos(u), Math.sin(u));
}

function complexNeg(a: Complex): Complex {
  return new Complex(-a.real, -a.img);
}

function complexMul(a: Complex, b: Complex | number): Complex {
  let { real: ra, img: ia } = a;
  if (b instanceof Complex) {
    let { real: rb, img: ib } = b;
    return new Complex(ra * rb - ia * ib, ra * ib + rb * ia);
  } else {
    return new Complex(ra * b, ia * b);
  }
}

function complexAdd(a: Complex, b: Complex | number): Complex {
  let { real: ra, img: ia } = a;
  if (b instanceof Complex) {
    let { real: rb, img: ib } = b;
    return new Complex(ra + rb, ia + ib);
  } else {
    return new Complex(ra + b, ia);
  }
}

function complexSub(a: Complex, b: Complex | number): Complex {
  if (b instanceof Complex) {
    return complexAdd(a, b.neg());
  } else {
    return complexAdd(a, -b);
  }
}

function complexInverse(a: Complex): Complex {
  let { real, img } = a;
  let sqSum = real ** 2 + img ** 2;
  return new Complex(real / sqSum, -img / sqSum);
}

function complexPow(a: Complex, n: number): Complex {
  if (n === 0) {
    return new Complex(1, 0);
  } else {
    let half = Math.floor(n / 2);
    let pow = complexPow(a, half);
    pow = complexMul(pow, pow);
    if (n % 2 === 1) {
      pow = complexMul(pow, a);
    }
    return pow;
  }
}

function complexNorm(a: Complex): number {
  let { real, img } = a;
  return Math.hypot(real, img);
}

function complexDiv(a: Complex, b: Complex | number) {
  if (b instanceof Complex) {
    return complexMul(a, complexInverse(b));
  } else {
    let { real, img } = a;
    return new Complex(real / b, img / b);
  }
}
