import { useState } from 'react'
import '../styles/AoC.css'


function Day404() {
  let s = `\
  _  _      ___    _  _                  _______   ____     _____
 | || |    / _ \\  | || |                |__   __| |  _ \\   / ____|
 | || |_  | | | | | || |_     ______       | |    | |_) | | |
 |__   _| | | | | |__   _|   |______|      | |    |  _ <  | |
    | |   | |_| |    | |                   | |    | |_) | | |____
    |_|    \\___/     |_|                   |_|    |____/   \\_____|
`
  return (
    <>
      <div id={'day404'}>
        <h1 >
          Day 404
        </h1>
        <p className={'red bold ascii-art'}>{s}</p>
      </div>
    </>
  )
}

export default Day404
