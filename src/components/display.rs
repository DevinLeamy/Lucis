// use js_sys::{Promise, Function};
// use ray_tracer::{Scene, RayTracer, Image, Camera, WorkerPool, Vec3};
// use wasm_bindgen::prelude::{wasm_bindgen, Closure};
// use wasm_bindgen::{JsCast, JsValue};

// use web_sys::console::log_1;

// use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
// use std::rc::Rc;
// use yew::prelude::*;
// use yewdux::prelude::*;

// use instant::Instant;

// const CANVAS_WIDTH: u32 = 1000;
// const CANVAS_HEIGHT: u32 = (CANVAS_WIDTH as f64 / (16.0 / 9.0)) as u32;

// #[derive(Default, Store)]
// pub struct FrameStore {
//     frame: Option<Image>,
// }

// impl PartialEq for FrameStore {
//     fn eq(&self, _: &Self) -> bool { false }
// }

// #[derive(Default, Store, Clone)]
// pub struct CameraStore {
//     camera: Camera 
// }


// enum CanvasClickState {
//     Clicked(i32, i32),
//     Idle,
// }

// impl Default for CanvasClickState {
//     fn default() -> Self {
//        CanvasClickState::Idle 
//     }
// }


// #[derive(Default, Store)]
// pub struct CanvasClickStore {
//     click_state: CanvasClickState
// }

// impl PartialEq for CanvasClickStore {
//     fn eq(&self, _: &Self) -> bool { false }
// }

// impl PartialEq for CameraStore {
//     fn eq(&self, _: &Self) -> bool { false }
// }

// fn log(s: String) {
//     log_1(&JsValue::from(s));
// }


// #[function_component(FnRayTracerDisplay)]
// pub fn display() -> Html {
//     let (camera_store, camera_dispatch) = use_store::<CameraStore>();
//     let (frame_store, frame_dispatch) = use_store::<FrameStore>();
//     let canvas_ctx = use_state::<Option<CanvasRenderingContext2d>>(|| None);
//     let canvas_ref = use_node_ref();

//     {
//         let canvas_ref = canvas_ref.clone();

//         use_effect_with_deps(|canvas_ref: NodeRef| {
//             let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();

//             canvas.set_height(CANVAS_HEIGHT);
//             canvas.set_width(CANVAS_WIDTH);
    
//             let on_canvas_click = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
//                 log(format!("{} {}", event.offset_x(), event.offset_y()));
//                 let dispatch = Dispatch::<CanvasClickStore>::new();
//                 dispatch.set(CanvasClickStore {
//                     click_state: CanvasClickState::Clicked(event.offset_x(), event.offset_y())
//                 });
//             });
//             canvas.add_event_listener_with_callback("mousedown", on_canvas_click.as_ref().unchecked_ref()).unwrap();
//             on_canvas_click.forget();
    
//             canvas_ctx = Some(
//                 canvas
//                     .get_context("2d")
//                     .unwrap()
//                     .unwrap()
//                     .dyn_into::<web_sys::CanvasRenderingContext2d>()
//                     .unwrap(),
//             );
//         })
//     }

//     let on_camera_change = camera_dispatch.reduce_mut_callback(|camera| {

//     });


//     fn get_canvas_image(canvas_ref: NodeRef) -> String {
//         let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
//         canvas.to_data_url_with_type("image/png").unwrap()
//     }

//     fn download_image(png_image: String) {
//         let window = web_sys::window().expect("no global `window` exists");
//         let document = window.document().expect("should have a document on window");
//         let link = document
//             .create_element("a")
//             .unwrap()
//             .dyn_into::<web_sys::HtmlAnchorElement>()
//             .unwrap();

//         let now = instant::now() as u32;
//         let file_name = format!("{}_render.png", now);
//         let _res = link.set_download(file_name.as_str());
//         link.set_href(png_image.as_str());
//         link.click();
//     }

//     let on_download = || {
//         match frame_store.as_ref().frame {
//             Some(_) => {
//                 let canvas_image = get_canvas_image(canvas_ref);
//                 download_image(canvas_image)
//             }
//             None => {
//                 log::info!("There is no frame to download");
//             }
//         }
//     };

//     html! {
//         <div>
//             <button id="create_frame_btn">
//                 { "Hook" }
//             </button>
//             <h5>{format!("Origin: {:?}", camera_store.camera.origin())}</h5>
//             <div>
//                 <button onclick={increase_x}>{"+x"}</button>
//                 <button onclick={decrease_x}>{"-x"}</button>
//             </div>
//             <div>
//                 <button onclick={increase_y}>{"+y"}</button>
//                 <button onclick={decrease_y}>{"-y"}</button>
//             </div>
//             <div>
//                 <button onclick={increase_z}>{"+z"}</button>
//                 <button onclick={decrease_z}>{"-z"}</button>
//             </div>
//             <button onclick={request_render}>
//                 { "Render" }
//             </button>
//             <div>
//                 <h1 class="display">
//                     {"Display"}
//                 </h1>
//                 <canvas ref={canvas_ref} />
//             </div>
//             <button onclick={request_download}>
//                 { "Download Image" }
//             </button>
//             <h5>{format!("{:?}", match self.render_status {
//                 RenderStatus::Rendering(_)     => "Rendering".to_string(),
//                 RenderStatus::Complete(t0, t1) => {
//                     let elapsed = t1.duration_since(t0);
//                     format!("{:?}", elapsed)
//                 },
//                 RenderStatus::Idle             => "".to_string()
//             })}</h5>
//         </div>
//     }
// }
