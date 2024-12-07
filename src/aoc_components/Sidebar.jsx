import { useState } from 'react'
import '../styles/AoC.css'

function ListItem({ title, selectedPage, selectPage }) {
  let classNames;
  if (title == selectedPage) {
    classNames = "sidebar-list-item yellow"
  } else {
    classNames = "sidebar-list-item"
  }
  return <a className={classNames} onClick={() => {
    selectPage(title);
    console.log(title, selectedPage)
  }
  }>
    {title}
    {
      title == selectedPage && "*"
    }
  </a>
}


function Sidebar({ pageTitles, selectedPage, selectPage }) {

  return (
    <>
      <div className={'sidebar'}>
        <ul>
          {pageTitles.map((v, i) => {
            return <li key={i}>
              <ListItem title={v} selectedPage={selectedPage} selectPage={selectPage} />
            </li>
          })}
        </ul>
      </div>
    </>
  )
}

export default Sidebar
