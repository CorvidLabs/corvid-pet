//! Live pet mode with real-time TUI.
//!
//! This module provides an interactive, real-time pet experience
//! using ratatui for the terminal UI.
//!
//! Requires the `live` feature to be enabled.
//!
//! # Example
//!
//! ```no_run
//! # #[cfg(feature = "live")]
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use corvid_pet::{Pet, Species};
//! use corvid_pet::live::LivePetApp;
//!
//! let pet = Pet::new("Corvin".to_string(), Species::Crow);
//! let mut app = LivePetApp::new(pet);
//! app.run().await?;
//! # Ok(())
//! # }
//! ```

use crate::{Mood, Pet};

/// State for the live pet application.
#[derive(Debug)]
#[allow(dead_code)]
pub struct LivePetApp {
    pet: Pet,
    running: bool,
    frame_count: u64,
    last_interaction: std::time::Instant,
    auto_blink: bool,
    show_help: bool,
}

impl LivePetApp {
    /// Creates a new live pet app with the given pet.
    pub fn new(pet: Pet) -> Self {
        Self {
            pet,
            running: true,
            frame_count: 0,
            last_interaction: std::time::Instant::now(),
            auto_blink: true,
            show_help: true,
        }
    }

    /// Runs the live pet loop.
    #[cfg(feature = "live")]
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use crossterm::{
            event::{self},
            terminal::{disable_raw_mode, enable_raw_mode},
        };
        use ratatui::{Terminal, backend::CrosstermBackend};
        use std::io;
        use tokio::time::{Duration, interval};

        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        crossterm::execute!(
            stdout,
            crossterm::terminal::EnterAlternateScreen,
            crossterm::event::EnableMouseCapture
        )?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main loop with 30 FPS
        let mut tick_interval = interval(Duration::from_millis(33));

        while self.running {
            tokio::select! {
                _ = tick_interval.tick() => {
                    self.on_tick();
                    terminal.draw(|f| self.draw(f))?;
                }
                evt = async {
                    if event::poll(Duration::from_millis(10)).unwrap_or(false)
                        && let Ok(evt) = event::read()
                    {
                        return Some(evt);
                    }
                    None
                } => {
                    if let Some(evt) = evt {
                        self.handle_event(evt);
                    }
                }
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::event::DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    /// Stub for when live feature is disabled.
    #[cfg(not(feature = "live"))]
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Err("Live mode requires the 'live' feature".into())
    }

    #[allow(dead_code)]
    fn on_tick(&mut self) {
        self.frame_count += 1;

        // Auto-blur eyes every 5 seconds (at 30 FPS)
        if self.auto_blink && self.frame_count.is_multiple_of(150) {
            // Trigger a temporary blink state
        }

        // Check idle timeout (30 seconds)
        if self.last_interaction.elapsed().as_secs() > 30 {
            self.pet.set_mood(Mood::Sleepy);
        }
    }

    #[allow(dead_code)]
    #[cfg(feature = "live")]
    fn handle_event(&mut self, event: crossterm::event::Event) {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        if let Event::Key(key) = event
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                KeyCode::Char('h') => self.pet.set_mood(Mood::Happy),
                KeyCode::Char('s') => self.pet.set_mood(Mood::Sad),
                KeyCode::Char('n') => self.pet.set_mood(Mood::Neutral),
                KeyCode::Char('c') => self.pet.set_mood(Mood::Confused),
                KeyCode::Char('e') => self.pet.set_mood(Mood::Excited),
                KeyCode::Char('z') => self.pet.set_mood(Mood::Sleepy),
                KeyCode::Char('?') => self.show_help = !self.show_help,
                KeyCode::Char('b') => self.auto_blink = !self.auto_blink,
                _ => {}
            }
            self.last_interaction = std::time::Instant::now();
        }
    }

    #[cfg(not(feature = "live"))]
    fn handle_event(&mut self, _event: ()) {}

    #[cfg(feature = "live")]
    fn draw(&self, frame: &mut ratatui::Frame) {
        use ratatui::{
            layout::{Alignment, Constraint, Direction, Layout},
            widgets::{Block, Borders, Paragraph, Wrap},
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Min(10), Constraint::Length(8)])
            .split(frame.area());

        // Pet display area
        let art = self.pet.render();
        let pet_widget = Paragraph::new(art)
            .block(
                Block::default()
                    .title(format!(" {} - {} ", self.pet.name(), self.pet.mood()))
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Center);
        frame.render_widget(pet_widget, chunks[0]);

        // Status/Help area
        let status_text = if self.show_help {
            format!(
                "Controls:\n\
                h/s/n/c/e/z - Set mood (Happy/Sad/Neutral/Confused/Excited/Sleepy)\n\
                b - Toggle auto-blink: {}\n\
                ? - Toggle help\n\
                q/ESC - Quit",
                if self.auto_blink { "ON" } else { "OFF" }
            )
        } else {
            format!(
                "Press '?' for help | {} | {} | Auto-blink: {}",
                self.pet.name(),
                self.pet.mood(),
                if self.auto_blink { "ON" } else { "OFF" }
            )
        };

        let status_widget = Paragraph::new(status_text)
            .block(Block::default().title(" Status ").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(status_widget, chunks[1]);
    }
}

/// A simpler non-async version for synchronous contexts.
#[allow(dead_code)]
pub struct SimpleLivePet {
    pet: Pet,
}

impl SimpleLivePet {
    /// Creates a new simple live pet.
    pub fn new(pet: Pet) -> Self {
        Self { pet }
    }

    /// Displays the pet in the terminal with some interactivity.
    #[cfg(feature = "live")]
    pub fn run_interactive(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use crossterm::{
            event::{self, Event, KeyCode},
            terminal::{disable_raw_mode, enable_raw_mode},
        };

        enable_raw_mode()?;
        let mut stdout = std::io::stdout();

        // Clear screen and hide cursor
        crossterm::execute!(
            stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0),
            crossterm::cursor::Hide
        )?;

        Self::draw_frame(&mut stdout, &self.pet)?;

        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('h') => self.pet.set_mood(Mood::Happy),
                        KeyCode::Char('s') => self.pet.set_mood(Mood::Sad),
                        KeyCode::Char('n') => self.pet.set_mood(Mood::Neutral),
                        KeyCode::Char('c') => self.pet.set_mood(Mood::Confused),
                        KeyCode::Char('e') => self.pet.set_mood(Mood::Excited),
                        KeyCode::Char('z') => self.pet.set_mood(Mood::Sleepy),
                        _ => {}
                    }

                    // Clear and redraw
                    crossterm::execute!(
                        stdout,
                        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                        crossterm::cursor::MoveTo(0, 0)
                    )?;
                    Self::draw_frame(&mut stdout, &self.pet)?;
                }
            }
        }

        // Show cursor again
        crossterm::execute!(stdout, crossterm::cursor::Show)?;

        disable_raw_mode()?;
        Ok(())
    }

    #[cfg(feature = "live")]
    fn draw_frame(
        stdout: &mut std::io::Stdout,
        pet: &Pet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;

        // Use crossterm cursor positioning for proper layout
        use crossterm::{cursor::MoveTo, execute};

        // Print header with explicit cursor positioning
        execute!(stdout, MoveTo(0, 0))?;
        println!("╔══════════════════════════════════════╗");
        execute!(stdout, MoveTo(0, 1))?;
        println!("║       CORVID PET - Live Mode         ║");
        execute!(stdout, MoveTo(0, 2))?;
        println!("╚══════════════════════════════════════╝");

        // Print the pet art starting at row 4
        let mut row = 4;
        for line in pet.render().lines() {
            execute!(stdout, MoveTo(0, row))?;
            println!("  {}", line);
            row += 1;
        }

        // Print status below the art
        row += 1;
        execute!(stdout, MoveTo(0, row))?;
        println!("  Name:  {}", pet.name());
        row += 1;
        execute!(stdout, MoveTo(0, row))?;
        println!("  Mood:  {}", pet.mood());
        row += 2;
        execute!(stdout, MoveTo(0, row))?;
        println!("  Controls:");
        row += 1;
        execute!(stdout, MoveTo(0, row))?;
        println!("    h/s/n/c/e/z - Change mood");
        row += 1;
        execute!(stdout, MoveTo(0, row))?;
        println!("    q - Quit");

        stdout.flush()?;
        Ok(())
    }

    #[cfg(not(feature = "live"))]
    pub fn run_interactive(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Err("Live mode requires the 'live' feature".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Species;

    #[test]
    fn test_live_app_creation() {
        let pet = Pet::new("Test".to_string(), Species::Crow);
        let app = LivePetApp::new(pet);
        assert!(app.running);
        assert_eq!(app.frame_count, 0);
    }

    #[test]
    fn test_simple_live_pet() {
        let pet = Pet::new("Test".to_string(), Species::Crow);
        let live = SimpleLivePet::new(pet);
        assert_eq!(live.pet.name(), "Test");
    }
}
