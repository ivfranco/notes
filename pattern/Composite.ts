export {
  Equipment,
  FloppyDisk,
  Card,
  Chassis,
  Bus,
  CompositeEquipment,
  Currency,
  Watt
}

import { EquipmentVisitor } from "./Visitor";

type Currency = number;
type Watt = number;

abstract class Equipment implements Iterable<Equipment> {
  private _name: string

  constructor(n: string) {}

  abstract power(): Watt
  abstract netPrice(): Currency
  abstract discountPrice(): Currency

  abstract add(e: Equipment): void
  abstract remove(e: Equipment): void
  abstract [Symbol.iterator](): Iterator<Equipment>

  abstract accept(visitor: EquipmentVisitor): void
}

abstract class FloppyDisk extends Equipment {
  accept(visitor: EquipmentVisitor) {
    visitor.visitFloppyDisk(this);
  }
}

abstract class Card extends Equipment {
  accept(visitor: EquipmentVisitor) {
    visitor.visitCard(this);
  }
}

abstract class Chassis extends Equipment {
  private _parts: Array<Equipment>

  accept(visitor: EquipmentVisitor) {
    for (let e of this) {
      e.accept(visitor);
    }
    visitor.visitChassis(this);
  }

  [Symbol.iterator](): Iterator<Equipment> {
    return this._parts[Symbol.iterator]();
  }
}

abstract class Bus extends Equipment {
  accept(visitor: EquipmentVisitor) {
    visitor.visitBus(this);
  }
} 

abstract class CompositeEquipment extends Equipment {
  private _equipment: Array<Equipment>

  constructor(n: string) {
    super(n);
  }

  netPrice() {
    let total = 0;
    for (let p of this) {
      total += p.netPrice();
    }
    return total;
  }

  [Symbol.iterator](): Iterator<Equipment> {
    return this._equipment[Symbol.iterator]();
  }
}
