let createFrameButton = null; 
let renderPreviewBtn = null;

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

    /*
    Setup onclick functions that are to be called from Rust.
    This is a hack because calling non-browser Js from Rust
    seems to be challenging if you are bundling using
    "wasm-bindgen ... --target no-modules" (or --target web)
    */

    setInterval(() => {
        createFrameButton = document.getElementById("create_frame_btn")
        if (createFrameButton != null) {
            createFrameButton.onclick = js_render;
        }

        renderPreviewBtn = document.getElementById("element_preview")
        if (renderPreviewBtn != null) {
            renderPreviewBtn.onclick = renderPreview;
        }

    }, 100);

    mod.launch_yew();
}

let mod = null; // wasm module 
let pool = null; // web worker pool 

function js_render() {
    let requestEmitter = new RequestEmitter();
    requestEmitter.send_request(pool)
        .then(wasm_image => {
            requestEmitter.display_image(wasm_image)
        })
}

function renderPreview() {
    // tell Yew to render the element preview
    console.log("Render element preview");
}
