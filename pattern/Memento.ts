export {}

interface Graphic {
  move(delta: Point): void
}
interface Point {
  invert(): Point
}

abstract class MoveCommand {
  private _state: ConstraintSolverMemento
  private _delta: Point
  private _target: Graphic

  constructor(target: Graphic, delta: Point) {}

  execute(): void {
    const solver = ConstraintSolver.Instance();
    this._state = solver.createMemento();
    this._target.move(this._delta);
    solver.solve();
  }

  unexecute(): void {
    const solver = ConstraintSolver.Instance();
    this._target.move(this._delta.invert());
    solver.setMemento(this._state);
    solver.solve();
  }
}

abstract class ConstraintSolver {
  static Instance(): ConstraintSolver { return null; }

  abstract solve(): void
  abstract addContraint(startConnection: Graphic, endConnection: Graphic): void
  abstract removeConstraint(startConnection: Graphic, endConnection: Graphic): void
  abstract createMemento(): ConstraintSolverMemento
  abstract setMemento(memento: ConstraintSolverMemento): void
}

abstract class ConstraintSolverMemento {}