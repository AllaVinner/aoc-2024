import { useState } from 'react'
import '../styles/AoC.css'
import '../styles/index.css'
import Sidebar from "./Sidebar"
import Header from "./Header"
import Day404 from './Day404'


function AoCApp({ days }) {
  const today = 1;
  const [selectedPage, selectPage] = useState(days[today - 1].title)
  const day_titles = Array.from({ length: 25 }, (_, i) => String(i + 1).padStart(2, ' '))

  return (
    <>
      <div id={'dashoard'}>
        <div id={"header"}>
          <Header />
        </div>
        <div id={'sidebar'}>
          <Sidebar pageTitles={day_titles} selectedPage={selectedPage} selectPage={selectPage} />
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
