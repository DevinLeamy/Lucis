use crate::core::*;
use crate::pool::WorkerPool;
use crate::worlds::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

use web_sys::console::log_1;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, PartialEq, Store)]
pub struct FrameStore {
    frame: i32,
}

pub struct RayTracerDisplay {
    canvas_ref: NodeRef,
    canvas: Option<CanvasRenderingContext2d>,
    ray_tracer: RayTracer,
    render: Option<Frame>,
    canvas_width: u32,
    canvas_height: u32,
    frame_store: Rc<FrameStore>,
    dispatch: Dispatch<FrameStore>,

}

pub enum Signal {
    Render,
    RenderComplete(Frame),
    Download,
    UpdateFrame(Rc<FrameStore>),
}

impl Component for RayTracerDisplay {
    type Message = Signal;
    type Properties = (); // Props;

    fn create(ctx: &Context<Self>) -> Self {
        let camera = Camera::new(CameraConfig {
            time0: 0.0,
            time1: 1.0,
            ..CameraConfig::default()
        });
        let image_width = 400;
        let image_height = (image_width as f64 / camera.aspect_ratio()) as u32;

        let frame_update_callback = ctx.link().callback(Signal::UpdateFrame);
        let dispatch = Dispatch::<FrameStore>::subscribe(frame_update_callback);

        Self {
            canvas_ref: NodeRef::default(),
            canvas: None,
            ray_tracer: RayTracer::new(
                RayTracerConfig::default(),
                camera,
                Frame::new(image_width, image_height),
            ),
            render: None,
            canvas_width: image_width,
            canvas_height: image_height,
            frame_store: dispatch.get(),
            dispatch
        }
    }

    fn update(&mut self, ctx: &Context<Self>, signal: Self::Message) -> bool {
        match signal {
            Signal::RenderComplete(frame) => {
                self.render(frame, ctx);
                log::info!("Render complete!");
            }
            Signal::Download => {
                self.download_render();
            }
            Signal::UpdateFrame(frame_store) => {
                self.frame_store = frame_store;
            }
            _ => (),
        }
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.initialize_canvas();
        }
    }
    

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let request_download = link.callback(|_| {
            log::info!("Requesting download");
            Signal::Download
        });


        html! {
            <div>
                <button id="create_frame_btn">
                    { "Create frame!" }
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
                <h1>
                    {&format!("Frame: {}", self.frame_store.frame)}
                </h1>
            </div>
        }
    }
}

impl RayTracerDisplay {
    fn initialize_canvas(&mut self) {
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

        canvas.set_height(self.canvas_height);
        canvas.set_width(self.canvas_width);

        self.canvas = Some(
            canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap(),
        );
    }
    fn render(&mut self, frame: Frame, ctx: &Context<Self>) {
        let canvas = self.canvas.as_ref().unwrap();

        for i in 0..frame.width() {
            for j in 0..frame.height() {
                let color = frame.get_color(i, j);
                let js_color: JsValue = JsValue::from_str(
                    format!("rgb({}, {}, {})", color[0], color[1], color[2]).as_str(),
                );

                canvas.set_fill_style(&js_color);
                canvas.fill_rect(i.into(), (frame.height() - 1 - j).into(), 1.0, 1.0);
            }
        }

        self.render = Some(frame)
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
        match &self.render {
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
struct RequestEmitter { }

#[allow(dead_code)]
#[allow(unused_variables)]
#[wasm_bindgen]
impl RequestEmitter {
    #[wasm_bindgen(constructor)] 
    pub fn new() -> Result<RequestEmitter, JsValue> {
        Ok(RequestEmitter {})
    }

    pub fn send_request(self, pool: &WorkerPool) {
        log_1(&pool.size().into());
        let dispatch = Dispatch::<FrameStore>::new();
        dispatch.set(FrameStore { frame: pool.size() as i32 });
    }
}

/*
When the user clicks the "Render" button we
need to be able to pass a WorkerPool (that was initialized in
JavaScript) to the RayTracer's render method.

Options:
1. We pass a worker pool created during initialization, store it, and then retrieve it.
2. We trigger an event that causes the JavaScript to call a render method that 
   takes in a WorkerPool as input.

*/