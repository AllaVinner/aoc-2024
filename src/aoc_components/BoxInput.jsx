import { useState, useEffect } from 'react'
import TextFileUpload from './TextFileUploader'
import "../styles/AoC.css"

function BoxInput({ inputContent, setInputContent }) {
  const [fileContent, setFileContent] = useState("");
  useEffect(() => {
    setInputContent(fileContent)
  }, [fileContent])
  return (
    <>
      <div className='aoc-input'>
        Input text or upload file:&nbsp;
        <TextFileUpload setText={setFileContent} />
        <br></br>
        <textarea
          wrap={'off'}
          name="Text1"
          cols="60"
          rows="8"
          value={inputContent}
          onChange={(e) => setInputContent(e.target.value)}
        >
        </textarea>
      </div>
    </>
  )
}

export default BoxInput
