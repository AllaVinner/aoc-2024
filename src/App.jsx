import { useState } from 'react'
//src/App.js
import * as wasm from "../wasm-src/pkg/wasm_src.js";
import AoCApp from "./aoc_components/AoCApp"
import Day01 from "./days/Day01";
import Day02 from "./days/Day02";
import Day00 from "./days/Day00";


function App() {
  const days = [
    { title: " 1", content: <Day01 /> },
    { title: " 2", content: <Day02 /> }
  ];
  return <>
    <AoCApp days={days} />
  </>
}

export default App
