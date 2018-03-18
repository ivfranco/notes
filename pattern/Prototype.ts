import { MazeFactory } from "./AbstractFactory"
import { Maze, Room, Wall, Door } from "./MazeBase"

class MazePrototypeFactory implements MazeFactory {
  private _prototypeMaze: Maze
  private _prototypeWall: Wall
  private _prototypeRoom: Room
  private _prototypeDoor: Door

  constructor(maze: Maze, wall: Wall, room: Room, door: Door) {
    this._prototypeMaze = maze;
    this._prototypeWall = wall;
    this._prototypeRoom = room;
    this._prototypeDoor = door;
  }
}