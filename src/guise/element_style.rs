use std::sync::Arc;

use anyhow::anyhow;
use bevy::{
    asset::LoadContext,
    prelude::{Color, Handle, Image},
    reflect::Reflect,
    ui,
};

use super::{
    computed::ComputedStyle,
    from_ast::{FromAst, ReflectFromAst},
    typed_expr::TypedExpr,
    Expr,
};

#[derive(Debug, Clone)]
enum ElementStyleAttr {
    BackgroundImage(Option<Handle<Image>>),
    BackgroundColor(TypedExpr<Option<Color>>),
    BorderColor(TypedExpr<Option<Color>>),
    Color(TypedExpr<Option<Color>>),

    ZIndex(TypedExpr<i32>),

    Display(TypedExpr<ui::Display>),
    Position(TypedExpr<ui::PositionType>),
    Overflow(TypedExpr<ui::OverflowAxis>),
    OverflowX(TypedExpr<ui::OverflowAxis>),
    OverflowY(TypedExpr<ui::OverflowAxis>),
    Direction(TypedExpr<ui::Direction>),

    Left(TypedExpr<ui::Val>),
    Right(TypedExpr<ui::Val>),
    Top(TypedExpr<ui::Val>),
    Bottom(TypedExpr<ui::Val>),

    Width(TypedExpr<ui::Val>),
    Height(TypedExpr<ui::Val>),
    MinWidth(TypedExpr<ui::Val>),
    MinHeight(TypedExpr<ui::Val>),
    MaxWidth(TypedExpr<ui::Val>),
    MaxHeight(TypedExpr<ui::Val>),
    // // pub aspect_ratio: StyleProp<f32>,

    // Allow margin sides to be set individually
    Margin(TypedExpr<ui::UiRect>),
    MarginLeft(TypedExpr<ui::Val>),
    MarginRight(TypedExpr<ui::Val>),
    MarginTop(TypedExpr<ui::Val>),
    MarginBottom(TypedExpr<ui::Val>),

    Padding(TypedExpr<ui::UiRect>),
    PaddingLeft(TypedExpr<ui::Val>),
    PaddingRight(TypedExpr<ui::Val>),
    PaddingTop(TypedExpr<ui::Val>),
    PaddingBottom(TypedExpr<ui::Val>),

    Border(TypedExpr<ui::UiRect>),
    BorderLeft(TypedExpr<ui::Val>),
    BorderRight(TypedExpr<ui::Val>),
    BorderTop(TypedExpr<ui::Val>),
    BorderBottom(TypedExpr<ui::Val>),

    FlexDirection(TypedExpr<ui::FlexDirection>),
    FlexWrap(TypedExpr<ui::FlexWrap>),
    // Flex(ExprList),
    FlexGrow(TypedExpr<f32>),
    FlexShrink(TypedExpr<f32>),
    FlexBasis(TypedExpr<ui::Val>),
    RowGap(TypedExpr<ui::Val>),
    ColumnGap(TypedExpr<ui::Val>),
    Gap(TypedExpr<ui::Val>),

    AlignItems(TypedExpr<ui::AlignItems>),
    AlignSelf(TypedExpr<ui::AlignSelf>),
    AlignContent(TypedExpr<ui::AlignContent>),
    JustifyItems(TypedExpr<ui::JustifyItems>),
    JustifySelf(TypedExpr<ui::JustifySelf>),
    JustifyContent(TypedExpr<ui::JustifyContent>),
    // // TODO:
    // GridAutoFlow(bevy::ui::GridAutoFlow),
    // // pub grid_template_rows: Option<Vec<RepeatedGridTrack>>,
    // // pub grid_template_columns: Option<Vec<RepeatedGridTrack>>,
    // // pub grid_auto_rows: Option<Vec<GridTrack>>,
    // // pub grid_auto_columns: Option<Vec<GridTrack>>,
    // GridRow(bevy::ui::GridPlacement),
    // GridRowStart(TypedExpr<i16>),
    // GridRowSpan(TypedExpr<u16>),
    // GridRowEnd(i16),
    // GridColumn(bevy::ui::GridPlacement),
    // GridColumnStart(i16),
    // GridColumnSpan(u16),
    // GridColumnEnd(i16),

    // LineBreak(BreakLineOn),
}

/// A collection of style attributes which can be merged to create a `ComputedStyle`.
#[derive(Debug, Default, Clone, Reflect)]
#[type_path = "panoply::guise::ElementStyle"]
#[reflect(FromAst)]
pub struct ElementStyle {
    /// List of style attributes.
    /// Rather than storing the attributes in a struct full of optional fields, we store a flat
    /// vector of enums, each of which stores a single style attribute. This "sparse" representation
    /// allows for fast merging of styles, particularly for styles which have few or no attributes.
    #[reflect(ignore)]
    attrs: Vec<ElementStyleAttr>,
    // /// List of style variables to define when this style is invoked.
    // #[reflect(ignore)]
    // vars: VarsMap,

    // /// List of conditional styles
    // #[reflect(ignore)]
    // selectors: SelectorsMap,
}

impl ElementStyle {
    pub fn new() -> Self {
        Self {
            attrs: Vec::new(),
            // vars: VarsMap::new(),
            // selectors: SelectorsMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            attrs: Vec::with_capacity(capacity),
            // vars: VarsMap::new(),
            // selectors: SelectorsMap::new(),
        }
    }

    // / Construct a new `StyleMap` from a list of `StyleAttr`s.
    // pub fn from_attrs(attrs: &[StyleAttr]) -> Self {
    //     Self {
    //         attrs: Vec::from(attrs),
    //         vars: VarsMap::new(),
    //         selectors: SelectorsMap::new(),
    //     }
    // }

    /// Merge the style properties into a computed `Style` object.
    pub fn apply_to(&self, computed: &mut ComputedStyle) {
        self.apply_attrs_to(&self.attrs, computed);
        // TODO: Check selectors
    }

    fn apply_attrs_to(&self, attrs: &Vec<ElementStyleAttr>, computed: &mut ComputedStyle) {
        for attr in attrs.iter() {
            match attr {
                ElementStyleAttr::BackgroundImage(image) => {
                    computed.image = image.clone();
                }
                ElementStyleAttr::BackgroundColor(expr) => {
                    if let Ok(color) = expr.eval() {
                        computed.background_color = *color;
                    }
                }
                ElementStyleAttr::BorderColor(expr) => {
                    if let Ok(color) = expr.eval() {
                        computed.border_color = *color;
                    }
                }
                ElementStyleAttr::Color(expr) => {
                    if let Ok(color) = expr.eval() {
                        computed.color = *color;
                    }
                }
                ElementStyleAttr::ZIndex(expr) => {
                    if let Ok(val) = expr.eval() {
                        computed.z_index = Some(*val);
                    }
                }
                ElementStyleAttr::Display(expr) => {
                    if let Ok(disp) = expr.eval() {
                        computed.style.display = *disp;
                    }
                }
                ElementStyleAttr::Position(expr) => {
                    if let Ok(pos) = expr.eval() {
                        computed.style.position_type = *pos;
                    }
                }
                ElementStyleAttr::OverflowX(expr) => {
                    if let Ok(ov) = expr.eval() {
                        computed.style.overflow.x = *ov;
                    }
                }
                ElementStyleAttr::OverflowY(expr) => {
                    if let Ok(ov) = expr.eval() {
                        computed.style.overflow.y = *ov;
                    }
                }
                ElementStyleAttr::Overflow(expr) => {
                    if let Ok(ov) = expr.eval() {
                        computed.style.overflow.x = *ov;
                        computed.style.overflow.y = *ov;
                    }
                }
                ElementStyleAttr::Direction(expr) => {
                    if let Ok(dir) = expr.eval() {
                        computed.style.direction = *dir;
                    }
                }
                ElementStyleAttr::Left(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.left = *length;
                    }
                }
                ElementStyleAttr::Right(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.right = *length;
                    }
                }
                ElementStyleAttr::Top(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.top = *length;
                    }
                }
                ElementStyleAttr::Bottom(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.bottom = *length;
                    }
                }
                ElementStyleAttr::Width(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.width = *length;
                    }
                }
                ElementStyleAttr::Height(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.height = *length;
                    }
                }
                ElementStyleAttr::MinWidth(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.min_width = *length;
                    }
                }
                ElementStyleAttr::MinHeight(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.min_height = *length;
                    }
                }
                ElementStyleAttr::MaxWidth(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.max_width = *length;
                    }
                }
                ElementStyleAttr::MaxHeight(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.max_height = *length;
                    }
                }
                ElementStyleAttr::Margin(expr) => {
                    if let Ok(rect) = expr.eval() {
                        computed.style.margin = *rect;
                    }
                }
                ElementStyleAttr::MarginLeft(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.margin.left = *length;
                    }
                }
                ElementStyleAttr::MarginRight(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.margin.right = *length;
                    }
                }
                ElementStyleAttr::MarginTop(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.margin.top = *length;
                    }
                }
                ElementStyleAttr::MarginBottom(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.margin.bottom = *length;
                    }
                }
                ElementStyleAttr::Padding(expr) => {
                    if let Ok(rect) = expr.eval() {
                        computed.style.padding = *rect;
                    }
                }
                ElementStyleAttr::PaddingLeft(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.padding.left = *length;
                    }
                }
                ElementStyleAttr::PaddingRight(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.padding.right = *length;
                    }
                }
                ElementStyleAttr::PaddingTop(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.padding.top = *length;
                    }
                }
                ElementStyleAttr::PaddingBottom(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.padding.bottom = *length;
                    }
                }
                ElementStyleAttr::Border(expr) => {
                    if let Ok(rect) = expr.eval() {
                        computed.style.border = *rect;
                    }
                }
                ElementStyleAttr::BorderLeft(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.border.left = *length;
                    }
                }
                ElementStyleAttr::BorderRight(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.border.right = *length;
                    }
                }
                ElementStyleAttr::BorderTop(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.border.top = *length;
                    }
                }
                ElementStyleAttr::BorderBottom(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.border.bottom = *length;
                    }
                }
                ElementStyleAttr::FlexDirection(expr) => {
                    if let Ok(dir) = expr.eval() {
                        computed.style.flex_direction = *dir;
                    }
                }
                ElementStyleAttr::FlexWrap(expr) => {
                    if let Ok(wrap) = expr.eval() {
                        computed.style.flex_wrap = *wrap;
                    }
                }
                ElementStyleAttr::FlexGrow(expr) => {
                    if let Ok(amt) = expr.eval() {
                        computed.style.flex_grow = *amt;
                    }
                }
                ElementStyleAttr::FlexShrink(expr) => {
                    if let Ok(amt) = expr.eval() {
                        computed.style.flex_shrink = *amt;
                    }
                }
                ElementStyleAttr::FlexBasis(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.flex_basis = *length;
                    }
                }
                ElementStyleAttr::ColumnGap(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.column_gap = *length;
                    }
                }
                ElementStyleAttr::RowGap(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.row_gap = *length;
                    }
                }
                ElementStyleAttr::Gap(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.column_gap = *length;
                        computed.style.row_gap = *length;
                    }
                }

                ElementStyleAttr::AlignItems(expr) => {
                    if let Ok(align) = expr.eval() {
                        computed.style.align_items = *align;
                    }
                }
                ElementStyleAttr::AlignSelf(expr) => {
                    if let Ok(align) = expr.eval() {
                        computed.style.align_self = *align;
                    }
                }
                ElementStyleAttr::AlignContent(expr) => {
                    if let Ok(align) = expr.eval() {
                        computed.style.align_content = *align;
                    }
                }
                ElementStyleAttr::JustifyItems(expr) => {
                    if let Ok(justify) = expr.eval() {
                        computed.style.justify_items = *justify;
                    }
                }
                ElementStyleAttr::JustifySelf(expr) => {
                    if let Ok(justify) = expr.eval() {
                        computed.style.justify_self = *justify;
                    }
                }
                ElementStyleAttr::JustifyContent(expr) => {
                    if let Ok(justify) = expr.eval() {
                        computed.style.justify_content = *justify;
                    }
                }
            }
        }
    }
}

impl FromAst for ElementStyle {
    fn from_ast<'a>(
        members: bevy::utils::HashMap<String, super::expr::Expr>,
        load_context: &'a mut LoadContext,
    ) -> Result<Expr, anyhow::Error> {
        type A = ElementStyleAttr;
        let mut attrs = Vec::with_capacity(members.len());
        for (key, value) in members.iter() {
            match key.as_str() {
                "background_image" => match value {
                    Expr::AssetPath(path) => {
                        attrs.push(A::BackgroundImage(Some(load_context.load(path))));
                    }
                    Expr::Null => {
                        attrs.push(A::BackgroundImage(None));
                    }
                    Expr::Ident(id) if id == "none" => {
                        attrs.push(A::BackgroundImage(None));
                    }
                    _ => {
                        // TODO: Expressions
                        panic!("Invalid background image: {}", value)
                    }
                },
                "background_color" => attrs.push(A::BackgroundColor(TypedExpr::from_expr(value))),
                "border_color" => attrs.push(A::BorderColor(TypedExpr::from_expr(value))),
                "color" => attrs.push(A::Color(TypedExpr::from_expr(value))),

                "z_index" => attrs.push(A::ZIndex(TypedExpr::from_expr(value))),

                "display" => attrs.push(A::Display(TypedExpr::from_expr(value))),
                "position" => attrs.push(A::Position(TypedExpr::from_expr(value))),
                "overflow_x" => attrs.push(A::OverflowX(TypedExpr::from_expr(value))),
                "overflow_y" => attrs.push(A::OverflowY(TypedExpr::from_expr(value))),
                "overflow" => attrs.push(A::Overflow(TypedExpr::from_expr(value))),
                "direction" => attrs.push(A::Direction(TypedExpr::from_expr(value))),

                "left" => attrs.push(A::Left(TypedExpr::from_expr(value))),
                "right" => attrs.push(A::Right(TypedExpr::from_expr(value))),
                "top" => attrs.push(A::Top(TypedExpr::from_expr(value))),
                "bottom" => attrs.push(A::Bottom(TypedExpr::from_expr(value))),

                "width" => attrs.push(A::Width(TypedExpr::from_expr(value))),
                "height" => attrs.push(A::Height(TypedExpr::from_expr(value))),
                "min_width" => attrs.push(A::MinWidth(TypedExpr::from_expr(value))),
                "min_height" => attrs.push(A::MinHeight(TypedExpr::from_expr(value))),
                "max_width" => attrs.push(A::MaxWidth(TypedExpr::from_expr(value))),
                "max_height" => attrs.push(A::MaxHeight(TypedExpr::from_expr(value))),

                "margin" => attrs.push(A::Margin(TypedExpr::from_expr(value))),
                "margin_left" => attrs.push(A::MarginLeft(TypedExpr::from_expr(value))),
                "margin_right" => attrs.push(A::MarginRight(TypedExpr::from_expr(value))),
                "margin_top" => attrs.push(A::MarginTop(TypedExpr::from_expr(value))),
                "margin_bottom" => attrs.push(A::MarginBottom(TypedExpr::from_expr(value))),

                "padding" => attrs.push(A::Padding(TypedExpr::from_expr(value))),
                "padding_left" => attrs.push(A::PaddingLeft(TypedExpr::from_expr(value))),
                "padding_right" => attrs.push(A::PaddingRight(TypedExpr::from_expr(value))),
                "padding_top" => attrs.push(A::PaddingTop(TypedExpr::from_expr(value))),
                "padding_bottom" => attrs.push(A::PaddingBottom(TypedExpr::from_expr(value))),

                "border" => attrs.push(A::Border(TypedExpr::from_expr(value))),
                "border_left" => attrs.push(A::BorderLeft(TypedExpr::from_expr(value))),
                "border_right" => attrs.push(A::BorderRight(TypedExpr::from_expr(value))),
                "border_top" => attrs.push(A::BorderTop(TypedExpr::from_expr(value))),
                "border_bottom" => attrs.push(A::BorderBottom(TypedExpr::from_expr(value))),

                "flex_direction" => attrs.push(A::FlexDirection(TypedExpr::from_expr(value))),
                "flex_wrap" => attrs.push(A::FlexWrap(TypedExpr::from_expr(value))),
                "flex_grow" => attrs.push(A::FlexGrow(TypedExpr::from_expr(value))),
                "flex_shrink" => attrs.push(A::FlexShrink(TypedExpr::from_expr(value))),
                "flex_basis" => attrs.push(A::FlexBasis(TypedExpr::from_expr(value))),

                "row_gap" => attrs.push(A::RowGap(TypedExpr::from_expr(value))),
                "column_gap" => attrs.push(A::ColumnGap(TypedExpr::from_expr(value))),
                "gap" => attrs.push(A::Gap(TypedExpr::from_expr(value))),

                "align_items" => attrs.push(A::AlignItems(TypedExpr::from_expr(value))),
                "align_self" => attrs.push(A::AlignSelf(TypedExpr::from_expr(value))),
                "align_content" => attrs.push(A::AlignContent(TypedExpr::from_expr(value))),
                "justify_items" => attrs.push(A::JustifyItems(TypedExpr::from_expr(value))),
                "justify_self" => attrs.push(A::JustifySelf(TypedExpr::from_expr(value))),
                "justify_content" => attrs.push(A::JustifyContent(TypedExpr::from_expr(value))),

                _ => return Err(anyhow!("Invalid property: '{}'", key)),
            }
            // println!("{}: {}", key, value);
        }
        Ok(Expr::Style(Arc::new(Self { attrs })))
    }
}
