use std::io::{self, Write};

use crossterm::{
    cursor::{
        position, MoveLeft, MoveRight, MoveTo, MoveToColumn, MoveToNextLine, RestorePosition,
        SavePosition,
    },
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand, Result,
};

fn print_ver(mut stdout: &io::Stdout) -> Result<()> {
    stdout
        .queue(Clear(ClearType::All))?
        .queue(MoveTo(0, 0))?
        .queue(Print("Lispy Version 0.1.0"))?
        .queue(MoveToNextLine(1))?
        .queue(Print("Press Ctrl + c to Exit"))?
        .queue(MoveToNextLine(2))?;
    stdout.flush()?;
    Ok(())
}

fn print_prompt(mut stdout: &io::Stdout) -> Result<()> {
    stdout
        .queue(SetForegroundColor(Color::Blue))?
        .queue(Print("lispy > "))?
        .queue(ResetColor)?;
    stdout.flush()?;
    Ok(())
}

fn cursor_column() -> Result<u16> {
    let (col, _) = position()?;
    Ok(col)
}

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    terminal::enable_raw_mode()?;
    print_ver(&mut stdout)?;

    'repl: loop {
        print_prompt(&mut stdout)?;

        let start_of_buffer_pos = cursor_column()?;
        let mut end_of_buffer_pos = start_of_buffer_pos;
        let mut cursor_pos = start_of_buffer_pos;

        'input: loop {
            match read()? {
                Event::Key(KeyEvent { code, modifiers }) => {
                    if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('c') {
                        break 'repl;
                    }

                    match code {
                        KeyCode::Char(c) => {
                            if cursor_pos == end_of_buffer_pos {
                                buffer.push(c);
                                stdout.queue(Print(c))?;
                            } else {
                                let insert_idx = (cursor_pos - start_of_buffer_pos) as usize;
                                buffer.insert(insert_idx, c);

                                stdout
                                    .queue(SavePosition)?
                                    .queue(Print(&buffer[insert_idx..]))?
                                    .queue(RestorePosition)?
                                    .queue(MoveRight(1))?;
                            }

                            cursor_pos += 1;
                            end_of_buffer_pos += 1;

                            stdout.flush()?;
                        }
                        KeyCode::Backspace => {
                            if buffer.is_empty() {
                                continue;
                            }

                            stdout
                                .queue(MoveLeft(1))?
                                .queue(Print(" "))?
                                .queue(MoveLeft(1))?;

                            if cursor_pos == end_of_buffer_pos {
                                buffer.pop();
                            } else {
                                let remove_idx = (cursor_pos - start_of_buffer_pos - 1) as usize;
                                buffer.remove(remove_idx);

                                stdout
                                    .queue(SavePosition)?
                                    .queue(Print(format!("{} ", &buffer[remove_idx..])))?
                                    .queue(RestorePosition)?;
                            }

                            end_of_buffer_pos -= 1;
                            cursor_pos -= 1;

                            stdout.flush()?;
                        }
                        KeyCode::Delete => {
                            if cursor_pos == end_of_buffer_pos {
                                continue;
                            }

                            let remove_idx = (cursor_pos - start_of_buffer_pos) as usize;
                            buffer.remove(remove_idx);

                            stdout
                                .queue(SavePosition)?
                                .queue(Print(format!("{} ", &buffer[remove_idx..])))?
                                .queue(RestorePosition)?;
                            stdout.flush()?;
                        }
                        KeyCode::Enter => {
                            break 'input;
                        }
                        KeyCode::Left => {
                            if cursor_pos == start_of_buffer_pos {
                                continue;
                            }

                            cursor_pos -= 1;
                            stdout.execute(MoveLeft(1))?;
                        }
                        KeyCode::Right => {
                            if cursor_pos == end_of_buffer_pos {
                                continue;
                            }

                            cursor_pos += 1;
                            stdout.execute(MoveRight(1))?;
                        }
                        KeyCode::Home => {
                            let move_to_col = start_of_buffer_pos + 1;
                            stdout.execute(MoveToColumn(move_to_col))?; // cursor::position is 0 indexed, but Column is 1 indexed.
                            cursor_pos = start_of_buffer_pos;
                        }
                        KeyCode::End => {
                            let move_to_col = end_of_buffer_pos + 1;
                            stdout.execute(MoveToColumn(move_to_col))?;
                            cursor_pos = end_of_buffer_pos;
                        }
                        KeyCode::Up => {}
                        KeyCode::Down => {}
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
