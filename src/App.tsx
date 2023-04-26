import React, {useEffect, useRef, useState} from 'react';
import './App.css';
import init, {GameBoy} from 'gameboy';
import { useGamepads } from 'react-gamepads';

function App() {
    let canvasRef = useRef<HTMLCanvasElement>(null)
    let fileRef = useRef(null)

    const [gb, setGb] = useState<GameBoy | null>(null)

    const [ready, setReady] = useState(false)

    let animationRef = useRef<any>(null)
    const [paused, setPaused] = useState(true)
    const [started, setStarted] = useState(false)

    let lastRenderRef = useRef(0)
    let [fps, setFps] = useState(0)

    let ARef = useRef(0)
    let BRef = useRef(0)
    let UpRef = useRef(0)
    let RightRef = useRef(0)
    let DownRef = useRef(0)
    let LeftRef = useRef(0)
    let SelectRef = useRef(0)
    let StartRef = useRef(0)

    useGamepads(gamepads => {
        console.log(gamepads)
    })

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
            setGb(GameBoy.new(val, file.name))
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
        if (e.code === 'ArrowUp') {
            UpRef.current = 1;
        }
        else if (e.code === 'ArrowRight') {
            RightRef.current = 1;
        }
        else if (e.code === 'ArrowDown') {
            DownRef.current = 1;
        }
        else if (e.code === 'ArrowLeft') {
            LeftRef.current = 1;
        }
        else if (e.code === 'ArrowUp') {
            UpRef.current = 1;
        }
        else if (e.code === 'KeyS') {
            ARef.current = 1;
        }
        else if (e.code === 'KeyA') {
            BRef.current = 1;
        }
        else if (e.code === 'Period') {
            SelectRef.current = 1;
        }
        else if (e.code === 'Enter') {
            StartRef.current = 1;
        }
    }

    let onKeyUp = (e: React.KeyboardEvent<HTMLCanvasElement>) => {

        if (e.code === 'ArrowUp') {
            UpRef.current = 0;
        }
        else if (e.code === 'ArrowRight') {
            RightRef.current = 0;
        }
        else if (e.code === 'ArrowDown') {
            DownRef.current = 0;
        }
        else if (e.code === 'ArrowLeft') {
            LeftRef.current = 0;
        }
        else if (e.code === 'ArrowUp') {
            UpRef.current = 0;
        }
        else if (e.code === 'KeyS') {
            ARef.current = 0;
        }
        else if (e.code === 'KeyA') {
            BRef.current = 0;
        }
        else if (e.code === 'Period') {
            SelectRef.current = 0;
        }
        else if (e.code === 'Enter') {
            StartRef.current = 0;
        }
    }

    let run = () => {
        const ctx = canvasRef?.current?.getContext("2d")
        if (ctx == null) return;
        gb?.start(ctx)
        animationRef.current = requestAnimationFrame(loop)
        setPaused(false)
        setStarted(true)
    }

    let loop = (delta: DOMHighResTimeStamp) => {
        let fpsTmp = (1 / (delta - lastRenderRef.current)) * 1000
        lastRenderRef.current = delta
        setFps(fpsTmp)        
        const ctx = canvasRef?.current?.getContext("2d")
        if (ctx == null) return;

        let up = UpRef.current
        let right = RightRef.current
        let down = DownRef.current
        let left = LeftRef.current
        let a = ARef.current
        let b = BRef.current
        let select = SelectRef.current
        let start = StartRef.current

        const gp = navigator.getGamepads()?.[0]
        if (gp != null) {
            if (gp.axes[0] > 0.5) {
                right = 1
            }
            if (gp.axes[0] < -0.5) {
                left = 1
            }
            if (gp.axes[1] > 0.5) {
                down = 1
            }
            if (gp.axes[1] < -0.5) {
                up = 1
            }
            if (gp.buttons[0].pressed || gp.buttons[4].pressed) {
                b = 1
            }
            if (gp.buttons[1].pressed || gp.buttons[3].pressed) {
                a = 1
            }
            if (gp.buttons[11].pressed) {
                start = 1
            }
            if (gp.buttons[10].pressed) {
                select = 1
            }
        }


        gb?.set_joypad_state(up, right, down, left, a, b, select, start);
        gb?.run(ctx);
        animationRef.current = requestAnimationFrame(loop)
    }

    let getPauseButton = () => {
        return (
            (paused) ?
            <button onClick={() => {
                animationRef.current = requestAnimationFrame(loop)
                setPaused(false)
            }}>Start</button>:
            <button onClick={() => {
                cancelAnimationFrame(animationRef.current)
                animationRef.current = null
                setPaused(true)
            }}>Pause</button>
        )
    }

  return (
    <div className="App">
        <p>FPS: {fps}</p>
        <div className='container'>
            <div className='canvas-container'>
                <canvas tabIndex={0} ref={canvasRef} width={160} height={144} onKeyDown={(e) => onKeyDown(e)} onKeyUp={(e) => onKeyUp(e)}/>
            </div>
        {(ready && !started) &&
            <div>
                <input ref={fileRef} type={"file"} onChange={onFile} accept='.gb' />
                {gb != null && <button onClick={run}>Run</button>}
            </div>
        }
        
        {
            started ? getPauseButton() : null
        }
        </div>
    </div>
  );
}

export default App;
