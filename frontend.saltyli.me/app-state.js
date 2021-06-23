let _instance = null;

export default class AppState {

  static getInstance() {
    console.log("hi");
    if (_instance) {
      return _instance;
    } else {
      return new AppState();
    }
  }

  constructor() {
    this._core = {};
    window.Controller = this; //for debugging purposes
  }
  getCore() {
    return this._core;
  }
  setCore(prop, value) {
    this._core[prop] = value;
  }
  }
  