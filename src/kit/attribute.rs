use taffy::prelude::*;
use taffy::geometry::Point;
use taffy::style::{
    Overflow, TextAlign, GridTemplateComponent, GridTemplateArea,
    TrackSizingFunction, GridAutoFlow, GridPlacement, LengthPercentage
};
use windows::UI::{
    Color,
    Composition::{CompositionBrush, Visual},
};

// we need to put the entire fucking taffy::Style here
pub enum Attribute {
    // Existing Visual/Color Attributes
    BackgroundColor(Color),
    BorderColor(Color),
    BorderRadius(Dimension),
    BorderWidth(LengthPercentage), // Changed from Dimension to match Style::border type
    Width(Dimension),
    Height(Dimension),

    // Updater
    BackdropFilter(CompositionBrush),
    Visual(Box<dyn FnOnce(Visual)>),

    // Taffy Style Fields
    Display(Display),
    ItemIsTable(bool),
    ItemIsReplaced(bool),
    BoxSizing(BoxSizing),
    Overflow(Point<Overflow>),
    ScrollbarWidth(f32),
    Position(Position),
    Inset(Rect<LengthPercentageAuto>),
    Size(Size<Dimension>),
    MinSize(Size<Dimension>),
    MaxSize(Size<Dimension>),
    // Flattened Mappings
    MinWidth(Dimension),
    MinHeight(Dimension),
    MaxWidth(Dimension),
    MaxHeight(Dimension),

    AspectRatio(Option<f32>),
    Margin(Rect<LengthPercentageAuto>),
    Padding(Rect<LengthPercentage>),
    Border(Rect<LengthPercentage>),
    AlignItems(Option<AlignItems>),
    AlignSelf(Option<AlignSelf>),
    JustifyItems(Option<AlignItems>),
    JustifySelf(Option<AlignSelf>),
    AlignContent(Option<AlignContent>),
    JustifyContent(Option<JustifyContent>),
    Gap(Size<LengthPercentage>),
    TextAlign(TextAlign),
    FlexDirection(FlexDirection),
    FlexWrap(FlexWrap),
    FlexBasis(Dimension),
    FlexGrow(f32),
    FlexShrink(f32),
    GridTemplateRows(Vec<GridTemplateComponent<String>>),
    GridTemplateColumns(Vec<GridTemplateComponent<String>>),
    GridAutoRows(Vec<TrackSizingFunction>),
    GridAutoColumns(Vec<TrackSizingFunction>),
    GridAutoFlow(GridAutoFlow),
    GridTemplateAreas(Vec<GridTemplateArea<String>>),
    // GridTemplateColumnNames(Vec<Vec<String>>),
    // GridTemplateRowNames(Vec<Vec<String>>),
    GridRow(Line<GridPlacement>),
    GridColumn(Line<GridPlacement>),
}

impl Attribute {
    pub fn is_taffy_style(&self) -> bool {
        matches!(
            self,
            Attribute::Display(_)
                | Attribute::ItemIsTable(_)
                | Attribute::ItemIsReplaced(_)
                | Attribute::BoxSizing(_)
                | Attribute::Overflow(_)
                | Attribute::ScrollbarWidth(_)
                | Attribute::Position(_)
                | Attribute::Inset(_)
                | Attribute::Size(_)
                | Attribute::MinSize(_)
                | Attribute::MaxSize(_)
                | Attribute::MinWidth(_)
                | Attribute::MinHeight(_)
                | Attribute::MaxWidth(_)
                | Attribute::MaxHeight(_)
                | Attribute::AspectRatio(_)
                | Attribute::Margin(_)
                | Attribute::Padding(_)
                | Attribute::Border(_)
                | Attribute::AlignItems(_)
                | Attribute::AlignSelf(_)
                | Attribute::JustifyItems(_)
                | Attribute::JustifySelf(_)
                | Attribute::AlignContent(_)
                | Attribute::JustifyContent(_)
                | Attribute::Gap(_)
                | Attribute::TextAlign(_)
                | Attribute::FlexDirection(_)
                | Attribute::FlexWrap(_)
                | Attribute::FlexBasis(_)
                | Attribute::FlexGrow(_)
                | Attribute::FlexShrink(_)
                | Attribute::GridTemplateRows(_)
                | Attribute::GridTemplateColumns(_)
                | Attribute::GridAutoRows(_)
                | Attribute::GridAutoColumns(_)
                | Attribute::GridAutoFlow(_)
                | Attribute::GridTemplateAreas(_)
                | Attribute::GridRow(_)
                | Attribute::GridColumn(_)
                | Attribute::Width(_)
                | Attribute::Height(_)
                | Attribute::BorderWidth(_)
        )
    }

    pub fn patch_taffy_style(&self, style: &mut Style) {
        match self {
            Attribute::Display(v) => style.display = *v,
            Attribute::ItemIsTable(v) => style.item_is_table = *v,
            Attribute::ItemIsReplaced(v) => style.item_is_replaced = *v,
            Attribute::BoxSizing(v) => style.box_sizing = *v,
            Attribute::Overflow(v) => style.overflow = *v,
            Attribute::ScrollbarWidth(v) => style.scrollbar_width = *v,
            Attribute::Position(v) => style.position = *v,
            Attribute::Inset(v) => style.inset = v.clone(),
            Attribute::Size(v) => style.size = v.clone(),
            Attribute::MinSize(v) => style.min_size = v.clone(),
            Attribute::MaxSize(v) => style.max_size = v.clone(),
            Attribute::MinWidth(v) => style.min_size.width = v.clone(),
            Attribute::MinHeight(v) => style.min_size.height = v.clone(),
            Attribute::MaxWidth(v) => style.max_size.width = v.clone(),
            Attribute::MaxHeight(v) => style.max_size.height = v.clone(),
            Attribute::AspectRatio(v) => style.aspect_ratio = *v,
            Attribute::Margin(v) => style.margin = v.clone(),
            Attribute::Padding(v) => style.padding = v.clone(),
            Attribute::Border(v) => style.border = v.clone(),
            Attribute::AlignItems(v) => style.align_items = *v,
            Attribute::AlignSelf(v) => style.align_self = *v,
            Attribute::JustifyItems(v) => style.justify_items = *v,
            Attribute::JustifySelf(v) => style.justify_self = *v,
            Attribute::AlignContent(v) => style.align_content = *v,
            Attribute::JustifyContent(v) => style.justify_content = *v,
            Attribute::Gap(v) => style.gap = v.clone(),
            Attribute::TextAlign(v) => style.text_align = *v,
            Attribute::FlexDirection(v) => style.flex_direction = *v,
            Attribute::FlexWrap(v) => style.flex_wrap = *v,
            Attribute::FlexBasis(v) => style.flex_basis = v.clone(),
            Attribute::FlexGrow(v) => style.flex_grow = *v,
            Attribute::FlexShrink(v) => style.flex_shrink = *v,
            Attribute::GridTemplateRows(v) => style.grid_template_rows = v.clone(),
            Attribute::GridTemplateColumns(v) => style.grid_template_columns = v.clone(),
            Attribute::GridAutoRows(v) => style.grid_auto_rows = v.clone(),
            Attribute::GridAutoColumns(v) => style.grid_auto_columns = v.clone(),
            Attribute::GridAutoFlow(v) => style.grid_auto_flow = *v,
            Attribute::GridTemplateAreas(v) => style.grid_template_areas = v.clone(),
            Attribute::GridRow(v) => style.grid_row = v.clone(),
            Attribute::GridColumn(v) => style.grid_column = v.clone(),
            Attribute::Width(d) => style.size.width = d.clone(),
            Attribute::Height(d) => style.size.height = d.clone(),
            // BorderWidth uses LengthPercentage directly now
            Attribute::BorderWidth(lp) => {
                 let val = lp.clone();
                 style.border = Rect { left: val.clone(), right: val.clone(), top: val.clone(), bottom: val };
            }

            _ => {}
        }
    }
}
