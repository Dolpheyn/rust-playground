use std::io::{self, Write};

use crossterm::{
    cursor::{MoveLeft, MoveRight, MoveTo, MoveToNextLine},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand, Result,
};

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    let mut buffer = String::new();
    terminal::enable_raw_mode()?;

    stdout
        .queue(Clear(ClearType::All))?
        .queue(MoveTo(0, 0))?
        .queue(Print("Lispy Version 0.1.0"))?
        .queue(MoveToNextLine(1))?
        .queue(Print("Press Ctrl + c to Exit"))?
        .queue(MoveToNextLine(2))?;
    stdout.flush()?;

    'repl: loop {
        stdout
            .queue(SetForegroundColor(Color::Blue))?
            .queue(Print("lispy > "))?
            .queue(ResetColor)?;
        stdout.flush()?;

        'input: loop {
            match read()? {
                Event::Key(KeyEvent { code, modifiers }) => {
                    if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('c') {
                        break 'repl;
                    }

                    match code {
                        KeyCode::Char(c) => {
                            buffer.push(c);
                            stdout.execute(Print(c))?;
                        }
                        KeyCode::Backspace => {
                            if buffer.is_empty() {
                                continue;
                            }

                            buffer.pop();
                            stdout
                                .queue(MoveLeft(1))?
                                .queue(Print(" "))?
                                .queue(MoveLeft(1))?;
                            stdout.flush()?;
                        }
                        KeyCode::Enter => {
                            break 'input;
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

        if buffer.is_empty() {
            stdout.execute(MoveToNextLine(1))?;
            continue;
        }

        if buffer == "exit" {
            break 'repl;
        }

        stdout
            .queue(MoveToNextLine(1))?
            .queue(Print("Lispy says: "))?
            .queue(Print(&buffer))?
            .queue(MoveToNextLine(1))?;
        stdout.flush()?;
        buffer.clear();
    }

    stdout.queue(MoveToNextLine(1))?.queue(Print("Bye bye!"))?;
    stdout.flush()?;

    terminal::disable_raw_mode()?;
    Ok(())
}
