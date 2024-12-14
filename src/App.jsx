import { useState } from 'react'
//src/App.js
import * as wasm from "../wasm-src/pkg/wasm_src.js";
import AoCApp from "./aoc_components/AoCApp"
import Day01 from "./days/Day01";
import Day02 from "./days/Day02";
import Day03 from "./days/Day03";
import Day04 from "./days/Day04";
import Day05 from "./days/Day05";
import Day06 from "./days/Day06";
import Day07 from "./days/Day07";
import Day08 from "./days/Day08";
import Day00 from "./days/Day00";


function App() {
  const days = [
    { title: " 1", content: <Day01 /> },
    { title: " 2", content: <Day02 /> },
    { title: " 3", content: <Day03 /> },
    { title: " 4", content: <Day04 /> },
    { title: " 5", content: <Day05 /> },
    { title: " 6", content: <Day06 /> },
    { title: " 7", content: <Day07 /> },
    { title: " 8", content: <Day08 /> }
  ];
  return <>
    <AoCApp days={days} />
  </>
}

export default App
