import { useState } from 'react'
import '../styles/AoC.css'


function TextFileUpload({ setText }) {

  function handleFileUpload(e) {
    var fileReader = new FileReader();
    fileReader.addEventListener(
      "load",
      () => {
        // this will then display a text file
        console.log('in listner before asignment')
        setText(fileReader.result.replace(/\r/g, ''));
        console.log('in listner after asignment')
      },
      false,
    );

    if (e.target.files) {
      console.log('Before')
      fileReader.readAsText(e.target.files[0], "UTF-8");
      console.log('After')
    }
  }

  function handleFileUpload2(e) {
    const next_file = URL.createObjectURL(e.target.files[0])
    if (!next_file) {
      console.log("No file found")
      return "<No File>";
    }
    fetch(next_file)
      .then((response) => {
        var fileReader = new FileReader();
        fileReader.readAsText(response.blob(), "UTF-8");
      })

  }

  return (
    <>
      <input type="file" id="myFile" multiple size="50" onChange={handleFileUpload}></input>
    </>
  )
}

export default TextFileUpload
