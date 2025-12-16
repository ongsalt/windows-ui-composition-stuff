use windows::{Foundation::Rect, UI::Composition::*};
use windows::UI::*;
use windows_numerics::Vector2;
use windows::core::Interface;

use crate::kit::math::{rect_offset, rect_size};
use crate::kit::{math::{Constraints, Margin, SizePreference}, node::Node, renderer::RenderContext};


pub struct ImageNode {}

// Should offer most functionality of Windows Compositor
// Need to know relative position
// Until we have better name
#[derive(Debug)]
pub struct DivNode {
    id: Option<String>,
    visual: LayerVisual,
    bg_geometry: CompositionRoundedRectangleGeometry,
    bg_rect_obj: CompositionSpriteShape,

    // actual rect
    prefered_w: SizePreference,
    prefered_h: SizePreference,

    margin: Margin,

    // Preferences
    corner_radius: f32,
    background_color: Color,
    opacity: f32,
    clip: bool,

    border_width: f32,
    border_color: Color,

    // shadow_opacity: f32,

    // Container layouting
    children: Vec<Box<dyn Node>>,
    measured_children_sizes: Vec<Vector2>,
}

impl DivNode {
    pub fn new(ctx: &mut RenderContext) -> Self {
        let visual = ctx.compositor.CreateLayerVisual().unwrap();
        let bg_visual = ctx.compositor.CreateShapeVisual().unwrap();

        let bg_geometry = ctx.compositor.CreateRoundedRectangleGeometry().unwrap();

        let bg_rect_obj = ctx
            .compositor
            .CreateSpriteShapeWithGeometry(&bg_geometry)
            .unwrap();
        let background_color = Colors::Transparent().unwrap();

        let brush = ctx
            .compositor
            .CreateColorBrushWithColor(background_color)
            .unwrap();

        bg_rect_obj.SetFillBrush(&brush).unwrap();

        // bg_visual.SetBrush(&brush).unwrap();
        bg_visual.Shapes().unwrap().Append(&bg_rect_obj).unwrap();
        bg_visual.SetRelativeSizeAdjustment(Vector2::one()).unwrap();

        visual.Children().unwrap().InsertAtTop(&bg_visual).unwrap();

        Self {
            id: None,
            visual,
            bg_geometry,
            bg_rect_obj,
            prefered_w: SizePreference::Default,
            prefered_h: SizePreference::Default,
            margin: Margin::zero(),
            background_color,
            corner_radius: 0.0,
            children: vec![],
            measured_children_sizes: vec![],
            border_color: Colors::Red().unwrap(),
            border_width: 0.0,
            clip: true,
            opacity: 1.0,
        }
    }

    // TODO: implement all of SwayRenderer method
    pub fn add_children<N: Node + 'static>(&mut self, node: N) {
        self.visual
            .Children()
            .unwrap()
            .InsertAtTop(&node.get_visual())
            .unwrap();
        self.children.push(Box::new(node));
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

    pub fn corner_radius(&self) -> f32 {
        self.corner_radius
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    pub fn clip(&self) -> bool {
        self.clip
    }

    pub fn border_width(&self) -> f32 {
        self.border_width
    }

    pub fn border_color(&self) -> Color {
        self.border_color
    }

    pub fn margin(&self) -> Margin {
        self.margin
    }

    pub fn set_margin(&mut self, margin: Margin) {
        self.margin = margin
    }

    pub fn set_corner_radius(&mut self, corner_radius: f32) {
        self.corner_radius = corner_radius;
        // try updating background visual corner radius if supported
        self.bg_geometry
            .SetCornerRadius(Vector2::new(corner_radius, corner_radius))
            .unwrap();
    }

    pub fn set_background_color(&mut self, background_color: Color, ctx: &mut RenderContext) {
        self.background_color = background_color;
        // create and apply a color brush to the shape visual
        if let Ok(brush) = ctx.compositor.CreateColorBrushWithColor(background_color) {
            self.set_background_brush(brush.cast().unwrap());
        }
    }

    pub fn set_background_brush(&mut self, brush: CompositionBrush) {
        let _ = self.bg_rect_obj.SetFillBrush(&brush);
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
        let _ = self.visual.SetOpacity(opacity as f32);
    }

    pub fn set_clip(&mut self, clip: bool) {
        self.clip = clip;
        todo!()
    }

    pub fn set_border_width(&mut self, border_width: f32) {
        self.border_width = border_width;
        let _ = self.bg_rect_obj.SetStrokeThickness(border_width);
    }

    pub fn set_border_color(&mut self, border_color: Color, ctx: &mut RenderContext) {
        self.border_color = border_color;
        if let Ok(brush) = ctx.compositor.CreateColorBrushWithColor(border_color) {
            self.set_border_brush(brush.cast().unwrap())
        }
    }

    pub fn set_border_brush(&mut self, brush: CompositionBrush) {
        let _ = self.bg_rect_obj.SetStrokeBrush(&brush);
    }

    pub fn prefered_h(&self) -> SizePreference {
        self.prefered_h
    }

    pub fn prefered_w(&self) -> SizePreference {
        self.prefered_w
    }

    pub fn set_prefered_h(&mut self, prefered_h: SizePreference) {
        self.prefered_h = prefered_h;
        // TODO: mark relayout
    }

    pub fn set_prefered_w(&mut self, prefered_w: SizePreference) {
        self.prefered_w = prefered_w;
        // TODO: mark relayout
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
}

impl Node for DivNode {
    fn measure(&mut self, constraints: Constraints) -> Vector2 {
        self.measured_children_sizes.clear();

        // TODO: sizing mode
        let mut w = 0.0;
        let mut h = 0.0;

        let children_constraints = Constraints {
            min_h: constraints.min_h,
            min_w: constraints.min_w,
            max_h: constraints.max_h - self.margin.h(),
            max_w: constraints.max_w - self.margin.w(),
        };
        // just assume its h stack
        for c in &mut self.children {
            // well, we should decrease y too
            let size = c.measure(children_constraints);
            // TODO: cache this size
            if size.X > w {
                w = size.X
            }
            h += size.Y;
            self.measured_children_sizes.push(size);
        }

        let w = match self.prefered_w {
            SizePreference::Default => w + 2. * self.border_width + self.margin.w(),
            SizePreference::FillAvailable => constraints.max_w,
            SizePreference::Fixed(fixed) => fixed + 2. * self.border_width + self.margin.w(),
        };

        let h = match self.prefered_h {
            SizePreference::Default => h + 2. * self.border_width + self.margin.h(),
            SizePreference::FillAvailable => constraints.max_h,
            SizePreference::Fixed(fixed) => fixed + 2. * self.border_width + self.margin.h(),
        };

        let s = constraints.coerce(Vector2::new(w, h));

        if let Some(id) = &self.id {
            println!("[{:.?}] measure: constraints={:.?}", id, constraints);
            println!("[{:.?}] measure: size={:.?}", id, s);
        };
        // println!("s: {:.?}", s);
        s
    }

    // rect is relative to parent
    fn place(&mut self, rect: Rect) {
        let size = rect_size(&rect);
        let actual_size = Vector2::new(
            size.X - self.border_width - self.margin.w(),
            size.Y - self.border_width - self.margin.h(),
        );
        if let Some(id) = &self.id {
            println!("[{:.?}] place: rect={:.?}", id, rect);
            println!("[{:.?}] place: actual_size={:.?}", id, actual_size);
        }
        self.bg_geometry.SetSize(actual_size).unwrap();
        self.bg_geometry
            .SetOffset(Vector2::new(
                self.border_width / 2.0 + self.margin.left,
                self.border_width / 2.0 + self.margin.top,
            ))
            .unwrap();

        // println!("rect size: {:.?}", rect_size(&rect));
        self.visual.SetSize(size).unwrap();
        self.visual.SetOffset(rect_offset(&rect)).unwrap();

        let mut y = self.margin.top + self.border_width;
        let x = self.margin.left + self.border_width;
        for (node, size) in self.children.iter_mut().zip(&self.measured_children_sizes) {
            // Calculate child bounding box
            // but this is z stack tho
            let rect = Rect {
                X: x,
                Y: y,
                Width: size.X,
                Height: size.Y,
            };
            y += size.Y;
            node.place(rect);
        }
    }

    fn get_visual(&self) -> Visual {
        self.visual.cast().unwrap()
    }
}
