use std::io;

use crossterm::{
    cursor::{MoveToNextLine, RestorePosition},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Print,
    terminal, ExecutableCommand, Result,
};

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    let mut buffer = String::new();
    terminal::enable_raw_mode()?;

    stdout
        .execute(Print("Lispy Version 0.1.0"))?
        .execute(MoveToNextLine(1))?
        .execute(Print("Press Ctrl + c to Exit"))?
        .execute(MoveToNextLine(3))?;

    'outer: loop {
        buffer.clear();
        stdout.execute(Print("lispy > "))?;

        'inner: loop {
            match read()? {
                Event::Key(KeyEvent { code, modifiers }) => {
                    if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('c') {
                        break 'outer;
                    }

                    match code {
                        KeyCode::Char(c) => {
                            buffer.push(c);
                            stdout.execute(Print(c))?;
                        }
                        KeyCode::Backspace => {}
                        KeyCode::Enter => {
                            stdout.execute(MoveToNextLine(1))?;
                            break 'inner;
                        }
                        KeyCode::Left => {}
                        KeyCode::Right => {}
                        KeyCode::Up => {}
                        KeyCode::Down => {}
                        KeyCode::Home => {}
                        KeyCode::End => {}
                        _ => {}
                    }
                }

                _ => {}
            }
        }

        stdout
            .execute(Print("Lispy says: "))?
            .execute(Print(&buffer))?
            .execute(MoveToNextLine(2))?;
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
