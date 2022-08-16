use js_sys::{Promise, Function};
use ray_tracer::{Scene, RayTracer, Image, Camera, WorkerPool, Vec3, Lambertian, Color, MaterialType, Element, ElementId};
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::{JsCast, JsValue};
use super::stores::*;
use crate::utils::{log, download_image};

use web_sys::console::log_1;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

use instant::Instant;

const ASPECT: f64 = 1.0;
const CANVAS_WIDTH: u32 = 600;
const CANVAS_HEIGHT: u32 = (CANVAS_WIDTH as f64 / ASPECT) as u32;


#[derive(Debug)]
pub enum RenderStatus {
    Complete(Instant, Instant),
    Rendering(Instant),
    Idle,
}


#[allow(dead_code)]
pub struct RayTracerDisplay {
    canvas_ref: NodeRef,
    canvas: Option<CanvasRenderingContext2d>,
    render_status: RenderStatus,

    frame_store: Rc<FrameStore>,
    camera_store: Rc<CameraStore>,
    scene_store: Rc<SceneStore>,

    frame_dispatch: Dispatch<FrameStore>,
    camera_dispatch: Dispatch<CameraStore>,
    canvas_dispatch: Dispatch<CanvasClickStore>,
    scene_dispatch: Dispatch<SceneStore>,
}

pub enum Signal {
    Render,
    RenderComplete,
    Download,
    UpdateFrame(Rc<FrameStore>),
    OnCameraUpdate(Rc<CameraStore>),
    OnCanvasClick(Rc<CanvasClickStore>),
    OnSceneUpdate(Rc<SceneStore>),
}

impl Component for RayTracerDisplay {
    type Message = Signal;
    type Properties = (); 

    fn create(ctx: &Context<Self>) -> Self {
        let frame_update_callback = ctx.link().callback(Signal::UpdateFrame);
        let camera_update_callback = ctx.link().callback(Signal::OnCameraUpdate);
        let canvas_update_callback = ctx.link().callback(Signal::OnCanvasClick);
        let scene_update_callback = ctx.link().callback(Signal::OnSceneUpdate);
        let frame_dispatch = Dispatch::<FrameStore>::subscribe(frame_update_callback);
        let camera_dispatch = Dispatch::<CameraStore>::subscribe(camera_update_callback);
        let canvas_dispatch = Dispatch::<CanvasClickStore>::subscribe(canvas_update_callback);
        let scene_dispatch = Dispatch::<SceneStore>::subscribe(scene_update_callback);


        Self {
            canvas_ref: NodeRef::default(),
            canvas: None,
            render_status: RenderStatus::Idle,

            frame_store: frame_dispatch.get(),
            camera_store: camera_dispatch.get(),
            scene_store: scene_dispatch.get(),

            frame_dispatch,
            camera_dispatch,
            canvas_dispatch,
            scene_dispatch
        }
    }

    fn update(&mut self, ctx: &Context<Self>, signal: Self::Message) -> bool {
        match signal {
            Signal::Render => {
                self.request_render();
            }
            Signal::RenderComplete => {
                if let RenderStatus::Rendering(start_time) = self.render_status {
                    self.render_status = RenderStatus::Complete(start_time, Instant::now());
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
            Signal::OnCanvasClick(canvas_store) => {
                if let CanvasClickState::Clicked(x, y) = canvas_store.click_state {
                    log("Picked".to_string());
                    self.pick_element(x, y);
                }
            }
            Signal::OnSceneUpdate(scene_store) => {
                self.scene_store = scene_store;
                ctx.link().send_message(Signal::Render) 
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

        let request_render = link.callback(|_| {
            Signal::Render
        });

        let increase_y = |_| {
            RayTracerDisplay::translate_camera(Vec3::new(0.0, 0.5, 0.0));
        };

        let decrease_y = |_: MouseEvent| {
            RayTracerDisplay::translate_camera(Vec3::new(0.0, -0.5, 0.0));
        };

        let increase_x = |_| {
            RayTracerDisplay::translate_camera(Vec3::new(0.5, 0.0, 0.0));
        };

        let decrease_x = |_: MouseEvent| {
            RayTracerDisplay::translate_camera(Vec3::new(-0.5, 0.0, 0.0));
        };

        let increase_z = |_| {
            RayTracerDisplay::translate_camera(Vec3::new(0.0, 0.0, 0.5));
        };

        let decrease_z = |_: MouseEvent| {
            RayTracerDisplay::translate_camera(Vec3::new(-0.5, 0.0, -0.5));
        };

        let c_dispatch = Dispatch::<CameraStore>::new();
        let camera = c_dispatch.get().camera; 

        let element_id = Dispatch::<SceneStore>::new().get().element_id;

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
                <div>
                    <ElementDisplay element_id={element_id} />
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
                })}
                </h5>
            </div>
        }
    }
}


impl RayTracerDisplay {
    fn translate_camera(translation: Vec3) {
        let dispatch = Dispatch::<CameraStore>::new();
        let mut camera: Camera = dispatch.get().camera; 

        camera.translate(translation);

        dispatch.set(CameraStore { camera }); 
    }
    fn initialize_canvas(&mut self) {
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

        canvas.set_height(CANVAS_HEIGHT);
        canvas.set_width(CANVAS_WIDTH);

        let on_canvas_click = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            log(format!("{} {}", event.offset_x(), event.offset_y()));
            let dispatch = Dispatch::<CanvasClickStore>::new();
            dispatch.set(CanvasClickStore {
                click_state: CanvasClickState::Clicked(event.offset_x(), event.offset_y())
            });
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


    fn request_render(&mut self) {
        self.render_status = RenderStatus::Rendering(Instant::now());

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        
        let button = document
            .get_element_by_id("create_frame_btn")
            .unwrap()
            .dyn_into::<web_sys::HtmlButtonElement>();
        let render_btn = button.unwrap();

        let btn_onclick = render_btn.onclick();
        if btn_onclick.is_some() {
            let onclick = btn_onclick.unwrap();
            let _res = onclick.call0(&JsValue::undefined());
        }
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

    fn download_render(&self) {
        match &self.frame_store.as_ref().frame {
            Some(_) => {
                let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                let canvas_image = canvas.to_data_url_with_type("image/png").unwrap();
                download_image(canvas_image)
            }
            None => {
                log::info!("There is no frame to download");
            }
        }
    }

    /// highlights the selected element in the scene
    /// TODO: this clones the scene - let's try and avoid that
    fn pick_element(&mut self, mouse_x: i32, mouse_y: i32) {
        let scene = self.scene_store.scene.clone(); 
        let camera = &self.camera_store.camera;

        let ray = camera.create_ray(mouse_x as f64 / CANVAS_WIDTH as f64, ((CANVAS_HEIGHT as f64) - mouse_y as f64) / CANVAS_HEIGHT as f64);

        if let Some(element) = RayTracer::compute_collision_element(&scene, ray) {
            let dispatch = Dispatch::<SceneStore>::new();
            dispatch.set(
                SceneStore { 
                    scene,
                    element_id: Some(element.id)
                }
            );
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
        let scene: Scene = Dispatch::<SceneStore>::new().get().scene.clone();
        RayTracer::render_scene_wasm(scene, camera, CANVAS_WIDTH, CANVAS_HEIGHT, pool)
    }

    /// display a serialized image 
    pub fn display_image(&self, image: &JsValue) {
        let dispatch = Dispatch::<FrameStore>::new();
        dispatch.set(FrameStore { frame: Some(image.into_serde().unwrap()) });
    }
}

#[derive(Properties)]
pub struct Props {
    element_id: Option<ElementId>
}

impl PartialEq for Props {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[function_component(ElementDisplay)]
fn element_display(props: &Props) -> Html {

    if props.element_id.is_none() {
        return html! {
            <h1>{"No element selected"}</h1>
        }
    }
    let scene = &Dispatch::<SceneStore>::new().get().scene;
    let element = scene.get_element(props.element_id.unwrap());
    let on_material_change = Callback::from(move |material: MaterialType| {

    });

    let id = element.id; 
    let mat = &element.material;
    let shape = &element.shape;


    html! {
        <div>
            <h4>{format!("{:?}", id)}</h4>
            <MaterialDisplay {on_material_change} material={mat.clone()} />
        </div>
    }
}

#[derive(Properties)]
pub struct MatProps {
    material: MaterialType,
    on_material_change: Callback<MaterialType>, 
}

impl PartialEq for MatProps {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[function_component(MaterialDisplay)]
fn material_display(props: &MatProps) -> Html {
    html! {
        <div>
        {
            match &props.material {
                MaterialType::Lambertian(m) => { 
                    html! {
                        <h5>{"Lambertian"}</h5>
                    } 
                },
                MaterialType::Dielectric(m) => { 
                    html! {
                        <h5>{"Dielectric"}</h5>

                    } 
                },
                MaterialType::Metal(m)      => { 
                    html! {
                        <h5>{"Metal"}</h5>
                    } 
                },
            }
        }
        </div>
    }
}
