use ratatui::style::Style;
use serde::{Deserialize, Serialize};

use crate::config::parse_hex_color;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UiConfig {
    /// This is the max height of the window including the decorations,
    /// actual text area may be smaller
    pub max_height: Option<usize>,
    /// This is the max width of the window including the decorations,
    /// actual text area may be smaller
    pub max_width: Option<usize>,

    pub rounded_corners: Option<bool>,
    pub border: Option<bool>,
    pub border_color: Option<String>,

    /// Decorative, non-editable prefix shown at the start of the input box.
    pub prefix: Option<String>,
    pub prefix_color: Option<String>,

    /// When true, the input box accepts multiple lines (Ctrl+Enter inserts a
    /// newline). Independent of the box size; content scrolls within it.
    pub multiline: Option<bool>,

    // Cursor styling
    pub cursor_style: Option<CursorStyle>,
    pub cursor_color: Option<String>,
}

impl UiConfig {
    pub fn override_with(&self, ui_config: &UiConfig) -> Self {
        let mut result = self.clone();
        if ui_config.max_height.is_some() {
            result.max_height = ui_config.max_height;
        }
        if ui_config.max_width.is_some() {
            result.max_width = ui_config.max_width;
        }
        if ui_config.rounded_corners.is_some() {
            result.rounded_corners = ui_config.rounded_corners;
        }
        if ui_config.border.is_some() {
            result.border = ui_config.border;
        }
        if ui_config.border_color.is_some() {
            result.border_color = ui_config.border_color.clone();
        }
        if ui_config.prefix.is_some() {
            result.prefix = ui_config.prefix.clone();
        }
        if ui_config.prefix_color.is_some() {
            result.prefix_color = ui_config.prefix_color.clone();
        }
        if ui_config.multiline.is_some() {
            result.multiline = ui_config.multiline;
        }
        if ui_config.cursor_style.is_some() {
            result.cursor_style = ui_config.cursor_style.clone();
        }
        if ui_config.cursor_color.is_some() {
            result.cursor_color = ui_config.cursor_color.clone();
        }
        result
    }

    pub fn get_cursor_style(&self) -> Style {
        let mut style = Style::default();
        if let Some(s) = &self.cursor_style {
            match s {
                CursorStyle::Line => {
                    style = Style::dim(style);
                }
                CursorStyle::Block => {
                    style = Style::bold(style);
                }
            };
        };
        if let Some(c) = &self.cursor_color {
            let color = parse_hex_color(c);
            style = Style::fg(style, color);
        }
        style
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        UiConfig {
            max_height: Some(3),
            max_width: Some(40),
            rounded_corners: Some(false),
            border: Some(true),
            border_color: Some(String::from("#33FF33")),
            prefix: Some(String::from(" \u{f044} ")),
            prefix_color: Some(String::from("#33FFFF")),
            multiline: Some(true),
            cursor_style: Some(CursorStyle::Line),
            cursor_color: Some(String::from("#FFFFFF"))
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub enum CursorStyle {
    Line,
    Block,
}
