interface Point {}
interface IStream {}
interface OStream {}
interface Event {}

abstract class Graphic {
  protected constructor() {}

  abstract draw(at: Point): void
  abstract handleMouse(event: Event): void
  abstract getExtent(): Point
  abstract load(from: IStream): void
  abstract save(to: OStream): void
}

abstract class Image extends Graphic {
  constructor(file: string) {
    super();
  }
}

abstract class ImageProxy extends Graphic {
  private _image: Image
  private _extent: Point
  private _fileName: string

  constructor(imageFile: string) {
    super();
    this._fileName = imageFile;
    this._extent = Point.Zero;
    this._image = null;
  }

  protected getImage(): Image {
    if (this._image === null) {
      this._image = new Image(this._fileName);
    }
    return this._image;
  }

  getExtent() {
    if (this._extent === Point.Zero) {
      this._extent = this.getImage().getExtent();
    }
    return this._extent;
  }

  draw(at: Point) {
    this.getImage().draw(at);
  }

  handleMouse(event: Event) {
    this.getImage().handleMouse(event);
  }
}