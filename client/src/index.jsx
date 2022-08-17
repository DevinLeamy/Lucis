import React, { useRef } from "react";
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
        const canvasRef = useRef();

        const render_preview = (_e) => {
            let requestEmitter = new RequestEmitter();
            requestEmitter.send_request(pool)
                .then(wasm_image => {
                    console.log(wasm_image)
                    display_image(wasm_image)
                    requestEmitter.display_image(wasm_image)
                })
        }

        const colorToRGB = (color) => {
            return `rgb(${color.red}, ${color.green}, ${color.blue})`;
        }

        const display_image = (image) => {
            let width = image.width;
            let height = image.height;
            let buffer = image.buffer;

            let canvas = canvasRef.current;

            // canvas.setHeight(height);
            // canvas.setWidth(width);

            let context = canvas.getContext("2d");

            for (let i = 0; i < height; ++i) {
                for (let j = 0; j < width; ++j) {
                    let color = buffer[i][j];

                    context.setFillColor(colorToRGB(color));
                    context.fillRect(j, height - 1 - i, 1, 1);
                }
            }
        }

        return (
            <div>
                <button onClick={render_preview}>
                    Render
                </button>
                <canvas width={600} height={600} ref={canvasRef} />
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
