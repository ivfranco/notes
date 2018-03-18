import { MazeFactory } from "./AbstractFactory"

class SingletonFactory {
  private static _instance: MazeFactory = null

  static Instance(): MazeFactory {
    if (this._instance == null) {
      this._instance = new MazeFactory();
    }
    return this._instance;
  }

  protected constructor() {}
}