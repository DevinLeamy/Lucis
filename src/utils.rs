use wasm_bindgen::{JsValue, JsCast};
use web_sys::console::log_1;

pub fn log(s: String) {
    log_1(&JsValue::from(s));
}

pub fn download_image(png_image: String) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let link = document
        .create_element("a")
        .unwrap()
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .unwrap();

    let now = instant::now() as u32;
    let file_name = format!("{}_render.png", now);
    let _res = link.set_download(file_name.as_str());
    link.set_href(png_image.as_str());
    link.click();
}


