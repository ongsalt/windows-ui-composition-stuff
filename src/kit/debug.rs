use windows::UI::Color;
use windows::UI::Composition::{Compositor, ContainerVisual, Visual};
use windows::core::Interface;
use windows_numerics::Vector2;

pub fn show_debug_info(visual: &Visual, compositor: &Compositor) {
    let debug_visual = {
        let v = compositor.CreateSpriteVisual().unwrap();
        let c = Color {
            R: 255,
            A: 50,
            B: 0,
            G: 0,
        };
        let b = compositor.CreateColorBrushWithColor(c).unwrap();
        v.SetBrush(&b).unwrap();
        v.SetRelativeSizeAdjustment(Vector2::one()).unwrap();
        v
    };
    if let Ok(visual) = visual.cast::<ContainerVisual>() {
        visual
            .Children()
            .unwrap()
            .InsertAtBottom(&debug_visual)
            .unwrap();
    } else {
        println!("Its not container visual")
    }
}
