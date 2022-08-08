let mod; // wasm module 
let pool; // web worker pool 

function loadWasm() {
    wasm_bindgen("./Lucis_bg.wasm")
        .then(wasm => {
            console.log("Loaded WebAssembly module");
            main(wasm);   
        })
        .catch(console.error)
}
loadWasm();
const { WorkerPool } = wasm_bindgen;
let threadCount = navigator.hardwareConcurrency;

function main(wasm) {
    mod = wasm;
    pool = new WorkerPool(threadCount);

    mod.launch_yew();
}
