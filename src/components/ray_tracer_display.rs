use crate::*;

use std::borrow::Borrow;
use wasm_bindgen::{JsCast, JsValue};

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct RayTracerDisplay {
    canvas_ref: NodeRef,
    canvas: Option<CanvasRenderingContext2d>,
}

pub enum Signal {
    Render,
    RenderComplete(Frame),
}

impl Component for RayTracerDisplay {
    type Message = Signal;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            canvas: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, signal: Self::Message) -> bool {
        match signal {
            Signal::Render => {
                log::info!("Requesting a frame!");
                ctx.link().send_future(async {
                    let frame = create_frame().await;
                    Signal::RenderComplete(frame)
                })
            }
            Signal::RenderComplete(frame) => {
                self.render(frame, ctx);
                log::info!("Render complete!");
            }
            _ => (),
        }
        true
    }

    // fn changed(&mut self, ctx: &Context<Self>) -> bool {

    // }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            log::info!("First render");
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

            canvas.set_height(500u32);
            canvas.set_width(600u32);

            self.canvas = Some(
                canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap(),
            );
            ctx.link().send_message(Signal::Render);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let request_render = link.callback(|_| {
            log::info!("Requesting a frame");
            Signal::Render
        });

        html! {
            <div>
                <button onclick={request_render}>
                    { "Create frame!" }
                </button>
                <div>
                    <h1>
                        {"Display"}
                    </h1>
                    <canvas ref={self.canvas_ref.clone()} />
                </div>
            </div>
        }
    }
}

impl RayTracerDisplay {
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
    }
}

async fn create_frame() -> Frame {
    let camera: Camera = Camera::new(CameraConfig::default());

    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / camera.aspect_ratio()) as u32;

    let frame = RayTracer::render(
        image_width,
        image_height,
        camera,
        complex_not_random_scene(),
    );

    frame.lock().unwrap().borrow().write_to_console();
    let mut frame_clone: Frame = Frame::new(image_width, image_height);

    for i in 0..image_width {
        for j in 0..image_height {
            frame_clone.set_color(i, j, frame.lock().unwrap().borrow().get_color(i, j));
        }
    }

    frame_clone
}
