use ray_tracer::{Image, Camera, Scene, Element, ElementId};
use yewdux::store::Store;

#[derive(Default, Store)]
pub struct FrameStore {
    pub(super) frame: Option<Image>,
}

impl PartialEq for FrameStore {
    fn eq(&self, _: &Self) -> bool { false }
}

#[derive(Default, Store)]
pub struct CameraStore {
    pub(super) camera: Camera 
}

impl PartialEq for CameraStore {
    fn eq(&self, _: &Self) -> bool { false }
}



pub enum CanvasClickState {
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
    pub(super) click_state: CanvasClickState
}

impl PartialEq for CanvasClickStore {
    fn eq(&self, _: &Self) -> bool { false }
}

#[derive(Store)]
pub struct SceneStore {
    pub(super) scene: Scene,
    // selected element
    pub(super) element_id: Option<ElementId>
}

impl Default for SceneStore {
    fn default() -> Self {
        Self { scene: Default::default(), element_id: None }
    }
}

impl PartialEq for SceneStore {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
