import React, {useEffect, useRef, useState} from 'react';
import './App.css';
import init, {GameBoy} from 'gameboy';

function App() {
    let canvasRef = useRef<HTMLCanvasElement>(null)
    let bgRef = useRef<HTMLCanvasElement>(null)
    let fileRef = useRef(null)

    const [gb, setGb] = useState<GameBoy | null>(null)

    const [ready, setReady] = useState(false)

    let animationRef = useRef<any>(null)
    const [paused, setPaused] = useState(true)

    let lastRenderRef = useRef(0)
    let fpsRef = useRef(0)

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

    let loadFileIntoUint8Array = (file: any, callback: (val: Uint8Array) => void) => {
        const reader = new FileReader();
        reader.onload = function() {
            const arrayBuffer: any = reader.result;
            const uint8Array = new Uint8Array(arrayBuffer);
            callback(uint8Array);
        }
        reader.readAsArrayBuffer(file);
    }

    let onKeyDown = (e: React.KeyboardEvent<HTMLCanvasElement>) => {

        if (e.key === 's') {
        // console.log(e)
            gb?.set_joypad_state(0,0,0,0,1,0,0,0)
        }
    }

    let onKeyUp = (e: React.KeyboardEvent<HTMLCanvasElement>) => {

        if (e.key === 's') {
        console.log(e)
            gb?.set_joypad_state(0,0,0,0,0,0,0,0)
        }
    }

    let run = () => {
        const ctx = canvasRef?.current?.getContext("2d")
        if (ctx == null) return;
        gb?.start(ctx)
        animationRef.current = requestAnimationFrame(loop)
        setPaused(false)
    }

    let loop = (delta: DOMHighResTimeStamp) => {
        fpsRef.current++
        // console.log(`FPS: ${(1 / (delta - lastRenderRef.current)) * 1000} Rendertime: ${delta - lastRenderRef.current} Frames: ${fpsRef.current}`)
        lastRenderRef.current = delta
        const ctx = canvasRef?.current?.getContext("2d")
        const bgCtx = bgRef?.current?.getContext("2d")
        if (ctx == null || bgCtx == null) return;
        gb?.set_joypad_state(0,0,0,0,1,0,0,0);
        gb?.run(ctx, bgCtx);
        animationRef.current = requestAnimationFrame(loop)
    }

    let isPaused = () => {
        return animationRef.current == null
    }

  return (
    <div className="App">
        <canvas tabIndex={0} ref={canvasRef} width={160} height={144} onKeyPress={(e) => onKeyDown(e)} onKeyUp={(e) => onKeyUp(e)}/>
        {ready &&
            <input ref={fileRef} type={"file"} onChange={onFile} />
        }
        { gb != null &&
            <button onClick={run}>Run</button>
        }
        
        {
            paused ?
            <button onClick={() => {
                animationRef.current = requestAnimationFrame(loop)
                setPaused(false)
            }}>Start</button>:
            <button onClick={() => {
                cancelAnimationFrame(animationRef.current)
                animationRef.current = null
                setPaused(true)
            }}>Pause</button>
        }

        <canvas style={{width: 256, height: 256, border: "1px solid black"}} ref={bgRef} width={256} height={256} /> 

    </div>
  );
}

export default App;
