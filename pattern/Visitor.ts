export { EquipmentVisitor }

import { Equipment, FloppyDisk, CompositeEquipment, Card, Chassis, Bus,
         Currency, Watt 
       } from "./Composite";

abstract class EquipmentVisitor {
  abstract visitFloppyDisk(floppy: FloppyDisk): void
  abstract visitCard(card: Card): void
  abstract visitChassis(chassis: Chassis): void
  abstract visitBus(bus: Bus): void
}

class PricingVisitor extends EquipmentVisitor {
  private _total: number

  constructor() {
    super();
    this._total = 0;
  }

  visitFloppyDisk(floppy: FloppyDisk) {
    this._total += floppy.netPrice();
  }

  visitCard(card: Card) {
    this._total += card.netPrice();
  }

  visitChassis(chassis: Chassis) {
    this._total += chassis.discountPrice();
  }

  visitBus(bus: Bus) {
    this._total += bus.netPrice();
  }
}

