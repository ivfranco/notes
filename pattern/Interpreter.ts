interface Context {
  lookup(name: string): boolean
  assign(name: string, value: boolean): void
}

interface BooleanExp {
  evaluate(context: Context): boolean
  replace(name: string, exp: BooleanExp): BooleanExp
  copy(): BooleanExp
}

class Constant implements BooleanExp {
  private _value: boolean

  constructor(value: boolean) {
    this._value = value;
  }

  evaluate(context: Context) {
    return this._value;
  }

  replace(name: string, exp: BooleanExp) {
    return this.copy();
  }

  copy() {
    return new Constant(this._value);
  }
}

class VariableExp implements BooleanExp {
  private _name: string

  constructor(name: string) {
    this._name = name;
  }

  evaluate(context: Context) {
    return context.lookup(this._name);
  }

  replace(name: string, exp: BooleanExp): BooleanExp {
    if (this._name === name) {
      return exp.copy();
    } else {
      return this.copy();
    }
  }

  copy() {
    return new VariableExp(this._name);
  }
}

type BooleanBinOp = (b1: boolean, b2: boolean) => boolean
type BooleanUnOp = (b: boolean) => boolean

class BinaryExp implements BooleanExp {
  private _operand1: BooleanExp
  private _operand2: BooleanExp
  private _operation: BooleanBinOp

  constructor(op1: BooleanExp, op2: BooleanExp, operation: BooleanBinOp) {
    this._operand1 = op1;
    this._operand2 = op2;
    this._operation = operation;
  }

  evaluate(context: Context) {
    return this._operation(
      this._operand1.evaluate(context),
      this._operand2.evaluate(context)
    );
  }

  replace(name: string, exp: BooleanExp) {
    return new BinaryExp(
      this._operand1.replace(name, exp),
      this._operand2.replace(name, exp),
      this._operation
    );
  }

  copy() {
    return new BinaryExp(
      this._operand1.copy(),
      this._operand2.copy(),
      this._operation
    )
  }
}

class UnaryExp implements BooleanExp {
  private _operand: BooleanExp
  private _operation: BooleanUnOp

  constructor(op: BooleanExp, operation: BooleanUnOp) {
    this._operand = op;
    this._operation = operation;
  }

  evaluate(context: Context) {
    return this._operation(this._operand.evaluate(context));
  }

  replace(name: string, exp: BooleanExp) {
    return new UnaryExp(
      this._operand.replace(name, exp),
      this._operation
    );
  }

  copy() {
    return new UnaryExp(
      this._operand.copy(),
      this._operation
    );
  }
}

const and: BooleanBinOp = (x, y) => x && y
const or: BooleanBinOp = (x, y) => x || y
const not: BooleanUnOp = x => !x

const context = {
  _map: <Map<string, boolean>>new Map(),
  lookup(name: string) {
    return this._map.get(name);
  },
  assign(name: string, value: boolean) {
    return this._map.set(name, value);
  }
}

const x = new VariableExp("X");
const y = new VariableExp("Y");

const expression = new BinaryExp(
  new BinaryExp(new Constant(true), x, and),
  new BinaryExp(y, new UnaryExp(x, not), and),
  or
);

context.assign("X", false);
context.assign("Y", true);

let result = expression.evaluate(context);
console.log(result);

const z = new VariableExp("Z");
const replacement = expression.replace("Y", new UnaryExp(z, not));
context.assign("Z", true);

result = replacement.evaluate(context);
console.log(result);