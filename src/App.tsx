import React, {useEffect, useRef, useState} from 'react';
import './App.css';
import init, {GameBoy} from 'gameboy';

function App() {
    let canvasRef = useRef<HTMLCanvasElement>(null)
    let fileRef = useRef(null)

    const [gb, setGb] = useState<GameBoy | null>(null)

    const [ready, setReady] = useState(false)

    useEffect(() => {
        init().then(() => {
            setReady(true)
        })
    }, [])

    let onFile = (e: any) => {
        e.stopPropagation()
        e.preventDefault()
        let file = e.target.files[0]
        console.log(file)
        loadFileIntoUint8Array(file, (val) => {
            setGb(GameBoy.new(val))
        })
    }

    function loadFileIntoUint8Array(file: any, callback: (val: Uint8Array) => void) {
        const reader = new FileReader();
        reader.onload = function() {
            const arrayBuffer: any = reader.result;
            const uint8Array = new Uint8Array(arrayBuffer);
            callback(uint8Array);
        }
        reader.readAsArrayBuffer(file);
    }

    let run = async () => {
        const ctx = canvasRef?.current?.getContext("2d")
        if (ctx == null) return;
        gb?.start(ctx)
    }

  return (
    <div className="App">
        <canvas ref={canvasRef} width={160} height={144}/>
        {ready &&
            <input ref={fileRef} type={"file"} onChange={onFile} />
        }
        { gb != null &&
            <button onClick={run}>Run</button>
        }

    </div>
  );
}

export default App;
