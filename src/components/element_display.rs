use js_sys::Promise;
use ray_tracer::{Element, WorkerPool, Vec3, Camera, CameraConfig, ShapeType};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlButtonElement};
use yew::prelude::*;

use super::utils::get_element_ref;

#[derive(Properties)]
pub struct ElementDisplayProps {
    pub element: Element,
    pub on_update_element: Callback<Element>,
}

impl PartialEq for ElementDisplayProps {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

const PREVIEW_ASPECT: f64 = 16.0 / 9.0;
const PREVIEW_WIDTH: u32 = 400;
const PREVIEW_HEIGHT: u32 = (PREVIEW_WIDTH as f64 / PREVIEW_ASPECT) as u32;

#[function_component(ElementDisplay)]
pub fn element_display(props: &ElementDisplayProps) -> Html {
    // TODO: maybe Element should implement Copy?
    let element = props.element.clone();
    let canvas_ref = use_node_ref();
    let canvas_ctx = use_state(|| -> Option<CanvasRenderingContext2d> { None });

    // initialize canvas ref
    {
        let canvas_ref = canvas_ref.clone();
        let canvas_ctx = canvas_ctx.clone();

        use_effect_with_deps(move |canvas_ref| {
            let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            canvas.set_height(PREVIEW_HEIGHT);
            canvas.set_width(PREVIEW_WIDTH);

            canvas_ctx.set(Some(
                canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap()
            ));

            move || {}
        }, canvas_ref);
    }

    let on_render = |_| on_render_preview();

    let position = match element.shape {
        ShapeType::Sphere(s) => s.center()
    };

    html! {
        <div>
            <div>
                {format!("{:?}", element.id)}
            </div>
            
            <button class="render_preview w3-button w3-blue" onclick={on_render}>
                {"Render preview"}
            </button>
            <div class="w3-row-padding">
                <div class="w3-third">
                    <label>{"y"}</label>
                    <input class="w3-input w3-border" type="text" value={format!("{}", position.x)} />
                </div>
                <div class="w3-third">
                    <label>{"y"}</label>
                    <input class="w3-input w3-border" type="text" value={format!("{}", position.y)} />
                </div>
                <div class="w3-third">
                    <label>{"z"}</label>
                    <input class="w3-input w3-border" type="text" value={format!("{}", position.z)} />
                </div>
            </div>
            <canvas ref={canvas_ref.clone()} class="element_preview" />
        </div>
    }
}


// trigger onclick function defined for "element_preview" canvas, in index.js
fn on_render_preview() {
    let render_btn = get_element_ref::<HtmlButtonElement>("element_preview".into()).unwrap(); 

    if let Some(onclick) = render_btn.onclick() {
        let _ = onclick.call0(&JsValue::undefined());
    }
}

fn render_preview(pool: &WorkerPool) -> Result<Promise, JsValue> {
    let camera = Camera::new(
        CameraConfig {
            origin: Vec3::new(3.0, 1.0, 1.0),
            look_at: Vec3::new(0.5, 0.5, 0.0),
            ..CameraConfig::default()
        }
    );
    // let scene = 
        
    todo!();
}
