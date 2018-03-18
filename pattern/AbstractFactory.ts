export {
  MazeFactory
}

import { Maze, Room, RoomNo, Wall, Door } from "./MazeBase"

class MazeFactory {
  constructor() {}
  makeMaze(): Maze {
    return new Maze();
  }
  makeWall(): Wall {
    return new Wall();
  }
  makeRoom(roomNo: RoomNo): Room {
    return new Room(roomNo);
  }
  makeDoor(r1: Room, r2: Room): Door {
    return new Door(r1, r2);
  }
}