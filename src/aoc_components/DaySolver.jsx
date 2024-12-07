import { useEffect, useState } from 'react'
import '../styles/AoC.css'
import "../assets/prism.css"
import Prism from "../assets/prism.js"
import * as wasm from "../../wasm-src/pkg/wasm_src.js";
import InputBox from './InputBox';


function DaySolver({ day, aoc_puzzle_link, aoc_code_link }) {
  const [inputContent, setInputContent] = useState("");
  const [part1Ans, setPart1Ans] = useState(null);
  const [part2Ans, setPart2Ans] = useState(null);
  const [part1Err, setPart1Err] = useState(null);
  const [part2Err, setPart2Err] = useState(null);

  useEffect(() => {
    Prism.highlightAll();
  }, []);

  useEffect(() => {
    if (inputContent !== "") {
      try {
        // let result = wasm.day14_part1(inputContent);
        let result = wasm.solve(inputContent, day, 1)
        console.log("Result", result);
        setPart1Ans(result)
        setPart1Err(null)
      } catch (error) {
        console.log("Error: ", error);
        setPart1Ans(null)
        setPart1Err(error)
      }
      try {
        // let result = wasm.day14_part1(inputContent);
        let result = wasm.solve(inputContent, day, 2)
        console.log("Result", result);
        setPart2Ans(result)
        setPart2Err(null)
      } catch (error) {
        console.log("Error: ", error);
        setPart2Ans(null)
        setPart2Err(error)
      }
    } else {
      setPart1Ans(null)
      setPart1Err(null)
      setPart2Ans(null)
      setPart2Err(null)
    }

  }, [inputContent])

  const format_response = (ans, err) => {
    if (ans === null && err === null) {
      return <> &lt;Waiting for Input&gt;</>
    } else if (ans !== null) {
      return <y>{ans}</y>
    } else {
      return <> &lt;Input Error&gt; <br /> <r> {err}</r></>
    }
  }

  return (
    <>
      <InputBox
        inputContent={inputContent}
        setInputContent={setInputContent}
      />
      <p >
        Part 1: {format_response(part1Ans, part1Err)} <br />
        Part 2: {format_response(part2Ans, part2Err)}<br />
        <a href={aoc_puzzle_link}>Puzzle</a>
        {' '}
        <a href={aoc_code_link}>solution</a>
      </p > <br />
    </>
  )
}

export default DaySolver
