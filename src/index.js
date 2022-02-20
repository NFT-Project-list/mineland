import React from "react";
import ReactDOM from "react-dom";
import { App } from "./app";
import { Buffer } from "buffer";
import "./assets/styles/index.css";

// it prevents react to raise an error with Buffer size
// when we try to use window object
window.Buffer = Buffer;
const root = document.getElementById("root");
ReactDOM.render(<App />, root);
