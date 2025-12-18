// Constraints based layouting
// TODO: might do flexbox

use bitflags::bitflags;
use std::fmt::Debug;
use windows::{
    UI::Composition::{
        CompositionRoundedRectangleGeometry, CompositionSpriteShape, Compositor, ShapeVisual,
        SpriteVisual, Visual,
    },
    core::Interface,
};
use windows_numerics::Vector2;

pub enum Node {
    Div {
        visual: SpriteVisual,
        bg_visual: ShapeVisual,
        bg_shape: CompositionSpriteShape,
    },
    Leaf {
        visual: Visual,
    },
}

impl Node {
    pub fn new_div(compositor: &Compositor) -> Self {
        // TODO: might use LayerVisual
        let visual = compositor.CreateSpriteVisual().unwrap();
        let bg_visual = compositor.CreateShapeVisual().unwrap();

        let bg_geometry = compositor.CreateRoundedRectangleGeometry().unwrap();

        let bg_shape = compositor
            .CreateSpriteShapeWithGeometry(&bg_geometry)
            .unwrap();

        bg_visual.Shapes().unwrap().Append(&bg_shape).unwrap();
        bg_visual.SetRelativeSizeAdjustment(Vector2::one()).unwrap();

        visual.Children().unwrap().InsertAtTop(&bg_visual).unwrap();

        Self::Div {
            visual,
            bg_visual,
            bg_shape,
        }
    }

    pub fn new_leaf(compositor: &Compositor) -> Self {
        Self::Leaf {
            visual: compositor.CreateSpriteVisual().unwrap().cast().unwrap(),
        }
    }

    pub fn visual(&self) -> Visual {
        match self {
            Node::Div { visual, .. } => visual.cast().unwrap(),
            Node::Leaf { visual } => visual.to_owned(),
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DirtyFlags: u32 {
        const LAYOUT  = 0b00000001;
    }
}
