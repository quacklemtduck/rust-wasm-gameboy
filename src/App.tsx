import React, {useEffect, useRef, useState} from 'react';
import './App.css';
import init, {GameBoy} from 'gameboy';
import GameSelect, { Game } from './components/GameSelect';
import { Buttons } from './helpers/types';

function App() {
    let canvasRef = useRef<HTMLCanvasElement>(null)

    const [gb, setGb] = useState<GameBoy | null>(null)

    const [ready, setReady] = useState(false)

    let animationRef = useRef<any>(null)
    const [paused, setPaused] = useState(true)
    let pausedRef = useRef(true)
    const [started, setStarted] = useState(false)

    let lastRenderRef = useRef(0)
    const [fps, setFps] = useState(0)

    const [showAdvanced, setShowAdvanced] = useState(false)
    let speedRef = useRef(1)
    let speedCountRef = useRef(0)
    const [dynamic, setDynamic] = useState(true)
    let dynamicRef = useRef(true)
    let pauseCounterRef = useRef(0)

    const [showGameSelect, setShowGameSelect] = useState(false)

    let ARef = useRef(0)
    let BRef = useRef(0)
    let UpRef = useRef(0)
    let RightRef = useRef(0)
    let DownRef = useRef(0)
    let LeftRef = useRef(0)
    let SelectRef = useRef(0)
    let StartRef = useRef(0)

    let UpButtonRef = useRef<any>()
    let RightButtonRef = useRef<any>()
    let DownButtonRef = useRef<any>()
    let LeftButtonRef = useRef<any>()
    

    useEffect(() => {
        init().then(() => {
            setReady(true)
        })
    }, [])

    useEffect(() => {
        dynamicRef.current = dynamic
    }, [dynamic])

    let chooseGame = (g: Game) => {
        setGb(GameBoy.new(g.data, g.name))
        setShowGameSelect(false)
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

    let onTouchStart = (button: Buttons) => {
        switch (button) {
            case Buttons.UP:
                UpRef.current = 1
                break;
            case Buttons.RIGHT:
                RightRef.current = 1
                break;
            case Buttons.DOWN:
                DownRef.current = 1
                break;
            case Buttons.LEFT:
                LeftRef.current = 1
                break;
            case Buttons.A:
                ARef.current = 1
                break;
            case Buttons.B:
                BRef.current = 1
                break;
            case Buttons.SELECT:
                SelectRef.current = 1
                break;
            case Buttons.START:
                StartRef.current = 1
                break;
            default:
                break;
        }
    }

    let onTouchEnd = (button: Buttons) => {
        switch (button) {
            case Buttons.UP:
                UpRef.current = 0
                break;
            case Buttons.RIGHT:
                RightRef.current = 0
                break;
            case Buttons.DOWN:
                DownRef.current = 0
                break;
            case Buttons.LEFT:
                LeftRef.current = 0
                break;
            case Buttons.A:
                ARef.current = 0
                break;
            case Buttons.B:
                BRef.current = 0
                break;
            case Buttons.SELECT:
                SelectRef.current = 0
                break;
            case Buttons.START:
                StartRef.current = 0
                break;
            default:
                break;
        }
    }

    let onTouchMove = (e: React.TouchEvent<HTMLDivElement>) => {

        let upRect: DOMRect = UpButtonRef.current.getBoundingClientRect()
        let rightRect: DOMRect = RightButtonRef.current.getBoundingClientRect()
        let downRect: DOMRect = DownButtonRef.current.getBoundingClientRect()
        let leftRect: DOMRect = LeftButtonRef.current.getBoundingClientRect()

        UpRef.current = 0
        RightRef.current = 0
        DownRef.current = 0
        LeftRef.current = 0

        for (let i = 0; i < e.touches.length; i++) {
            let touch = e.touches[i]
            if (touch.pageX >= upRect.left &&
                touch.pageX <= upRect.right &&
                touch.pageY >= upRect.top &&
                touch.pageY <= upRect.bottom) {
                UpRef.current = 1
            }
            if (touch.pageX >= rightRect.left &&
                touch.pageX <= rightRect.right &&
                touch.pageY >= rightRect.top &&
                touch.pageY <= rightRect.bottom) {
                RightRef.current = 1
            }
            if (touch.pageX >= downRect.left &&
                touch.pageX <= downRect.right &&
                touch.pageY >= downRect.top &&
                touch.pageY <= downRect.bottom) {
                DownRef.current = 1
            }
            if (touch.pageX >= leftRect.left &&
                touch.pageX <= leftRect.right &&
                touch.pageY >= leftRect.top &&
                touch.pageY <= leftRect.bottom) {
                LeftRef.current = 1
            }
        }
    }
    // let test = () => {
    //     const ctx = canvasRef?.current?.getContext("2d")
    //     if (ctx == null) return;
    //     gb?.start()
    //     gb?.set_joypad_state(0,0,0,0,0,0,0,0)
    //     gb?.test()
    //     gb?.draw_frame(ctx)
    // }

    let run = () => {
        const ctx = canvasRef?.current?.getContext("2d")
        if (ctx == null) return;
        gb?.start()
        animationRef.current = requestAnimationFrame(loop)
        setPaused(false)
        pausedRef.current = false
        setStarted(true)
    }

    let loop = (delta: DOMHighResTimeStamp) => {
        let fpsTmp = (1 / (delta - lastRenderRef.current)) * 1000
        lastRenderRef.current = delta
        setFps(fpsTmp)        

        if (pauseCounterRef.current < 10) {
            pauseCounterRef.current++
            animationRef.current = requestAnimationFrame(loop)
            return
        }

        const ctx = canvasRef?.current?.getContext("2d")
        if (ctx == null || pausedRef.current) return;

        let up = UpRef.current
        let right = RightRef.current
        let down = DownRef.current
        let left = LeftRef.current
        let a = ARef.current
        let b = BRef.current
        let select = SelectRef.current
        let start = StartRef.current

        gb?.set_joypad_state(up, right, down, left, a, b, select, start);

        if (dynamicRef.current) {
            // console.log(`Dynamic count ${speedCountRef.current} + ${60 / fpsTmp}`)
            speedCountRef.current += 60 / fpsTmp

        } else {
            // console.log(`Set count ${speedCountRef.current} + ${speedRef.current}`)
            speedCountRef.current += speedRef.current
        }
        while (speedCountRef.current >= 1) {
            gb?.run()
            speedCountRef.current -= 1
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
                pausedRef.current = false
                pauseCounterRef.current = 0
            }}>Start</button>:
            <button onClick={() => {
                cancelAnimationFrame(animationRef.current)
                animationRef.current = null
                setPaused(true)
                pausedRef.current = true
            }}>Pause</button>
        )
    }

  return (
    <div className="App" onTouchMove={onTouchMove} onTouchStart={onTouchMove} onTouchEnd={onTouchMove}>
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
                        <div>
                            <label htmlFor="dynamicCheck">Dynamic FPS:</label>
                            <input id='dynamicCheck' type='checkbox' checked={dynamic} onChange={() => setDynamic((v) => !v)} />
                        </div>
                        <div style={{display: "flex", justifyContent: "space-between", marginTop: 5}}>
                            <label htmlFor="speedInput">Speed:</label>
                            <input disabled={dynamic} id='speedInput' type='number' min={0} step={0.1} defaultValue={speedRef.current} onChange={e => {
                                let val = e.target.value !== "" ? Number(e.target.value) : 1
                                speedRef.current = val
                            }
                                }/>
                        </div>
                    </div>
                : null}

            </div>

            <div className='mobile-controls'>
                <div className='mobile-top'>
                    <div className='mobile-dpad'>
                        <div className='dpad'>
                            <div />
                            <button ref={UpButtonRef} className='dup' onTouchStart={() => onTouchStart(Buttons.UP)}  onTouchEnd={() => onTouchEnd(Buttons.UP)}></button>
                            <div />

                            <button ref={LeftButtonRef} className='dleft' onTouchStart={() => onTouchStart(Buttons.LEFT)} onTouchEnd={() => onTouchEnd(Buttons.LEFT)}></button>
                            <div style={{backgroundColor: "#3a3e45"}}/>
                            <button ref={RightButtonRef} className='dright' onTouchStart={() => onTouchStart(Buttons.RIGHT)} onTouchEnd={() => onTouchEnd(Buttons.RIGHT)}></button>

                            <div />
                            <button ref={DownButtonRef} className='ddown' onTouchStart={() => onTouchStart(Buttons.DOWN)} onTouchEnd={() => onTouchEnd(Buttons.DOWN)}></button>
                            <div />

                        </div>
                    </div>
                    <div className='mobile-ab'>
                        <button className='ab-button' style={{marginRight: 15}} onTouchStart={() => onTouchStart(Buttons.B)} onTouchEnd={() => onTouchEnd(Buttons.B)}>B</button>
                        <button className='ab-button' style={{marginBottom: 20}} onTouchStart={() => onTouchStart(Buttons.A)} onTouchEnd={() => onTouchEnd(Buttons.A)}>A</button>
                    </div>
                </div>
                <div className='mobile-bottom'>
                    <button className='bottom-button' style={{marginRight: 10}} onTouchStart={() => onTouchStart(Buttons.SELECT)} onTouchEnd={() => onTouchEnd(Buttons.SELECT)}>select</button>
                    <button className='bottom-button' onTouchStart={() => onTouchStart(Buttons.START)} onTouchEnd={() => onTouchEnd(Buttons.START)}>start</button>
                </div>
            </div>
        </div>
    </div>
  );
}

export default App;
