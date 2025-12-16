use windows::{
    Foundation::Rect,
    UI::Composition::{Compositor, SpriteVisual, Visual},
};
use windows::core::Interface;
use windows_numerics::Vector2;

use crate::kit::{math::Constraints, node::Node};

#[derive(Debug)]
pub struct TextNode {
    visual: SpriteVisual,
    text: String,
}

// WrapperNode, node without its own visual

impl Node for TextNode {
    fn measure(&mut self, constraints: Constraints) -> Vector2 {
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
