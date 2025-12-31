// Constraints based layouting
// TODO: might do flexbox

use bitflags::bitflags;
use std::fmt::Debug;
use taffy::{AbsoluteAxis, Layout};
use windows::{
    UI::{
        Composition::{
            CompositionRoundedRectangleGeometry, CompositionSpriteShape, Compositor, ShapeVisual,
            SpriteVisual, Visual,
        },
        Input::GestureRecognizer,
    },
    core::Interface,
};
use windows_numerics::{Vector2, Vector3};

use crate::kit::debug::show_debug_info;

#[derive(Debug)]
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

        show_debug_info(&visual.cast().unwrap(), compositor);

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

    pub fn apply_layout(&mut self, layout: &Layout) -> Result<(), ()> {
        match self {
            Node::Div {
                visual,
                bg_visual,
                bg_shape,
            } => {
                // size not including padding because border-box
                visual
                    .SetSize(Vector2::new(
                        layout.size.width,
                        layout.size.height,
                    ))
                    .unwrap();
                bg_visual
                    .SetSize(Vector2::new(layout.size.width, layout.size.height))
                    .unwrap();

                // println!("{:.?}", layout);

                // Translate it by topleft margin
                // TODO: border

                visual
                    .SetOffset(Vector3::new(
                        layout.margin.left + layout.padding.left,
                        layout.margin.top + layout.padding.top,
                        0.0,
                    ))
                    .unwrap();

                // let compositor = visual.Compositor().unwrap();
                // let geometry = compositor.CreateRoundedRectangleGeometry().unwrap();
                // bg_shape.SetGeometry(&geometry);
            }
            Node::Leaf { visual } => {
                visual
                    .SetSize(Vector2::new(layout.size.width, layout.size.height))
                    .unwrap();
            }
        };

        Ok(())
    }
}
