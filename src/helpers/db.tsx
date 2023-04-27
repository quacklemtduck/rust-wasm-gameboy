import { Game } from "../components/GameSelect";

const dbName = "WebBoy"

let db: IDBDatabase
const request = indexedDB.open(dbName, 1)

request.onerror = () => {
    console.log("Error opening database for games")
}

request.onupgradeneeded = (event) => {
    db = request.result;
    db.createObjectStore("games", {keyPath: "name"})
}

request.onsuccess = () => {
    db = request.result
}

export const saveGame = (game: Game) => {
    return new Promise<void>((resolve, reject) => {
        const tx = db.transaction("games", "readwrite")
        const store = tx.objectStore("games")
        store.put(game)
        tx.oncomplete = () => {
            console.log("Saved game to db")
            resolve()
        }
    })
    
}

export const getGames = () => {
    return new Promise<Game[]>((resolve, reject) => {
        if (db == null) {
            reject("No db yet")
        }
        const tx = db.transaction("games")
        const store = tx.objectStore("games")
        const req = store.getAll()
        req.onsuccess = () => {
            resolve(req.result)
        }
    })

}