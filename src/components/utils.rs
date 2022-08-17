use wasm_bindgen::JsCast;

pub fn get_element_ref<T: JsCast>(element_id: String) -> Result<T, ()> {
    let window = web_sys::window().expect("Window not found!");
    let document = window.document().expect("Document not found!");
    
    let element = document
        .get_element_by_id(element_id.as_str())
        .unwrap()
        .dyn_into::<T>()
        .unwrap();
    
    Ok(element)
}
