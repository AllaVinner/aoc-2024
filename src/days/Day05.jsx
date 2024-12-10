import { useEffect, useState } from 'react'
import '../styles/AoC.css'
import "../assets/prism.css"
import Prism from "../assets/prism.js"
import DaySolver from '../aoc_components/DaySolver.jsx';


function Day05() {
  const puzzle_link = "https://adventofcode.com/2024/day/5"
  const code_link = "https://github.com/AllaVinner/aoc-2024/blob/main/wasm-src/src/days/day_05.rs"
  useEffect(() => {
    Prism.highlightAll();
  }, []);

  return (
    <>
      <div id={'day2'}>
        <h1>
          Day 05: Print Queue
        </h1>
        <div>----------------------------------------------------</div>
        <DaySolver
          day={5}
          aoc_puzzle_link={puzzle_link}
          aoc_code_link={code_link}
        />
        <h2>Part 1: Start Describing the solution</h2>
      </div >
    </>
  )
}

export default Day05
