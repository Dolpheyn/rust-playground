use std::io;

use crossterm::{
    cursor::{MoveLeft, MoveRight, MoveToNextLine},
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
                        KeyCode::Backspace => {
                            if buffer.len() == 0 {
                                continue;
                            }

                            buffer.pop();
                            stdout
                                .execute(MoveLeft(1))?
                                .execute(Print(" "))?
                                .execute(MoveLeft(1))?;
                        }
                        KeyCode::Enter => {
                            stdout.execute(MoveToNextLine(1))?;
                            break 'inner;
                        }
                        KeyCode::Left => {
                            stdout.execute(MoveLeft(1))?;
                        }
                        KeyCode::Right => {
                            stdout.execute(MoveRight(1))?;
                        }
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
