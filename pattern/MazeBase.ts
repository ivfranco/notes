export {
  MapSite,
  RoomNo,
  Direction,
  directions,
  Room,
  Wall,
  Door,
  Maze
}

interface ICloneable {
  clone(): ICloneable
}

interface MapSite extends ICloneable {
  enter(): void
  clone(): MapSite
}

type RoomNo = number

const enum Direction { North, West, South, East };
const directions = [
  Direction.North,
  Direction.West,
  Direction.South,
  Direction.East
]

class Room implements MapSite {
  private _roomNo: RoomNo
  private _sides: Map<Direction, MapSite>

  constructor(roomNo: RoomNo) {
    this.initialize(roomNo);
  }

  clone() {
    let newRoom = new Room(this._roomNo);
    for (let [k, v] of this._sides) {
      newRoom.setSide(k, v);
    }
    return newRoom;
  }

  initialize(roomNo: RoomNo) {
    this._roomNo = roomNo;
    this._sides = new Map();
  }

  getSide(d: Direction): MapSite {
    return this._sides.get(d);
  }

  setSide(d: Direction, m: MapSite): void {
    this._sides.set(d, m);
  }

  enter(): void {}
}

class Wall implements MapSite {
  constructor() {}

  clone() {
    return new Wall();
  }

  enter(): void {}
}

class Door implements MapSite {
  private _room1: Room
  private _room2: Room
  private _isOpen: boolean

  constructor(first: Room, second: Room) {
    this.initialize(first, second);
  }

  clone() {
    let newDoor = new Door(this._room1, this._room2);
    newDoor._isOpen = this._isOpen;
    return newDoor;
  }

  initialize(first: Room, second: Room) {
    this._room1 = first;
    this._room2 = second;
    this._isOpen = false;
  }

  otherSideFrom(room: Room): Room {
    if (room === this._room1) return this._room2;
    if (room === this._room2) return this._room1;
    return null;
  }

  enter(): void {}
}

class Maze implements ICloneable {
  private _roomMap: Map<RoomNo, MapSite>

  constructor() {}

  // undefined
  clone() { return this; }

  // undefined
  addRoom(room: Room): void {}
  // undefined
  getRoom(roomNo: RoomNo): Room {
    return null;
  }
}