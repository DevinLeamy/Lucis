let createFrameButton = null; 

function loadWasm() {
    wasm_bindgen("./Lucis_bg.wasm")
        .then(wasm => {
            console.log("Loaded WebAssembly module");
            main(wasm);   
        })
        .catch(console.error)
}
loadWasm();
const { WorkerPool, RequestEmitter } = wasm_bindgen;
let threadCount = navigator.hardwareConcurrency;

function main(wasm) {
    mod = wasm;
    pool = new WorkerPool(threadCount);

    mod.launch_yew();

    setInterval(() => {
        createFrameButton = document.getElementById("create_frame_btn")
        if (createFrameButton != null) {
            createFrameButton.onclick = js_render
        }
    }, 100);

}

let mod = null; // wasm module 
let pool = null; // web worker pool 

function js_render() {
    console.log("(JS)", pool)
    let requestEmitter = new RequestEmitter();
    requestEmitter.send_request(pool)
        .then(wasm_image => {
            requestEmitter.display_image(wasm_image)
        })
}
