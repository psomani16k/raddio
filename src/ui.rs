use anyhow::Ok;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use edtui::{
    EditorEventHandler, EditorMode, EditorState, EditorTheme, EditorView, Lines,
    actions::{MoveBackward, MoveForward},
};
use ratatui::{
    Terminal,
    backend::Backend,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use unicode_width::UnicodeWidthStr;

use crate::config::{CONFIG, Station, UiConfig};

pub struct App {
    editor: EditorState,
    event_handler: EditorEventHandler,
    submitted: Option<String>,
    station: &'static Station,
    ui_conf: UiConfig,
}

fn new_editor() -> EditorState {
    let mut editor = EditorState::new(Lines::from(""));
    editor.mode = EditorMode::Insert;
    editor.set_single_line(true);
    editor
}

impl App {
    pub fn new(station: &'static Station) -> Self {
        let ui_conf = CONFIG.get_ui();
        let ui_conf = match &station.override_ui {
            Some(conf) => ui_conf.override_with(conf),
            None => ui_conf,
        };
        return Self {
            editor: new_editor(),
            event_handler: EditorEventHandler::emacs_mode(),
            submitted: None,
            station,
            ui_conf,
        };
    }

    pub fn run<B: Backend>(&mut self, term: &mut Terminal<B>) -> anyhow::Result<()>
    where
        B::Error: Send + Sync + 'static,
    {
        loop {
            self.draw(term)?;
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match key.code {
                        KeyCode::Char('c') => break,
                        KeyCode::Char('h') => self.editor.execute(MoveBackward(1)),
                        KeyCode::Char('l') => self.editor.execute(MoveForward(1)),
                        _ => self.event_handler.on_key_event(key, &mut self.editor),
                    }
                } else {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Enter => {
                            let text = self
                                .editor
                                .lines
                                .flatten(&None)
                                .into_iter()
                                .collect::<String>();
                            self.submitted = Some(text);
                            break;
                        }
                        _ => self.event_handler.on_key_event(key, &mut self.editor),
                    }
                }
            }
        }
        return Ok(());
    }

    /// Runs the station command for the submitted input, if any. Call this
    /// after the terminal has been restored so the spawned process gets a
    /// clean terminal. Falls back to the station's `default` for empty input.
    pub fn execute(&self) -> anyhow::Result<()> {
        let Some(input) = &self.submitted else {
            return Ok(());
        };
        let input = if input.is_empty() {
            self.station.default.as_deref().unwrap_or(input)
        } else {
            input
        };
        self.station.execute(input)
    }

    pub fn draw<B: Backend>(&mut self, term: &mut Terminal<B>) -> anyhow::Result<()>
    where
        B::Error: Send + Sync + 'static,
    {
        let ui = &self.ui_conf;
        let border_type = if ui.rounded_corners.unwrap_or(false) {
            BorderType::Rounded
        } else {
            BorderType::Plain
        };
        let borders = if ui.border.unwrap_or(true) {
            Borders::ALL
        } else {
            Borders::NONE
        };
        let border_color = ui
            .border_color
            .as_deref()
            .map(parse_hex_color)
            .unwrap_or(Color::White);
        let icon = ui.prefix.as_deref().unwrap_or("");
        let icon_width = icon.width() as u16;
        let icon_color = ui
            .prefix_color
            .as_deref()
            .map(parse_hex_color)
            .unwrap_or(border_color);

        term.draw(|frame| {
            let area = constrained_centered(
                ui.max_width.unwrap_or(usize::MAX) as u16,
                ui.max_height.unwrap_or(usize::MAX) as u16,
                frame.area(),
            );

            let desc = format!(" {} ", self.station.description.as_str());
            let block = Block::default()
                .title(desc)
                .title_alignment(Alignment::Center)
                .borders(borders)
                .border_type(border_type)
                .border_style(Style::default().fg(border_color));

            // Draw the border ourselves so we can place a fixed, non-editable
            // prefix icon before the editor inside the box.
            let inner = block.inner(area);
            frame.render_widget(block, area);

            let [prefix_area, editor_area] =
                Layout::horizontal([Constraint::Length(icon_width), Constraint::Min(0)])
                    .areas(inner);

            let prefix = Paragraph::new(icon).style(Style::default().fg(icon_color));
            frame.render_widget(prefix, prefix_area);

            let theme = EditorTheme::default()
                .block(Block::default())
                .base(Style::default().fg(Color::White))
                .hide_status_line();

            EditorView::new(&mut self.editor)
                .theme(theme)
                .single_line(true)
                .wrap(false)
                .render(editor_area, frame.buffer_mut());

            if let Some(pos) = self.editor.cursor_screen_position() {
                frame.set_cursor_position(pos);
            }
        })?;
        return Ok(());
    }
}

fn constrained_centered(max_width: u16, max_height: u16, area: Rect) -> Rect {
    let width = max_width.min(area.width);
    let height = max_height.min(area.height);
    Rect {
        x: area.x + (area.width - width) / 2,
        y: area.y + (area.height - height) / 2,
        width,
        height,
    }
}

fn parse_hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    Color::Rgb(r, g, b)
}
