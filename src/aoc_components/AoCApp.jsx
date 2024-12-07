import { useState } from 'react'
import '../styles/AoC.css'
import '../styles/index.css'
import Sidebar from "./Sidebar"
import Header from "./Header"


function AoCApp({ days }) {
  const today = 1;
  const [selectedPage, selectPage] = useState(days[today - 1].title)
  return (
    <>
      <div id={'dashoard'}>
        <div id={"header"}>
          <Header />
        </div>
        <div id={'sidebar'}>
          <Sidebar pageTitles={days.map((v) => v.title)} selectedPage={selectedPage} selectPage={selectPage} />
        </div>
        <div id={'content'}>
          {days.find((p) => p.title == selectedPage) ? days.find((p) => p.title == selectedPage).content : <Day404 />
          }
        </div>
      </div>
    </>
  )
}

export default AoCApp
