// Constraints based layouting
// TODO: might do flexbox

use std::fmt::Debug;
use std::ops::Deref;

use windows::core::Interface;
use windows::{
    Foundation::Rect,
    UI::{
        Color, Colors,
        Composition::{Compositor, IVisual, LayerVisual, SpriteVisual, Visual},
    },
};
use windows_numerics::Vector2;

use crate::kit::math::{rect_offset, rect_size};
use crate::kit::renderer::RenderContext;

#[derive(Debug, Clone, Copy)]
pub struct Contraints {
    pub max_w: f32,
    pub min_w: f32,
    pub max_h: f32,
    pub min_h: f32,
}

impl Contraints {
    pub fn from_size(size: Vector2) -> Self {
        Self {
            max_w: size.X,
            min_w: 0.0,
            max_h: size.Y,
            min_h: 0.0,
        }
    }
}

// know absolute position
pub trait Node: Debug {
    // size
    fn measure(&mut self, contraints: Contraints) -> Vector2;
    fn place(&mut self, rect: Rect);
    fn get_visual(&self) -> Visual;
}

#[derive(Debug)]
pub struct TextNode {
    visual: SpriteVisual,
    text: String,
}

// WrapperNode, node without its own visual

impl Node for TextNode {
    fn measure(&mut self, contraints: Contraints) -> Vector2 {
        todo!()
    }

    fn place(&mut self, rect: Rect) {
        todo!()
    }

    fn get_visual(&self) -> Visual {
        self.visual.cast().unwrap()
    }
}

impl TextNode {
    // We shuold defer layer creation to onPlace
    pub fn new(compositor: &mut Compositor, text: String) -> Self {
        let visual = compositor.CreateSpriteVisual().unwrap();
        // TODO: setup directwrite
        Self { text, visual }
    }
}

pub struct ImageNode {}

// Should offer most functionality of Windows Compositor
// Need to know relative position
// Until we have better name
#[derive(Debug)]
pub struct DivNode {
    visual: LayerVisual,
    bg_visual: SpriteVisual,

    // actual rect
    prefered_size: Vector2,
    offset: Vector2,

    // Preferences
    corner_radius: f32,
    background_color: Color,

    // Container layouting
    children: Vec<Box<dyn Node>>,
    measured_children_sizes: Vec<Vector2>,
}

impl DivNode {
    pub fn new(ctx: &mut RenderContext) -> Self {
        let visual = ctx.compositor.CreateLayerVisual().unwrap();
        let bg_visual = ctx.compositor.CreateSpriteVisual().unwrap();
        let background_color = Colors::Red().unwrap();
        // shuold this be shape visual tho

        let brush = ctx.compositor
            .CreateColorBrushWithColor(background_color)
            .unwrap();

        bg_visual.SetBrush(&brush).unwrap();
        bg_visual.SetRelativeSizeAdjustment(Vector2::one()).unwrap();

        visual.Children().unwrap().InsertAtTop(&bg_visual).unwrap();

        Self {
            visual,
            bg_visual,
            prefered_size: Vector2::zero(),
            offset: Vector2::zero(),
            background_color,
            corner_radius: 0.0,
            children: vec![],
            measured_children_sizes: vec![],
        }
    }

    // TODO: implement all sway renderer expected method
    pub fn add_children(&mut self, node: Box<dyn Node>) {
        self.visual
            .Children()
            .unwrap()
            .InsertAtTop(&node.get_visual())
            .unwrap();
        self.children.push(node);
    }

    pub fn remove_child(&mut self, index: usize) {
        // its will fucking panic if index is out of bound
        let node = self.children.remove(index);
        self.visual
            .Children()
            .unwrap()
            .Remove(&node.get_visual())
            .unwrap();
    }
}

impl Node for DivNode {
    fn measure(&mut self, contraints: Contraints) -> Vector2 {
        return Vector2::new(contraints.max_w, contraints.max_h);

        self.measured_children_sizes.clear();
        let mut w = 0.0;
        let mut h = 0.0;
        // just assume its z stack
        for c in &mut self.children {
            let size = c.measure(contraints);
            // TODO: cache this size
            if size.X > w {
                w = size.X
            }
            if size.Y > h {
                h = size.Y
            }
            self.measured_children_sizes.push(size);
        }

        Vector2::new(w, h)
    }

    // rect is relative to parent
    fn place(&mut self, rect: Rect) {
        // println!("rect size: {:.?}", rect_size(&rect));
        self.visual.SetSize(rect_size(&rect)).unwrap();
        self.visual.SetOffset(rect_offset(&rect)).unwrap();

        for (node, size) in self.children.iter_mut().zip(&self.measured_children_sizes) {
            // Calculate child bounding box
            // but this is z stack tho
            let rect = Rect {
                X: 0.0,
                Y: 0.0,
                Width: size.X,
                Height: size.Y,
            };
            node.place(rect);
        }
    }

    fn get_visual(&self) -> Visual {
        self.visual.cast().unwrap()
    }
}
