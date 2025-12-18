use taffy::{Dimension, Style};
use windows::UI::{
    Color,
    Composition::{CompositionBrush, Visual},
};

// we need to put the entire fucking taffy::Style here
pub enum Attribute {
    BackgroundColor(Color),
    BorderColor(Color),
    BorderRadius(Dimension),
    BorderWidth(Dimension),
    Width(Dimension),
    Height(Dimension),

    BackdropFilter(CompositionBrush),

    // Updater
    Visual(Box<dyn FnOnce(Visual)>),
}

impl Attribute {
    pub fn is_taffy_style(&self) -> bool {
        false
    }

    pub fn patch_taffy_style(&self, style: &mut Style) {
        todo!()
    }
}
