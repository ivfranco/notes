import { RoomNo, Maze, Direction, Room, Door, Wall, directions } from "./MazeBase"

abstract class MazeBuilder {
  constructor() {}

  abstract buildMaze(): void
  abstract getMaze(): Maze
  abstract buildRoom(room: RoomNo): void
  abstract buildDoor(room1: RoomNo, room2: RoomNo): void
}

class StandardMazeBuilder extends MazeBuilder {
  private _currentMaze: Maze
  
  constructor() {
    super();
    this._currentMaze = null;
  }
  
  // undefined
  commonWall(room1: Room, room2: Room): Direction {
    return null;
  }

  buildMaze() {
    this._currentMaze = new Maze();
  }

  getMaze() {
    return this._currentMaze;
  }

  buildRoom(n: number) {
    if (!this._currentMaze.getRoom(n)) {
      let room = new Room(n);
      this._currentMaze.addRoom(room);
      directions.forEach(d => room.setSide(d, new Wall()));
    }
  }

  buildDoor(n1: RoomNo, n2: RoomNo) {
    let room1 = this._currentMaze.getRoom(n1);
    let room2 = this._currentMaze.getRoom(n2);
    let door = new Door(room1, room2);

    room1.setSide(this.commonWall(room1, room2), door);
    room2.setSide(this.commonWall(room2, room1), door);
  }
}