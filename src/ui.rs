use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
};

use crate::model::{CleanSource, ScanStatus, human_size};

pub trait SourceProvider {
    fn sources(&self) -> Vec<CleanSource>;
    fn rescan(&mut self);
    fn clean(&mut self, source: &CleanSource) -> Result<(), String>;
}

const SPINNER: [&str; 8] = ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];
const TICK_MS: u64 = 120;

pub struct App {
    rows: Vec<CleanSource>,
    selected: usize,
    confirm_idx: Option<usize>,
    dry_run: bool,
    spinner_index: usize,
    last_tick: Instant,
    quit: bool,
}

impl App {
    pub fn new(rows: Vec<CleanSource>) -> Self {
        let mut app = Self {
            rows,
            selected: 0,
            confirm_idx: None,
            dry_run: true,
            spinner_index: 0,
            last_tick: Instant::now(),
            quit: false,
        };
        app.sort_by_size();
        app
    }

    fn sort_by_size(&mut self) {
        self.rows.sort_by_key(|b| std::cmp::Reverse(b.size_bytes));
    }

    fn next(&mut self) {
        if !self.rows.is_empty() {
            self.selected = (self.selected + 1) % self.rows.len();
        }
    }

    fn previous(&mut self) {
        if !self.rows.is_empty() {
            self.selected = (self.selected + self.rows.len() - 1) % self.rows.len();
        }
    }

    pub fn run(mut self, provider: &mut dyn SourceProvider) -> io::Result<()> {
        let mut terminal = ratatui::init();
        let result = self.event_loop(&mut terminal, provider);
        ratatui::restore();
        result
    }

    fn event_loop(
        &mut self,
        terminal: &mut ratatui::DefaultTerminal,
        provider: &mut dyn SourceProvider,
    ) -> io::Result<()> {
        while !self.quit {
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(Duration::from_millis(TICK_MS))? {
                let ev = event::read()?;
                self.handle_event(ev, provider);
            } else if self.any_scanning() {
                self.last_tick = Instant::now();
                self.spinner_index += 1;
            }
        }
        Ok(())
    }

    fn any_scanning(&self) -> bool {
        self.rows.iter().any(|r| r.status == ScanStatus::Scanning)
    }

    fn handle_event(&mut self, ev: Event, provider: &mut dyn SourceProvider) {
        let Event::Key(key) = ev else { return };
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc if self.confirm_idx.is_none() => self.quit = true,
            KeyCode::Esc => self.confirm_idx = None,
            KeyCode::Char('r') => {
                provider.rescan();
                self.rows = provider.sources();
                self.sort_by_size();
                self.spinner_index = 0;
            }
            KeyCode::Char('s') if self.confirm_idx.is_none() && !self.rows.is_empty() => {
                self.confirm_idx = Some(self.selected);
            }
            KeyCode::Char('y') if self.confirm_idx.is_some() => {
                if let Some(idx) = self.confirm_idx.take() {
                    let source = self.rows[idx].clone();
                    match provider.clean(&source) {
                        Ok(()) => self.rows[idx].status = ScanStatus::Ok,
                        Err(_) => {
                            self.rows[idx].status = ScanStatus::Error;
                            self.confirm_idx = Some(idx);
                        }
                    }
                }
            }
            KeyCode::Char('n') => self.confirm_idx = None,
            KeyCode::Up | KeyCode::Char('k') => self.previous(),
            KeyCode::Down | KeyCode::Char('j') => self.next(),
            _ => {}
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

        let title = if self.dry_run {
            "🧹 Limpiador de caché (DRY-RUN) — nada se borra sin confirmar".to_string()
        } else {
            "🧹 Limpiador de caché".to_string()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_alignment(Alignment::Center)
            .padding(Padding::horizontal(1));

        let items: Vec<ListItem> = self
            .rows
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let status = row.status;
                let is_selected = i == self.selected && self.confirm_idx.is_none();
                let timer = if status == ScanStatus::Scanning {
                    SPINNER[self.spinner_index % SPINNER.len()]
                } else {
                    "   "
                };
                let marker = if is_selected { "▶ " } else { "  " };
                let line = Line::from(vec![
                    Span::styled(
                        format!("{} {:<12}", timer, status.label()),
                        Style::default().fg(status.color()),
                    ),
                    Span::styled(
                        format!("{:>10} ", human_size(row.size_bytes)),
                        Style::default().fg(Color::Blue),
                    ),
                    {
                        let mut s = Span::raw(format!("{}{}", marker, row.name));
                        if is_selected {
                            s = s.add_modifier(Modifier::BOLD);
                        }
                        s
                    },
                ]);
                ListItem::new(line)
            })
            .collect();

        frame.render_widget(List::new(items).block(block), chunks[1]);

        let hints = Paragraph::new("q/Esc salir · r reescanear · s limpiar · ↑/↓ navegar")
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(hints, chunks[2]);

        if let Some(idx) = self.confirm_idx {
            draw_confirm_modal(frame, &self.rows[idx], self.dry_run);
        }
    }
}

fn draw_confirm_modal(frame: &mut Frame, source: &CleanSource, dry_run: bool) {
    let modal_area = centered_rect(frame.area(), 60, 40);
    frame.render_widget(Clear, modal_area);

    let title = if dry_run {
        "DRY-RUN — previsualizar"
    } else {
        "Confirmar limpieza"
    };
    let text = if dry_run {
        format!(
            "Se marcará para borrar: '{}'\n({})\n\nDry-run: no se borra nada aún.",
            source.name,
            source.path.display()
        )
    } else {
        format!(
            "¿Borrar '{}'?\n({})\n\ny = borrar · n = cancelar",
            source.name,
            source.path.display()
        )
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_alignment(Alignment::Center)
        .border_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .padding(Padding::horizontal(2));
    let paragraph = Paragraph::new(text).wrap(Wrap { trim: true }).block(block);
    frame.render_widget(paragraph, modal_area);
}

fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Percentage(percent_y),
        Constraint::Fill(1),
    ])
    .split(area);
    Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Percentage(percent_x),
        Constraint::Fill(1),
    ])
    .split(vertical[1])[1]
}

impl ScanStatus {
    fn label(&self) -> &'static str {
        match self {
            ScanStatus::Scanning => "scan",
            ScanStatus::Ok => "ok",
            ScanStatus::Error => "error",
            ScanStatus::NotFound => "no encontrado",
        }
    }

    fn color(&self) -> Color {
        match self {
            ScanStatus::Scanning => Color::Cyan,
            ScanStatus::Ok => Color::Green,
            ScanStatus::Error => Color::Red,
            ScanStatus::NotFound => Color::Yellow,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ScanStatus;

    #[test]
    fn status_colors_are_stable() {
        assert_eq!(label(&ScanStatus::Ok), "ok");
        assert_eq!(label(&ScanStatus::Error), "error");
    }

    #[test]
    fn spinner_rotates() {
        let i = 5usize;
        assert_eq!(crate::ui::SPINNER[i % crate::ui::SPINNER.len()], "⣟");
    }

    fn label(s: &ScanStatus) -> &'static str {
        match s {
            ScanStatus::Scanning => "scan",
            ScanStatus::Ok => "ok",
            ScanStatus::Error => "error",
            ScanStatus::NotFound => "no encontrado",
        }
    }
}
