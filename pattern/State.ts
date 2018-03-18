export {}

interface TCPOctetStream {}

abstract class TCPConnection {
  private _state: TCPState

  constructor() {
    this._state = TCPClosed.Instance();
  }

  activeOpen(): void {
    this._state.activeOpen(this);
  }

  passiveOpen(): void {
    this._state.passiveOpen(this);
  }

  close(): void {
    this._state.close(this);
  }

  send(): void {
    this._state.send(this);
  }

  acknowledge(): void {
    this._state.acknowledge(this);
  }

  synchronize(): void {
    this._state.synchronize(this);
  }

  abstract processOctet(stream: TCPOctetStream): void

  changeState(state: TCPState): void {
    this._state = state;
  }
}

abstract class TCPState {
  transmit(conn: TCPConnection, stream: TCPOctetStream): void {}
  activeOpen(conn: TCPConnection): void {}
  passiveOpen(conn: TCPConnection): void {}
  close(conn: TCPConnection): void {}
  synchronize(conn: TCPConnection): void {}
  acknowledge(conn: TCPConnection): void {}
  send(conn: TCPConnection): void {}
  protected changeState(conn: TCPConnection, state: TCPState): void {
    conn.changeState(state);
  }
}

class TCPEstablished extends TCPState {
  static Instance(): TCPEstablished { return null; }

  close(conn: TCPConnection) {
    this.changeState(conn, TCPListen.Instance());
  }

  transmit(conn: TCPConnection, stream: TCPOctetStream) {
    conn.processOctet(stream);
  }
}

class TCPListen extends TCPState {
  static Instance(): TCPListen { return null; }

  send(conn: TCPConnection) {
    // ...
    this.changeState(conn, TCPEstablished.Instance());
  }
}

class TCPClosed extends TCPState {
  static Instance(): TCPClosed { return null; }

  activeOpen(conn: TCPConnection) {
    // ...
    this.changeState(conn, TCPEstablished.Instance());
  }

  passiveOpen(conn: TCPConnection) {
    this.changeState(conn, TCPListen.Instance());
  }
}