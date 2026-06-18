use anyhow::Ok;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use edtui::{
    EditorEventHandler, EditorMode, EditorState, EditorTheme, EditorView, Lines,
    actions::{LineBreak, MoveBackward, MoveDown, MoveForward, MoveUp},
};
use ratatui::{
    Terminal,
    backend::Backend,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use unicode_width::UnicodeWidthStr;

use crate::config::{CONFIG, Station, UiConfig, parse_hex_color};

pub struct App {
    editor: EditorState,
    event_handler: EditorEventHandler,
    submitted: Option<String>,
    station: &'static Station,
    ui_conf: UiConfig,
    multiline: bool,
}

fn new_editor(multiline: bool) -> EditorState {
    let mut editor = EditorState::new(Lines::from(""));
    editor.mode = EditorMode::Insert;
    editor.set_single_line(!multiline);
    editor
}

impl App {
    pub fn new(station: &'static Station) -> Self {
        let ui_conf = CONFIG.get_ui();
        let ui_conf = match &station.override_ui {
            Some(conf) => ui_conf.override_with(conf),
            None => ui_conf,
        };
        let multiline = ui_conf.multiline.expect("This should never happen");
        Self {
            editor: new_editor(multiline),
            event_handler: EditorEventHandler::emacs_mode(),
            submitted: None,
            station,
            ui_conf,
            multiline,
        }
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
                        KeyCode::Enter if self.multiline => self.editor.execute(LineBreak(1)),
                        KeyCode::Char('j') if self.multiline => self.editor.execute(MoveDown(1)),
                        KeyCode::Char('k') if self.multiline => self.editor.execute(MoveUp(1)),
                        _ => self.event_handler.on_key_event(key, &mut self.editor),
                    }
                } else {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Enter => {
                            let text = self
                                .editor
                                .lines
                                .flatten(&Some('\n'))
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
        Ok(())
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
        let border_type = match ui.rounded_corners.expect("This should never be empty") {
            true => BorderType::Rounded,
            false => BorderType::Plain,
        };
        let borders = match ui.border.expect("This should never be empty") {
            true => Borders::ALL,
            false => Borders::NONE,
        };
        let border_color = ui
            .border_color
            .as_deref()
            .map(parse_hex_color)
            .expect("This should never be empty");
        let icon = ui.prefix.as_deref().expect("This should never be empty");
        let icon_width = icon.width() as u16;
        let icon_color = ui
            .prefix_color
            .as_deref()
            .map(parse_hex_color)
            .expect("This should never be empty");

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
                .cursor_style(self.ui_conf.get_cursor_style())
                .base(Style::default().fg(Color::White))
                .hide_status_line();

            EditorView::new(&mut self.editor)
                .theme(theme)
                .single_line(!self.multiline)
                .wrap(false)
                .render(editor_area, frame.buffer_mut());

            if let Some(pos) = self.editor.cursor_screen_position() {
                frame.set_cursor_position(pos);
            }
        })?;
        Ok(())
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
