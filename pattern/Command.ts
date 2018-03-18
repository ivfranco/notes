export {}

interface Application {
  add(doc: Document): void
}

interface Document {
  paste(): void
}

abstract class Command {
  constructor() {}

  abstract execute(): void
}

class OpenCommand extends Command {
  private _application: Application
  private _response: string

  constructor(app: Application) {
    super();
    this._application = app;
  }

  // placeholder
  askUser(): string {
    return "";
  }

  execute() {
    const name = this.askUser();
    if (name) {
      const document = new Document(name);
      this._application.add(document);
      document.open();
    }
  }
}

class PasteCommand extends Command {
  private _document: Document

  constructor(doc: Document) {
    super();
    this._document = doc;
  }

  execute() {
    this._document.paste();
  }
}

class SimpleCommand<Receiver> extends Command {
  private _receiver: Receiver
  private _action: (r: Receiver) => void

  constructor(receiver: Receiver, action: (r: Receiver) => void) {
    super();
    this._receiver = receiver;
    this._action = action;
  }

  execute() {
    this._action(this._receiver);
  }
}

class MacroCommand extends Command {
  private _cmds: Command[]

  constructor() {
    super();
  }

  add(cmd: Command) {
    this._cmds.push(cmd);
  }

  remove(cmd: Command) {
    let idx = this._cmds.indexOf(cmd);
    if (idx !== -1) {
      this._cmds.splice(idx, 1);
    }
  }

  execute() {
    this._cmds.forEach(cmd => cmd.execute());
  }
}