import React, {useEffect} from 'react';
import logo from './logo.svg';
import './App.css';
import init from 'gameboy';
import { add, dec } from 'gameboy';

function App() {
    console.log("Hello")
    useEffect(() => {
	init().then(() => {
	    console.log(add(5, 6))
        console.log(dec(5, 6))
	})
    }, [])

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
