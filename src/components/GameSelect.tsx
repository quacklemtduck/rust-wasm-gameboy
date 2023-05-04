import React, { useEffect, useState } from 'react'

import '../styles/GameSelect.css'
import { getGames, saveGame, deleteGame } from '../helpers/db'

export interface Game {
    name: string,
    data: Uint8Array
}

const GameSelect = (props: {
    onClose: () => void,
    show: boolean,
    onChoose: (game: Game) => void
}) => {
    const [games, setGames] = useState<Game[]>([])

    useEffect(() => {
        if (props.show) {
            updateGamesList()
        }
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [props.show])

    const updateGamesList = () => {
        getGames().then((games) => {
            setGames(games)
        }).catch(() => {
            setTimeout(updateGamesList, 500)
        })
    }

    let onFile = (e: any) => {
        e.stopPropagation()
        e.preventDefault()
        let file: File = e.target.files[0]
        loadFileIntoUint8Array(file, (val) => {
            let g = {name: file.name, data: val}
            saveGame(g).then(() => {
                updateGamesList()
            })
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

    let onDelete = (game: Game) => {
	deleteGame(game)
        updateGamesList()
    }

    let selectGame = (game: Game) => {
        props.onChoose(game)
    }

    return (
        props.show ?
        <div className='backdrop'>
            <div className='window' onClick={(e) => e.stopPropagation}>
                <div className='header'>
                    <h3 style={{margin: 0}}>Select a game</h3>
                    <span className='close-button' onClick={props.onClose}>X</span>
                </div>
                <div className='content'>
                    <div className='game-table'>
                        {
                            games.length > 0 ?
                            games.map((g, i) => {
                                return (
                                    <GameRow key={i} game={g} onSelect={g => selectGame(g)} onDelete={g => onDelete(g)} />
                                )
                            })
                            : <span>No games loaded</span>
                        }
                    </div>
                </div>
                <div className='footer'>
                    <div className='file-picker'>
                        <input className='' id="file-picker" type={"file"} onChange={onFile} accept='.gb' hidden/>
                        <label className='click' htmlFor="file-picker">Select local file</label>
                    </div>
                </div>
            </div>
        </div>
        :
        <></>
    )
}

export default GameSelect

let GameRow = (props: {
    game: Game,
    onSelect: (game: Game) => void,
    onDelete: (game: Game) => void,
}) => {

    return <div className='game-row'>
        <span className='ellipsis'><span className='click' onClick={() => props.onDelete(props.game)}>🗑️</span> {props.game.name}</span>
        <button onClick={() => props.onSelect(props.game)}>Select</button>
    </div>
}
