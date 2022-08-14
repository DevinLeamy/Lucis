use ray_tracer::{Scene, RayTracer, Image, Camera, WorkerPool};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

use web_sys::console::log_1;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Store)]
pub struct FrameStore {
    frame: Option<Image>,
}

impl PartialEq for FrameStore {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

pub struct RayTracerDisplay {
    canvas_ref: NodeRef,
    canvas: Option<CanvasRenderingContext2d>,
    render: Option<Image>,
    canvas_width: u32,
    canvas_height: u32,
    frame_store: Rc<FrameStore>,
    dispatch: Dispatch<FrameStore>,

}

pub enum Signal {
    Render,
    RenderComplete(Image),
    Download,
    UpdateFrame(Rc<FrameStore>),
}

impl Component for RayTracerDisplay {
    type Message = Signal;
    type Properties = (); // Props;

    fn create(ctx: &Context<Self>) -> Self {
        let camera = Camera::default();
        let image_width = 400;
        let image_height = (image_width as f64 / camera.aspect()) as u32;

        let frame_update_callback = ctx.link().callback(Signal::UpdateFrame);
        let dispatch = Dispatch::<FrameStore>::subscribe(frame_update_callback);

        Self {
            canvas_ref: NodeRef::default(),
            canvas: None,
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
                // self.render(frame, ctx);
                log::info!("Render complete!");
            }
            Signal::Download => {
                self.download_render();
            }
            Signal::UpdateFrame(frame_store) => {
                self.frame_store = frame_store;
                if (self.frame_store.as_ref().frame.is_some()) {
                    self.render(ctx);
                }
            }
            _ => (),
        }
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
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
                // <h1>
                //     {&format!("Frame: {}", self.frame_store.frame)}
                // </h1>
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
    fn render(&mut self, _ctx: &Context<Self>) {
        let canvas = self.canvas.as_ref().unwrap();
        let frame = self.frame_store.frame.as_ref().unwrap();

        for i in 0..self.canvas_width {
            for j in 0..self.canvas_height {
                let color = frame.get_color(i, j);
                let js_color: JsValue = JsValue::from_str(
                    format!("rgb({}, {}, {})", color.red, color.green, color.blue).as_str(),
                );

                canvas.set_fill_style(&js_color);
                canvas.fill_rect(i.into(), (self.canvas_height - 1 - j).into(), 1.0, 1.0);
            }
        }

        // self.render = Some(frame)
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
        // match &self.render {
        //     Some(_) => {
        //         let canvas_image = self.get_canvas_image();
        //         self.download_image(canvas_image)
        //     }
        //     None => {
        //         log::info!("There is no frame to download");
        //     }
        // }
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

        let image = RayTracer::render_scene_wasm(&Scene::simple(), Camera::default(), 400, 400, pool);
        dispatch.set(FrameStore { frame: Some(image) });

    }
}
