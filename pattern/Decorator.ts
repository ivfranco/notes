interface VisualComponent {
  draw(): void
  resize(): void
}

abstract class TextView implements VisualComponent {
  constructor() {}

  abstract draw: void
  abstract resize: void
}

abstract class Decorator implements VisualComponent {
  protected _component: VisualComponent

  constructor(component: VisualComponent) {}

  draw() {
    this._component.draw();
  }

  resize() {
    this._component.resize();
  }
}

abstract class BorderDecorator extends Decorator {
  private _width: number

  constructor(component: VisualComponent, borderWidth: number) {
    super(component);
  }

  protected abstract drawBorder(width: number): void

  draw() {
    this._component.draw();
    this.drawBorder(this._width);
  }
}