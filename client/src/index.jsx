import React from "react";
import ReactDOM from "react-dom/client";

function loadWasm() {
    wasm_bindgen("./glue_bg.wasm")
        .then((wasm) => {
            console.log("Loaded WebAssembly module");
            main(wasm);
        })
        .catch(console.error);
}
loadWasm();

const { WorkerPool, RequestEmitter } = wasm_bindgen;
let threadCount = navigator.hardwareConcurrency;

function main(wasm) {
    mod = wasm;
    pool = new WorkerPool(threadCount);

    const App = () => {
        const render_preview = (_e) => {
            let requestEmitter = new RequestEmitter();
            requestEmitter.send_request(pool)
                .then(wasm_image => {
                    requestEmitter.display_image(wasm_image)
                })
        }

        return (
            <div>
                <button onClick={() => wasm.big_computation()}>
                    Do Math
                </button>
                <button onClick={render_preview}>
                    Render
                </button>
            </div>
        )
    };

    const root = ReactDOM.createRoot(document.getElementById("root"));
    root.render(
        <React.StrictMode>
            <App />
        </React.StrictMode>
    );
}

let mod = null; // wasm module 
let pool = null; // web worker pool 

/*
Figure out how this can be done WITHOUT a bundler
*/
