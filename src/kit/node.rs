// Constraints based layouting
// TODO: might do flexbox

use std::fmt::Debug;

use windows::{Foundation::Rect, UI::Composition::Visual};
use windows_numerics::Vector2;

use crate::kit::math::Constraints;

pub trait Node: Debug {
    // size
    fn measure(&mut self, constraints: Constraints) -> Vector2;
    fn place(&mut self, rect: Rect);
    fn get_visual(&self) -> Visual;
}
