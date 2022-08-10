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
const { WorkerPool, Renderer } = wasm_bindgen;
let threadCount = navigator.hardwareConcurrency;

function main(wasm) {
    mod = wasm;
    pool = new WorkerPool(threadCount);

    mod.launch_yew();

    createFrameButton = document.getElementById("create_frame_btn")
    createFrameButton.onclick = render
}

let mod = null; // wasm module 
let pool = null; // web worker pool 

function render() {
    // console.log("(JS) Registered click")
    mod.test_on_click()
    console.log("(JS)", pool)
    let renderer = new Renderer();
    renderer.test_pass_workers(pool)
}


