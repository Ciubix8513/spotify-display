use crossterm::event::KeyModifiers;
use crossterm::terminal::disable_raw_mode;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use spotify::Spotify;
use tui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};

mod grimoire;
mod helpers;
mod spotify;

fn main() -> Result<(), std::io::Error> {
    let spotify = Spotify::new().expect("Failed to init dbus spotify");

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let d = spotify.get_data().unwrap_or_default();

        terminal.draw(|f| {
            let size = f.size();

            let text = Paragraph::new(format!(
                "{}\n{}",
                // size,
                helpers::add_linebreaks(&d.title().unwrap_or_default(), size.width as usize),
                d.artist().unwrap_or_default(),
            ));

            f.render_widget(text, size);
        })?;

        if event::poll(grimoire::TIMEOUT).unwrap_or_default() {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        break
                    }
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;

    Ok(())
}
