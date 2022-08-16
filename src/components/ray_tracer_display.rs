use js_sys::{Promise, Function};
use ray_tracer::{Scene, RayTracer, Image, Camera, WorkerPool, Vec3};
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::{JsCast, JsValue};

use web_sys::console::log_1;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

use instant::Instant;

const CANVAS_WIDTH: u32 = 1000;
const CANVAS_HEIGHT: u32 = (CANVAS_WIDTH as f64 / (16.0 / 9.0)) as u32;


#[derive(Debug)]
pub enum RenderStatus {
    Complete(Instant, Instant),
    Rendering(Instant),
    Idle,
}

#[derive(Default, Store)]
pub struct FrameStore {
    frame: Option<Image>,
}

impl PartialEq for FrameStore {
    fn eq(&self, _: &Self) -> bool { false }
}

#[derive(Default, Store)]
pub struct CameraStore {
    camera: Camera 
}


enum CanvasClickState {
    Clicked(i32, i32),
    Idle,
}

impl Default for CanvasClickState {
    fn default() -> Self {
       CanvasClickState::Idle 
    }
}


#[derive(Default, Store)]
pub struct CanvasClickStore {
    click_state: CanvasClickState
}

impl PartialEq for CanvasClickStore {
    fn eq(&self, _: &Self) -> bool { false }
}

impl PartialEq for CameraStore {
    fn eq(&self, _: &Self) -> bool { false }
}

pub struct RayTracerDisplay {
    canvas_ref: NodeRef,
    canvas: Option<CanvasRenderingContext2d>,
    frame_store: Rc<FrameStore>,
    #[allow(dead_code)]
    frame_dispatch: Dispatch<FrameStore>,
    camera_store: Rc<CameraStore>,
    #[allow(dead_code)]
    camera_dispatch: Dispatch<CameraStore>,
    render_status: RenderStatus,
}

pub enum Signal {
    Render,
    RenderComplete,
    Download,
    UpdateFrame(Rc<FrameStore>),
    OnCameraUpdate(Rc<CameraStore>),
    OnCanvasClick(i32, i32),
}

impl Component for RayTracerDisplay {
    type Message = Signal;
    type Properties = (); 

    fn create(ctx: &Context<Self>) -> Self {
        let frame_update_callback = ctx.link().callback(Signal::UpdateFrame);
        let camera_update_callback = ctx.link().callback(Signal::OnCameraUpdate);
        let frame_dispatch = Dispatch::<FrameStore>::subscribe(frame_update_callback);
        let camera_dispatch = Dispatch::<CameraStore>::subscribe(camera_update_callback);

        Self {
            canvas_ref: NodeRef::default(),
            canvas: None,
            frame_store: frame_dispatch.get(),
            frame_dispatch,
            camera_store: camera_dispatch.get(),
            camera_dispatch,
            render_status: RenderStatus::Idle,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, signal: Self::Message) -> bool {
        match signal {
            Signal::Render => {
                log_1(&JsValue::from("Requesting Render"));
                self.render_status = RenderStatus::Rendering(Instant::now());
                self.request_render();
            }
            Signal::RenderComplete => {
                if let RenderStatus::Rendering(start_time) = self.render_status {
                    self.render_status = RenderStatus::Complete(start_time, Instant::now());
                    log::info!("Render complete!");
                }
            }
            Signal::Download => {
                self.download_render();
            }
            Signal::UpdateFrame(frame_store) => {
                log::info!("Update frame!");
                self.frame_store = frame_store;
                if self.frame_store.as_ref().frame.is_some() {
                    self.render(ctx);
                }
            }
            Signal::OnCameraUpdate(camera_store) => {
                self.camera_store = camera_store;
            }
            Signal::OnCanvasClick(x, y) => {
                log("CLICKED".to_string());
            }
            _ => (),
        }
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.initialize_canvas(_ctx);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let request_download = link.callback(|_| {
            log::info!("Requesting download");
            Signal::Download
        });

        let request_render = link.callback(|_| {
            Signal::Render
        });

        let increase_y = link.callback(|_| {
            RayTracerDisplay::translate_camera(Vec3::new(0.0, 0.5, 0.0));
            Signal::Render
        });

        let decrease_y = link.callback(|_: MouseEvent| {
            RayTracerDisplay::translate_camera(Vec3::new(0.0, -0.5, 0.0));
            Signal::Render
        });

        let increase_x = link.callback(|_| {
            RayTracerDisplay::translate_camera(Vec3::new(0.5, 0.0, 0.0));
            Signal::Render
        });

        let decrease_x = link.callback(|_: MouseEvent| {
            RayTracerDisplay::translate_camera(Vec3::new(-0.5, 0.0, 0.0));
            Signal::Render
        });

        let increase_z = link.callback(|_| {
            RayTracerDisplay::translate_camera(Vec3::new(0.0, 0.0, 0.5));
            Signal::Render
        });

        let decrease_z = link.callback(|_: MouseEvent| {
            RayTracerDisplay::translate_camera(Vec3::new(-0.5, 0.0, -0.5));
            Signal::Render
        });

        let dispatch = Dispatch::<CameraStore>::new();
        let camera = dispatch.get().camera; 

        html! {
            <div>
                <button id="create_frame_btn">
                    { "Hook" }
                </button>
                <h5>{format!("Origin: {:?}", camera.origin())}</h5>
                <div>
                    <button onclick={increase_x}>{"+x"}</button>
                    <button onclick={decrease_x}>{"-x"}</button>
                </div>
                <div>
                    <button onclick={increase_y}>{"+y"}</button>
                    <button onclick={decrease_y}>{"-y"}</button>
                </div>
                <div>
                    <button onclick={increase_z}>{"+z"}</button>
                    <button onclick={decrease_z}>{"-z"}</button>
                </div>
                <button onclick={request_render}>
                    { "Render" }
                </button>
                <div>
                    <h1 class="display">
                        {"Display"}
                    </h1>
                    <canvas ref={self.canvas_ref.clone()} />
                </div>
                <button onclick={request_download}>
                    { "Download Image" }
                </button>
                <h5>{format!("{:?}", match self.render_status {
                    RenderStatus::Rendering(_)     => "Rendering".to_string(),
                    RenderStatus::Complete(t0, t1) => {
                        let elapsed = t1.duration_since(t0);
                        format!("{:?}", elapsed)
                    },
                    RenderStatus::Idle             => "".to_string()
                })}</h5>
            </div>
        }
    }
}

fn log(s: String) {
    log_1(&JsValue::from(s));
}

impl RayTracerDisplay {
    fn translate_camera(translation: Vec3) {
        log_1(&JsValue::from("Decrease X"));
        let dispatch = Dispatch::<CameraStore>::new();
        let mut camera: Camera = dispatch.get().camera; 

        camera.translate(translation);

        dispatch.set(CameraStore { camera }); 
    }
    fn initialize_canvas(&mut self, context: &Context<Self>) {
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

        canvas.set_height(CANVAS_HEIGHT);
        canvas.set_width(CANVAS_WIDTH);

        let on_canvas_click = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            log(format!("{} {}", event.offset_x(), event.offset_y()));
            // context.link().send_message(Signal::OnCanvasClick(event.offset_x(), event.offset_y()));
        });
        canvas.add_event_listener_with_callback("mousedown", on_canvas_click.as_ref().unchecked_ref()).unwrap();
        on_canvas_click.forget();

        self.canvas = Some(
            canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap(),
        );
    }


    fn request_render(&self) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let render_btn = document
            .get_element_by_id("create_frame_btn")
            .unwrap()
            .dyn_into::<web_sys::HtmlButtonElement>()
            .unwrap();
        let _res = render_btn.onclick().unwrap().call0(&JsValue::undefined());
    }

    fn render(&mut self, ctx: &Context<Self>) {
        let canvas = self.canvas.as_ref().unwrap();
        let frame = self.frame_store.frame.as_ref().unwrap();

        for i in 0..CANVAS_HEIGHT {
            for j in 0..CANVAS_WIDTH {
                let color = frame.get_color(i, j);
                let js_color: JsValue = JsValue::from_str(
                    format!("rgb({}, {}, {})", color.red, color.green, color.blue).as_str(),
                );

                canvas.set_fill_style(&js_color);
                canvas.fill_rect(j.into(), (CANVAS_HEIGHT - 1 - i).into(), 1.0, 1.0);
            }
        }

        ctx.link().send_message(Signal::RenderComplete);
    }

    fn get_canvas_image(&self) -> String {
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        canvas.to_data_url_with_type("image/png").unwrap()
    }

    fn download_image(&self, png_image: String) {
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

    fn download_render(&self) {
        match &self.frame_store.as_ref().frame {
            Some(_) => {
                let canvas_image = self.get_canvas_image();
                self.download_image(canvas_image)
            }
            None => {
                log::info!("There is no frame to download");
            }
        }
    }
}

#[wasm_bindgen]
pub struct RequestEmitter { }

#[allow(dead_code)]
#[allow(unused_variables)]
#[wasm_bindgen]
impl RequestEmitter {
    #[wasm_bindgen(constructor)] 
    pub fn new() -> Result<RequestEmitter, JsValue> {
        Ok(RequestEmitter {})
    }

    /// request an image to the rendered
    /// returns a callback to the resulting, serialized, image
    pub fn send_request(&self, pool: &WorkerPool) -> Result<Promise, JsValue> {
        let camera: Camera = Dispatch::<CameraStore>::new().get().camera;
        RayTracer::render_scene_wasm(Scene::materials(), camera, CANVAS_WIDTH, CANVAS_HEIGHT, pool)
    }

    /// display a serialized image 
    pub fn display_image(&self, image: &JsValue) {
        let dispatch = Dispatch::<FrameStore>::new();
        dispatch.set(FrameStore { frame: Some(image.into_serde().unwrap()) });
    }
}


