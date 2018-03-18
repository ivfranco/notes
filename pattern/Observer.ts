export {}

abstract class Subject {
  private _observers: Set<Observer>

  attach(o: Observer) {
    this._observers.add(o);
  }

  detach(o: Observer) {
    this._observers.delete(o);
  }

  notify() {
    this._observers.forEach(o => {
      o.update(this);
    });
  }
}

interface Observer {
  update(changed: Subject): void
}

class ClockTimer extends Subject {
  private _date: Date

  constructor() {
    super();
    this.sync();
  }

  getHour() {
    return this._date.getHours();
  }

  getMinute() {
    return this._date.getMinutes();
  }

  getSecond() {
    return this._date.getSeconds();
  }

  private sync() {
    this._date = new Date();
  }

  tick() {
    this.sync();
    this.notify();
  }
}

interface Widget {
  draw(): void
}

class DigitalClock implements Widget, Observer {
  private _subject: ClockTimer

  constructor(timer: ClockTimer) {}

  update(changed: Subject) {
    if (changed === this._subject) {
      this.draw();
    }
  }

  draw() {
    // ...
  }
}

class AnalogClock implements Widget, Observer {
  private _subject: ClockTimer

  constructor(timer: ClockTimer) {}

  update(changed: Subject) {
    if (changed === this._subject) {
      this.draw();
    }
  }

  draw() {
    // ...
  }
}

const timer = new ClockTimer();
const analogClock = new AnalogClock(timer);
const digitalClock = new DigitalClock(timer);

timer.tick();