use windows::{
    Foundation::Rect,
    UI::Composition::{Compositor, ContainerVisual, Desktop::DesktopWindowTarget, Visual},
    core::Interface,
};

use crate::{
    composition_host::CompositionHost,
    kit::{math::Constraints, node::Node, tree::NodeId},
};

pub struct Renderer {
    composition_host: CompositionHost,
    root: Box<dyn Node>,
    w: f32,
    h: f32,
}

// we should actually pass RenderContext into Node instead of the actual compositor
// TODO: rename to tree root or someshi
impl Renderer {
    pub fn new(composition_host: CompositionHost, root: Box<dyn Node>, size: (f32, f32)) -> Self {
        let container: ContainerVisual = composition_host.target.Root().unwrap().cast().unwrap();
        container
            .Children()
            .unwrap()
            .InsertAtTop(&root.get_visual())
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
        let size = self.root.measure(Constraints {
            min_h: 0.0,
            min_w: 0.0,
            max_h: self.h,
            max_w: self.w,
        });
        self.root.place(Rect {
            Width: size.X,
            Height: size.Y,
            X: 0.0,
            Y: 0.0,
        });
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

    pub fn invalidate(&mut self, node: NodeId) {
        
    }
}
