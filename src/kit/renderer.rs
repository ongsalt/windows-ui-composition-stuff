use windows::{
    Foundation::Rect,
    UI::Composition::{Compositor, ContainerVisual, Desktop::DesktopWindowTarget, Visual},
    core::Interface,
};

use crate::{
    composition_host::CompositionHost,
    kit::{math::Constraints, node::Node},
};

pub struct Renderer {
    composition_host: CompositionHost,
    root: Node,
    w: f32,
    h: f32,
}

// we should actually pass RenderContext into Node instead of the actual compositor
// TODO: rename to tree root or someshi
impl Renderer {
    pub fn new(composition_host: CompositionHost, root: Node, size: (f32, f32)) -> Self {
        let container: ContainerVisual = composition_host.target.Root().unwrap().cast().unwrap();
        container
            .Children()
            .unwrap()
            .InsertAtTop(&root.visual())
            .unwrap();
        Self {
            composition_host,
            root,
            w: size.0,
            h: size.1,
        }
    }

    pub fn resize(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
        self.update();
    }

    pub fn update(&mut self) {

    }

    pub fn close(&mut self) {
        self.composition_host.close();
    }
}

#[derive(Debug)]
pub struct RenderContext {
    pub compositor: Compositor,
    // window?
}

impl RenderContext {
    pub fn new(compositor: &Compositor) -> Self {
        Self {
            compositor: compositor.clone(),
            // root_visual,
        }
    }

    // pub fn invalidate(&mut self, node: NodeId) {}
}
