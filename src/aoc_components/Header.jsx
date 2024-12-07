import { useState } from 'react'
import "../styles/AoC.css"

function Header() {

  return (
    <>
      <div className={"header green-glow"}>
        Advent of Rust
      </div>
      <div>
        --- Learn Rust through Advent of Code ---
      </div>
      <div>
        <a className={"a-selected"} hhref="https://allavinner.github.io/aoc-2024/">2024</a>
        {" "}
        <a href="https://allavinner.github.io/aoc-2023/">2023</a>
      </div>
    </>
  )
}

export default Header
