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
    </>
  )
}

export default Header
