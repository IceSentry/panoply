use bevy::{prelude::Color, text::BreakLineOn, ui::*};
use lazy_static::lazy_static;
use quick_xml::events::BytesStart;
use regex::Regex;
use std::str::FromStr;

use crate::guise::GuiseError;

use super::{expr::Expr, ComputedStyle};

/** A single style-sheet property which can be applied to a computed style. */
#[derive(Debug, Clone, PartialEq)]
pub enum StyleAttr {
    BackgroundImage(Expr),
    BackgroundColor(Expr),
    BorderColor(Expr),
    Color(Expr),

    ZIndex(Expr),

    Display(Expr),
    Position(Expr),
    Overflow(Expr),
    OverflowX(Expr),
    OverflowY(Expr),
    Direction(Expr),

    Left(Expr),
    Right(Expr),
    Top(Expr),
    Bottom(Expr),

    Width(Expr),
    Height(Expr),
    MinWidth(Expr),
    MinHeight(Expr),
    MaxWidth(Expr),
    MaxHeight(Expr),

    // pub aspect_ratio: StyleProp<f32>,
    AlignItems(Expr),
    JustifyItems(Expr),
    AlignSelf(Expr),
    JustifySelf(Expr),
    AlignContent(Expr),
    JustifyContent(Expr),

    // Allow margin sides to be set individually
    Margin(Expr),
    MarginLeft(Expr),
    MarginRight(Expr),
    MarginTop(Expr),
    MarginBottom(Expr),

    Padding(Expr),
    PaddingLeft(Expr),
    PaddingRight(Expr),
    PaddingTop(Expr),
    PaddingBottom(Expr),

    Border(Expr),
    BorderLeft(Expr),
    BorderRight(Expr),
    BorderTop(Expr),
    BorderBottom(Expr),

    FlexDirection(bevy::ui::FlexDirection),
    FlexWrap(bevy::ui::FlexWrap),
    Flex(Expr),
    FlexGrow(Expr),
    FlexShrink(Expr),
    FlexBasis(Expr),

    RowGap(Expr),
    ColumnGap(Expr),
    Gap(Expr),

    // TODO:
    GridAutoFlow(bevy::ui::GridAutoFlow),
    // pub grid_template_rows: Option<Vec<RepeatedGridTrack>>,
    // pub grid_template_columns: Option<Vec<RepeatedGridTrack>>,
    // pub grid_auto_rows: Option<Vec<GridTrack>>,
    // pub grid_auto_columns: Option<Vec<GridTrack>>,
    GridRow(bevy::ui::GridPlacement),
    GridRowStart(i16),
    GridRowSpan(u16),
    GridRowEnd(i16),
    GridColumn(bevy::ui::GridPlacement),
    GridColumnStart(i16),
    GridColumnSpan(u16),
    GridColumnEnd(i16),

    LineBreak(BreakLineOn),
}

impl StyleAttr {
    /// Apply this style attribute to a computed style.
    pub fn apply(&self, computed: &mut ComputedStyle) {
        match self {
            StyleAttr::BackgroundImage(_asset) => {
                todo!("Implement background-image")
                // if let Some(c) = val.into_color() {
                //     computed.background_color = c;
                // }
            }
            StyleAttr::BackgroundColor(val) => {
                if let Some(c) = val.into_color() {
                    computed.background_color = c;
                }
            }
            StyleAttr::BorderColor(val) => {
                if let Some(c) = val.into_color() {
                    computed.border_color = c;
                }
            }
            StyleAttr::Color(val) => {
                if let Some(c) = val.into_color() {
                    computed.color = c;
                }
            }
            StyleAttr::ZIndex(val) => {
                if let Some(z) = val.into_i32() {
                    computed.z_index = Some(z);
                }
            }

            StyleAttr::Display(val) => {
                if let Some(d) = val.into_display() {
                    computed.style.display = d;
                }
            }
            StyleAttr::Position(val) => {
                if let Some(d) = val.into_position() {
                    computed.style.position_type = d;
                }
            }
            StyleAttr::Overflow(val) => {
                if let Some(d) = val.into_overflow() {
                    computed.style.overflow.x = d;
                    computed.style.overflow.y = d;
                }
            }
            StyleAttr::OverflowX(val) => {
                if let Some(d) = val.into_overflow() {
                    computed.style.overflow.x = d;
                }
            }
            StyleAttr::OverflowY(val) => {
                if let Some(d) = val.into_overflow() {
                    computed.style.overflow.y = d;
                }
            }
            StyleAttr::Direction(val) => {
                if let Some(d) = val.into_direction() {
                    computed.style.direction = d;
                }
            }

            StyleAttr::Left(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.left = l;
                }
            }
            StyleAttr::Right(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.right = l;
                }
            }
            StyleAttr::Top(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.top = l;
                }
            }
            StyleAttr::Bottom(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.bottom = l;
                }
            }

            StyleAttr::Width(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.width = l;
                }
            }
            StyleAttr::Height(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.height = l;
                }
            }
            StyleAttr::MinWidth(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.min_width = l;
                }
            }
            StyleAttr::MinHeight(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.min_height = l;
                }
            }
            StyleAttr::MaxWidth(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.max_width = l;
                }
            }
            StyleAttr::MaxHeight(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.max_height = l;
                }
            }

            StyleAttr::AlignItems(val) => {
                if let Some(l) = val.into_align_items() {
                    computed.style.align_items = l;
                }
            }
            StyleAttr::JustifyItems(val) => {
                if let Some(l) = val.into_justify_items() {
                    computed.style.justify_items = l;
                }
            }
            StyleAttr::AlignSelf(val) => {
                if let Some(l) = val.into_align_self() {
                    computed.style.align_self = l;
                }
            }
            StyleAttr::JustifySelf(val) => {
                if let Some(l) = val.into_justify_self() {
                    computed.style.justify_self = l;
                }
            }
            StyleAttr::AlignContent(val) => {
                if let Some(l) = val.into_align_content() {
                    computed.style.align_content = l;
                }
            }
            StyleAttr::JustifyContent(val) => {
                if let Some(l) = val.into_justify_content() {
                    computed.style.justify_content = l;
                }
            }

            StyleAttr::Margin(val) => {
                if let Some(r) = val.into_uirect() {
                    computed.style.margin = r;
                }
            }
            StyleAttr::MarginLeft(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.margin.left = l;
                }
            }
            StyleAttr::MarginRight(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.margin.right = l;
                }
            }
            StyleAttr::MarginTop(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.margin.top = l;
                }
            }
            StyleAttr::MarginBottom(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.margin.bottom = l;
                }
            }

            StyleAttr::Padding(val) => {
                if let Some(r) = val.into_uirect() {
                    computed.style.padding = r;
                }
            }
            StyleAttr::PaddingLeft(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.padding.left = l;
                }
            }
            StyleAttr::PaddingRight(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.padding.right = l;
                }
            }
            StyleAttr::PaddingTop(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.padding.top = l;
                }
            }
            StyleAttr::PaddingBottom(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.padding.bottom = l;
                }
            }

            StyleAttr::Border(val) => {
                if let Some(r) = val.into_uirect() {
                    computed.style.border = r;
                }
            }
            StyleAttr::BorderLeft(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.border.left = l;
                }
            }
            StyleAttr::BorderRight(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.border.right = l;
                }
            }
            StyleAttr::BorderTop(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.border.top = l;
                }
            }
            StyleAttr::BorderBottom(val) => {
                if let Some(l) = val.into_length() {
                    computed.style.border.bottom = l
                }
            }

            StyleAttr::FlexDirection(val) => {
                computed.style.flex_direction = *val;
            }
            StyleAttr::FlexWrap(val) => {
                computed.style.flex_wrap = *val;
            }
            StyleAttr::Flex(val) => match val {
                Expr::Number(flex) => {
                    computed.style.flex_grow = *flex;
                    computed.style.flex_shrink = *flex;
                    computed.style.flex_basis = Val::Auto;
                }
                Expr::List(items) => {
                    if items.len() == 3 {
                        match items[0] {
                            Expr::Number(n) => {
                                computed.style.flex_grow = n;
                            }
                            _ => (),
                        };
                        match items[1] {
                            Expr::Number(n) => {
                                computed.style.flex_shrink = n;
                            }
                            _ => (),
                        };
                        if let Some(basis) = items[3].into_length() {
                            computed.style.flex_basis = basis;
                        }
                    }
                }
                _ => (),
            },
            StyleAttr::FlexGrow(val) => {
                if let Some(flex) = val.into_f32() {
                    computed.style.flex_grow = flex;
                }
            }
            StyleAttr::FlexShrink(val) => {
                if let Some(flex) = val.into_f32() {
                    computed.style.flex_shrink = flex;
                }
            }
            StyleAttr::FlexBasis(val) => {
                if let Some(len) = val.into_length() {
                    computed.style.flex_basis = len;
                }
            }

            StyleAttr::RowGap(val) => {
                if let Some(len) = val.into_length() {
                    computed.style.row_gap = len;
                }
            }
            StyleAttr::ColumnGap(val) => {
                if let Some(len) = val.into_length() {
                    computed.style.column_gap = len;
                }
            }
            StyleAttr::Gap(val) => {
                if let Some(len) = val.into_length() {
                    computed.style.row_gap = len;
                    computed.style.column_gap = len;
                }
            }

            StyleAttr::GridAutoFlow(val) => {
                computed.style.grid_auto_flow = *val;
            }
            StyleAttr::GridRow(val) => {
                computed.style.grid_row = *val;
            }
            StyleAttr::GridRowStart(val) => {
                computed.style.grid_row.set_start(*val);
            }
            StyleAttr::GridRowSpan(val) => {
                computed.style.grid_row.set_span(*val);
            }
            StyleAttr::GridRowEnd(val) => {
                computed.style.grid_row.set_end(*val);
            }

            StyleAttr::GridColumn(val) => {
                computed.style.grid_column = *val;
            }
            StyleAttr::GridColumnStart(val) => {
                computed.style.grid_column.set_start(*val);
            }
            StyleAttr::GridColumnSpan(val) => {
                computed.style.grid_column.set_span(*val);
            }
            StyleAttr::GridColumnEnd(val) => {
                computed.style.grid_column.set_end(*val);
            }
            StyleAttr::LineBreak(val) => {
                computed.line_break = Some(*val);
            }
        }
    }

    /// Parse a `StyleAttr` from an XML attribute name/value pair.
    pub fn parse<'a>(name: &'a [u8], value: &str) -> Result<Option<Self>, GuiseError> {
        Ok(Some(match name {
            b"flex-direction" => StyleAttr::FlexDirection(match value {
                "row" => FlexDirection::Row,
                "column" => FlexDirection::Column,
                "row-reverse" => FlexDirection::RowReverse,
                "column-reverse" => FlexDirection::ColumnReverse,
                _ => {
                    return Err(GuiseError::UnknownAttributeValue(value.to_string()));
                }
            }),

            b"flex-wrap" => StyleAttr::FlexWrap(match value {
                "nowrap" => FlexWrap::NoWrap,
                "wrap" => FlexWrap::Wrap,
                "wrap-reverse" => FlexWrap::WrapReverse,
                _ => {
                    return Err(GuiseError::UnknownAttributeValue(value.to_string()));
                }
            }),

            b"grid-auto-flow" => StyleAttr::GridAutoFlow(match value {
                "row" => GridAutoFlow::Row,
                "column" => GridAutoFlow::Column,
                "row-dense" => GridAutoFlow::RowDense,
                "column-dense" => GridAutoFlow::ColumnDense,
                _ => {
                    return Err(GuiseError::UnknownAttributeValue(value.to_string()));
                }
            }),
            //     // TODO:
            //     // pub grid_template_rows: Option<Vec<RepeatedGridTrack>>,
            //     // pub grid_template_columns: Option<Vec<RepeatedGridTrack>>,
            //     // pub grid_auto_rows: Option<Vec<GridTrack>>,
            //     // pub grid_auto_columns: Option<Vec<GridTrack>>,
            b"grid-row" => StyleAttr::GridRow(StyleAttr::parse_grid_placement(value)?),
            b"grid-row-start" => StyleAttr::GridRowStart(StyleAttr::parse_i16(value)?),
            b"grid-row-span" => StyleAttr::GridRowSpan(StyleAttr::parse_u16(value)?),
            b"grid-row-end" => StyleAttr::GridRowEnd(StyleAttr::parse_i16(value)?),
            b"grid-column" => StyleAttr::GridColumn(StyleAttr::parse_grid_placement(value)?),
            b"grid-column-start" => StyleAttr::GridColumnStart(StyleAttr::parse_i16(value)?),
            b"grid-column-span" => StyleAttr::GridColumnSpan(StyleAttr::parse_u16(value)?),
            b"grid-column-end" => StyleAttr::GridColumnEnd(StyleAttr::parse_i16(value)?),

            b"line-break" => StyleAttr::LineBreak(match value {
                "nowrap" => bevy::text::BreakLineOn::NoWrap,
                "word" => bevy::text::BreakLineOn::WordBoundary,
                "char" => bevy::text::BreakLineOn::AnyCharacter,
                _ => {
                    return Err(GuiseError::UnknownAttributeValue(value.to_string()));
                }
            }),

            _ => return Ok(None),
        }))
    }

    pub fn write_xml(&self, elem: &mut BytesStart) {
        match self {
            StyleAttr::FlexDirection(dir) => {
                elem.push_attribute((
                    "flex-direction",
                    match dir {
                        bevy::ui::FlexDirection::Row => "row",
                        bevy::ui::FlexDirection::Column => "column",
                        bevy::ui::FlexDirection::RowReverse => "row-reverse",
                        bevy::ui::FlexDirection::ColumnReverse => "column-reverse",
                    },
                ));
            }

            StyleAttr::FlexWrap(dir) => {
                elem.push_attribute((
                    "flex-wrap",
                    match dir {
                        bevy::ui::FlexWrap::NoWrap => "nowrap",
                        bevy::ui::FlexWrap::Wrap => "wrap",
                        bevy::ui::FlexWrap::WrapReverse => "wrap-reverse",
                    },
                ));
            }

            StyleAttr::GridAutoFlow(val) => {
                elem.push_attribute((
                    "grid-auto-flow",
                    match val {
                        GridAutoFlow::Row => "row",
                        GridAutoFlow::Column => "column",
                        GridAutoFlow::RowDense => "row-dense",
                        GridAutoFlow::ColumnDense => "column-dense",
                    },
                ));
            }

            StyleAttr::GridRow(_) => {
                panic!("Unsupported, can't write GridPlacement");
            }
            StyleAttr::GridRowStart(val) => {
                elem.push_attribute(("grid-row-start", i16::to_string(val).as_str()));
            }
            StyleAttr::GridRowSpan(val) => {
                elem.push_attribute(("grid-row-span", u16::to_string(val).as_str()));
            }
            StyleAttr::GridRowEnd(val) => {
                elem.push_attribute(("grid-row-end", i16::to_string(val).as_str()));
            }

            StyleAttr::GridColumn(_) => {
                panic!("Unsupported, can't write GridPlacement");
            }
            StyleAttr::GridColumnStart(val) => {
                elem.push_attribute(("grid-column-start", i16::to_string(val).as_str()));
            }
            StyleAttr::GridColumnSpan(val) => {
                elem.push_attribute(("grid-column-span", u16::to_string(val).as_str()));
            }
            StyleAttr::GridColumnEnd(val) => {
                elem.push_attribute(("grid-column-end", i16::to_string(val).as_str()));
            }

            StyleAttr::LineBreak(dir) => {
                elem.push_attribute((
                    "line-break",
                    match dir {
                        bevy::text::BreakLineOn::NoWrap => "nowrap",
                        bevy::text::BreakLineOn::WordBoundary => "word",
                        bevy::text::BreakLineOn::AnyCharacter => "char",
                    },
                ));
            }

            _ => {
                todo!("Implement attr")
            }
        }
    }

    /// Convert a CSS-style color into a Color. Supports #hex, rgba() and hsla().
    fn parse_color(str: &str) -> Result<Color, GuiseError> {
        lazy_static! {
            static ref RE_RGBA: Regex =
                Regex::new(r"^rgba\(([\d\.]+),\s*([\d\.]+),\s*([\d\.]+),\s*([\d\.]+)\)$").unwrap();
            static ref RE_HSLA: Regex =
                Regex::new(r"^hsla\(([\d\.]+),\s*([\d\.]+),\s*([\d\.]+),\s*([\d\.]+)\)$").unwrap();
        }

        let h = Color::hex(str);
        if h.is_ok() {
            return Ok(h.unwrap());
        }

        RE_RGBA
            .captures(str)
            .map(|cap| {
                Color::rgba(
                    f32::from_str(&cap[1]).unwrap(),
                    f32::from_str(&cap[2]).unwrap(),
                    f32::from_str(&cap[3]).unwrap(),
                    f32::from_str(&cap[4]).unwrap(),
                )
            })
            .or(RE_HSLA.captures(str).map(|cap| {
                Color::hsla(
                    f32::from_str(&cap[1]).unwrap(),
                    f32::from_str(&cap[2]).unwrap(),
                    f32::from_str(&cap[3]).unwrap(),
                    f32::from_str(&cap[4]).unwrap(),
                )
            }))
            .ok_or(GuiseError::InvalidAttributeValue(str.to_string()))
    }

    /// Convert a CSS-style color into a Color. Supports #hex, rgba() and hsla().
    fn parse_grid_placement(str: &str) -> Result<GridPlacement, GuiseError> {
        lazy_static! {
            static ref RE_GRID_1: Regex = Regex::new(r"^([\d\.]+)\s*/\s*([\d\.]+)$").unwrap();
            static ref RE_GRID_2: Regex =
                Regex::new(r"^([\d\.]+)\s*/\s*span\s*([\d\.]+)$").unwrap();
        }

        RE_GRID_1
            .captures(str)
            .map(|cap| {
                GridPlacement::default()
                    .set_start(i16::from_str(&cap[1]).unwrap())
                    .set_end(i16::from_str(&cap[2]).unwrap())
            })
            .or(RE_GRID_2.captures(str).map(|cap| {
                GridPlacement::default()
                    .set_start(i16::from_str(&cap[1]).unwrap())
                    .set_span(u16::from_str(&cap[2]).unwrap())
            }))
            .ok_or(GuiseError::InvalidAttributeValue(str.to_string()))
    }

    /// Convert a CSS-style length string into a `Val`.
    pub(crate) fn parse_val(str: &str) -> Result<Val, GuiseError> {
        if str == "auto" {
            return Ok(Val::Auto);
        }
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([\-\d\.]+)(px|vw|vh|vmin|vmax|%)?$").unwrap();
        }
        RE.captures(str)
            .and_then(|cap| {
                let dist = f32::from_str(&cap[1]).unwrap();
                if cap.get(2).is_none() {
                    // Default to pixels if no unit
                    return Some(Val::Px(dist));
                }
                match &cap[2] {
                    "px" => Some(Val::Px(dist)),
                    "%" => Some(Val::Percent(dist)),
                    "vw" => Some(Val::Vw(dist)),
                    "vh" => Some(Val::Vh(dist)),
                    "vmin" => Some(Val::VMin(dist)),
                    "vmax" => Some(Val::VMax(dist)),
                    _ => {
                        panic!("Invalid unit");
                    }
                }
            })
            .ok_or(GuiseError::InvalidAttributeValue(str.to_string()))
    }

    /// Convert a CSS-style string representing a sequences of "lengths" into a `UiRect`.
    /// These go in CSS order: (top, right, bottom, left).
    /// CSS shortcut forms are supported.
    pub(crate) fn parse_uirect(str: &str) -> Result<UiRect, GuiseError> {
        let mut rect = UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Auto);
        let mut sides = str.split_whitespace();

        // Top
        if let Some(top) = sides.next() {
            rect.top = StyleAttr::parse_val(top)?;

            // Right defaults to top if not specified
            rect.right = match sides.next() {
                Some(val) => StyleAttr::parse_val(val)?,
                None => rect.top,
            };

            // Bottom defaults to top if not specified
            rect.bottom = match sides.next() {
                Some(val) => StyleAttr::parse_val(val)?,
                None => rect.top,
            };

            // Left defaults to right if not specified
            rect.left = match sides.next() {
                Some(val) => StyleAttr::parse_val(val)?,
                None => rect.right,
            };

            // Should be no more values.
            if sides.next().is_none() {
                Ok(rect)
            } else {
                Err(GuiseError::InvalidAttributeValue(str.to_string()))
            }
        } else {
            Err(GuiseError::InvalidAttributeValue(str.to_string()))
        }
    }

    /// Parse a scalar float.
    fn parse_f32(str: &str) -> Result<f32, GuiseError> {
        f32::from_str(str).or_else(|_| Err(GuiseError::InvalidAttributeValue(str.to_string())))
    }

    /// Parse a scalar i32.
    fn parse_i32(str: &str) -> Result<i32, GuiseError> {
        i32::from_str(str).or_else(|_| Err(GuiseError::InvalidAttributeValue(str.to_string())))
    }

    /// Parse a scalar i32.
    fn parse_i16(str: &str) -> Result<i16, GuiseError> {
        i16::from_str(str).or_else(|_| Err(GuiseError::InvalidAttributeValue(str.to_string())))
    }

    /// Parse a scalar i32.
    fn parse_u16(str: &str) -> Result<u16, GuiseError> {
        u16::from_str(str).or_else(|_| Err(GuiseError::InvalidAttributeValue(str.to_string())))
    }

    /// Convert a `Val` into a CSS-style string.
    fn val_to_str(val: Val) -> String {
        match val {
            Val::Auto => "auto".to_string(),
            Val::Px(px) => format!("{}px", px),
            Val::Percent(pct) => format!("{}%", pct),
            Val::Vw(v) => format!("{}vw", v),
            Val::Vh(v) => format!("{}vh", v),
            Val::VMin(v) => format!("{}vmin", v),
            Val::VMax(v) => format!("{}vmax", v),
        }
    }

    /// Convert a `UiRect` into a CSS-style string. The order of the values is (top, right, bottom,
    /// left).
    fn uirect_to_str(val: UiRect) -> String {
        format!(
            "{} {} {} {}",
            StyleAttr::val_to_str(val.top),
            StyleAttr::val_to_str(val.right),
            StyleAttr::val_to_str(val.bottom),
            StyleAttr::val_to_str(val.left)
        )
    }

    fn color_to_str(col: Color) -> String {
        match col {
            Color::Rgba {
                red,
                green,
                blue,
                alpha,
            } => format!("rgba({}, {}, {}, {})", red, green, blue, alpha),

            Color::Hsla {
                hue,
                saturation,
                lightness,
                alpha,
            } => format!("hsla({}, {}, {}, {})", hue, saturation, lightness, alpha),

            _ => {
                panic!("Unsupported color format")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_val() {
        assert_eq!(StyleAttr::parse_val("auto").unwrap(), Val::Auto);
        assert_eq!(StyleAttr::parse_val("1").unwrap(), Val::Px(1.));
        assert_eq!(StyleAttr::parse_val("1px").unwrap(), Val::Px(1.));
        assert_eq!(StyleAttr::parse_val("1vw").unwrap(), Val::Vw(1.));
        assert_eq!(StyleAttr::parse_val("1vh").unwrap(), Val::Vh(1.));
        assert_eq!(StyleAttr::parse_val("1.1px").unwrap(), Val::Px(1.1));

        assert!(StyleAttr::parse_val("1.1bad").is_err());
        assert!(StyleAttr::parse_val("bad").is_err());
        assert!(StyleAttr::parse_val("1.1.1bad").is_err());
    }

    #[test]
    fn test_parse_uirect() {
        assert_eq!(
            StyleAttr::parse_uirect("1px 2px 3px 4px").unwrap(),
            UiRect::new(Val::Px(4.), Val::Px(2.), Val::Px(1.), Val::Px(3.))
        );
        assert_eq!(
            StyleAttr::parse_uirect("1px 2px 3px").unwrap(),
            UiRect::new(Val::Px(2.), Val::Px(2.), Val::Px(1.), Val::Px(3.))
        );
        assert_eq!(
            StyleAttr::parse_uirect("1px 2px").unwrap(),
            UiRect::new(Val::Px(2.), Val::Px(2.), Val::Px(1.), Val::Px(1.))
        );
        assert_eq!(
            StyleAttr::parse_uirect("1px").unwrap(),
            UiRect::new(Val::Px(1.), Val::Px(1.), Val::Px(1.), Val::Px(1.))
        );

        assert!(StyleAttr::parse_uirect("1.1bad").is_err());
    }

    #[test]
    fn test_parse_attrs() {

        //         b"flex-direction" => StyleAttr::FlexDirection(match value {
        //             "row" => FlexDirection::Row,
        //             "column" => FlexDirection::Column,
        //             "row-reverse" => FlexDirection::RowReverse,
        //             "column-reverse" => FlexDirection::ColumnReverse,
        //             _ => {
        //                 return Err(GuiseError::UnknownAttributeValue(value.to_string()));
        //             }
        //         }),

        //         //     FlexWrap,
        //         // TODO: Allow shortcut forms for flex.
        //         b"flex" => StyleAttr::FlexGrow(StyleAttr::parse_f32(value)?),
        //         b"flex-grow" => StyleAttr::FlexGrow(StyleAttr::parse_f32(value)?),
        //         b"flex-shrink" => StyleAttr::FlexShrink(StyleAttr::parse_f32(value)?),
        //         b"flex-basis" => StyleAttr::FlexBasis(StyleAttr::parse_val(value)?),

        //         b"row-gap" => StyleAttr::RowGap(StyleAttr::parse_val(value)?),
        //         b"column-gap" => StyleAttr::ColumnGap(StyleAttr::parse_val(value)?),
        //         b"gap" => StyleAttr::Gap(StyleAttr::parse_val(value)?),

        //         b"grid-auto-flow" => StyleAttr::GridAutoFlow(match value {
        //             "row" => GridAutoFlow::Row,
        //             "column" => GridAutoFlow::Column,
        //             "row-dense" => GridAutoFlow::RowDense,
        //             "column-dense" => GridAutoFlow::ColumnDense,
        //             _ => {
        //                 return Err(GuiseError::UnknownAttributeValue(value.to_string()));
        //             }
        //         }),
        //         //     // TODO:
        //         //     // pub grid_template_rows: Option<Vec<RepeatedGridTrack>>,
        //         //     // pub grid_template_columns: Option<Vec<RepeatedGridTrack>>,
        //         //     // pub grid_auto_rows: Option<Vec<GridTrack>>,
        //         //     // pub grid_auto_columns: Option<Vec<GridTrack>>,
        //         b"grid-row" => StyleAttr::GridRow(StyleAttr::parse_grid_placement(value)?),
        //         b"grid-row-start" => StyleAttr::GridRowStart(StyleAttr::parse_i16(value)?),
        //         b"grid-row-span" => StyleAttr::GridRowSpan(StyleAttr::parse_u16(value)?),
        //         b"grid-row-end" => StyleAttr::GridRowEnd(StyleAttr::parse_i16(value)?),
        //         b"grid-column" => StyleAttr::GridColumn(StyleAttr::parse_grid_placement(value)?),
        //         b"grid-column-start" => StyleAttr::GridColumnStart(StyleAttr::parse_i16(value)?),
        //         b"grid-column-span" => StyleAttr::GridColumnSpan(StyleAttr::parse_u16(value)?),
        //         b"grid-column-end" => StyleAttr::GridColumnEnd(StyleAttr::parse_i16(value)?),
        //         _ => return Ok(None),
        //     }))
        // }
    }
}
