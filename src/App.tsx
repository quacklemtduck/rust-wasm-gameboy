import React, {useEffect, useRef, useState} from 'react';
import './App.css';
import init, {GameBoy} from 'gameboy';
import GameSelect, { Game } from './components/GameSelect';

function App() {
    let canvasRef = useRef<HTMLCanvasElement>(null)

    const [gb, setGb] = useState<GameBoy | null>(null)

    const [ready, setReady] = useState(false)

    let animationRef = useRef<any>(null)
    const [paused, setPaused] = useState(true)
    const [started, setStarted] = useState(false)

    let lastRenderRef = useRef(0)
    const [fps, setFps] = useState(0)

    const [showAdvanced, setShowAdvanced] = useState(false)
    let speedRef = useRef(1)

    const [showGameSelect, setShowGameSelect] = useState(false)

    let ARef = useRef(0)
    let BRef = useRef(0)
    let UpRef = useRef(0)
    let RightRef = useRef(0)
    let DownRef = useRef(0)
    let LeftRef = useRef(0)
    let SelectRef = useRef(0)
    let StartRef = useRef(0)

    useEffect(() => {
        init().then(() => {
            setReady(true)
        })
    }, [])

    let chooseGame = (g: Game) => {
        setGb(GameBoy.new(g.data, g.name))
        setShowGameSelect(false)
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

    let test = () => {
        const ctx = canvasRef?.current?.getContext("2d")
        if (ctx == null) return;
        gb?.start()
        gb?.set_joypad_state(0,0,0,0,0,0,0,0)
        gb?.test(ctx)
        gb?.draw_frame(ctx)
    }

    let run = () => {
        const ctx = canvasRef?.current?.getContext("2d")
        if (ctx == null) return;
        gb?.start()
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

        //console.log("Frame")
        gb?.set_joypad_state(up, right, down, left, a, b, select, start);
        //console.log(speedRef.current)
        for (let i = 0; i < speedRef.current; i++){
            gb?.run(ctx);
        }
        gb?.draw_frame(ctx)
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
        <div className='container'>
            <div className='canvas-container'>
                <canvas tabIndex={0} ref={canvasRef} width={160} height={144} onKeyDown={(e) => onKeyDown(e)} onKeyUp={(e) => onKeyUp(e)}/>
            </div>
        {(ready && !started) &&
            <div>
                <button onClick={() => setShowGameSelect(true)}>Pick game</button>
                {gb != null && <button onClick={run}>Run</button>}
            </div>
        }

        <GameSelect onClose={() => {setShowGameSelect(false)}} show={showGameSelect} onChoose={(g) => chooseGame(g)}  />
        
        {
            started ? getPauseButton() : null
        }
        {/* <button onClick={test}>Test</button> */}
        <div className='advanced-container'>
            <div className='advanced-title click' onClick={() => setShowAdvanced(!showAdvanced)}>
                <span>Advanced</span>
                {showAdvanced ? <i className="fa fa-caret-square-o-down ml-5" aria-hidden="true"></i> : <i className="fa fa-caret-square-o-right ml-5" aria-hidden="true"></i>}
            </div>

            {showAdvanced ? 
                <div className='advanced-content'>
                    <div style={{display: "grid", gridTemplateColumns: "1fr 1fr"}}>
                        <span>FPS: {fps.toFixed(2)}</span>
                        <span>Game FPS: {(fps * speedRef.current).toFixed(2)}</span>
                    </div>
                    <div style={{display: "flex", justifyContent: "space-between", marginTop: 5}}>
                        <label htmlFor="speedInput">Speed:</label>
                        <input id='speedInput' type='number' min={1} step={1} defaultValue={speedRef.current} onChange={e => {
                            console.log("Update", e.target.value)
                            let val = e.target.value !== "" ? Number(e.target.value) : 1
                            speedRef.current = val
                        }
                            }/>
                    </div>
                </div>
            : null}

        </div>
        </div>
    </div>
  );
}

export default App;
