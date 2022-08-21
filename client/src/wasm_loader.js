export function loadWasm(callback) {
    wasm_bindgen("./glue_bg.wasm")
        .then((wasm) => {
            console.log("Loaded WebAssembly module");
            callback(wasm)
        })
        .catch(console.error);
}

export const { WorkerPool, RequestEmitter } = wasm_bindgen;
