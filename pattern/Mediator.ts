export {}

interface MouseEvent {}

abstract class DialogDirector {
  constructor() {}

  abstract showDialog(): void
  abstract widgetChanged(w: Widget): void
  abstract createWidgets(): void
}

abstract class Widget {
  protected _director: DialogDirector

  constructor(director: DialogDirector) {}

  changed(): void {
    this._director.widgetChanged(this);
  }

  abstract handleMouse(event: MouseEvent): void
}

abstract class ListBox extends Widget {
  abstract getSelection(): string
  abstract setList(listItems: Array<string>): void
}

abstract class EntryField extends Widget {
  abstract setText(text: string): void
  abstract getText(): string
}

abstract class Button extends Widget {
  abstract setText(text: string): void

  handleMouse(event: MouseEvent) {
    // ...
    this.changed();
  }
}

abstract class FontDialogDirector extends DialogDirector {
  private _ok: Button
  private _cancel: Button
  private _fontList: ListBox
  private _fontName: EntryField

  createWidgets() {
    this._ok = new Button(this);
    this._cancel = new Button(this);
    this._fontList = new ListBox(this);
    this._fontName = new EntryField(this);
    // ...
  }

  widgetChanged(wChanged: Widget) {
    if (wChanged === this._fontList) {
      this._fontName.setText(this._fontList.getSelection());
    } else if (wChanged === this._ok) {
      // ...
    } else if (wChanged === this._cancel) {
      // ...
    }
  }
}