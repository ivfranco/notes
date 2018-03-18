export {}

type Topic = number;
const NO_HELP_TOPIC: Topic = -1;
const PRINT_TOPIC: Topic = 1;
const PAPER_ORIENTATION_TOPIC: Topic = 2;
const APPLICATION_TOPIC: Topic = 3;

class HelpHander {
  protected _successor: HelpHander
  protected _topic: Topic

  constructor(successor: HelpHander = null, topic: Topic = NO_HELP_TOPIC) {}

  hasHelp(): boolean {
    return this._topic !== NO_HELP_TOPIC;
  }

  handleHelp() {
    if (this._successor !== null) {
      this._successor.handleHelp();
    }
  }

  setHendler(handler: HelpHander, topic: Topic): void {}
}

class Widget extends HelpHander {
  private _parent: Widget

  constructor(parent: Widget, topic: Topic = NO_HELP_TOPIC) {
    super(parent, topic);
    this._parent = parent;
  }
}

class Button extends Widget {
  constructor(parent: Widget, topic: Topic = NO_HELP_TOPIC) {
    super(parent, topic);
  }

  handleHelp() {
    if (this.hasHelp()) {
      // ...
    } else {
      super.handleHelp();
    }
  }
}

class Dialog extends Widget {
  constructor(parent: HelpHander, topic: Topic = NO_HELP_TOPIC) {
    super(null);
    this.setHendler(parent, topic);
  }

  handleHelp() {
    if (this.hasHelp()) {
      // ...
    } else {
      super.handleHelp();
    }
  }
}

class Application extends HelpHander {
  constructor(topic: Topic) {
    super(null, topic);
  }
}

let application = new Application(APPLICATION_TOPIC);
let dialog = new Dialog(application, PRINT_TOPIC);
let button = new Button(dialog, PAPER_ORIENTATION_TOPIC);

button.handleHelp();