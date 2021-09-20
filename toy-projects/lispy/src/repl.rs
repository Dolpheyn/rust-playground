use std::io::{self, Stdout, Write};

use crossterm::{
    cursor::{
        position, MoveLeft, MoveRight, MoveTo, MoveToColumn, MoveToNextLine, RestorePosition,
        SavePosition,
    },
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand, QueueableCommand, Result,
};

pub(crate) enum ReplInput {
    String(String),
    Skip,
    Exit,
}

pub(crate) fn get_input(stdout: &mut Stdout) -> Result<ReplInput> {
    let start_of_buffer_pos = cursor_column()?;
    let mut end_of_buffer_pos = start_of_buffer_pos;
    let mut cursor_pos = start_of_buffer_pos;

    let mut buffer = String::new();
    loop {
        match read()? {
            Event::Key(KeyEvent { code, modifiers }) => {
                if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('c') {
                    return Ok(ReplInput::Exit);
                }

                match code {
                    KeyCode::Enter => {
                        break;
                    }
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

    // Just print the prompt again in the next line if user presses `Enter`
    // with an empty input.
    if buffer.is_empty() {
        stdout.execute(MoveToNextLine(1))?;
        return Ok(ReplInput::Skip);
    }

    if buffer.eq("exit") || buffer.eq("quit") {
        return Ok(ReplInput::Exit);
    }

    Ok(ReplInput::String(buffer))
}

pub(crate) fn print_ver(stdout: &mut io::Stdout) -> Result<()> {
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

pub(crate) fn print_prompt(stdout: &mut io::Stdout) -> Result<()> {
    stdout
        .queue(SetForegroundColor(Color::Blue))?
        .queue(Print("lispy > "))?
        .queue(ResetColor)?;
    stdout.flush()?;
    Ok(())
}

pub(crate) fn print_eval(stdout: &mut Stdout, output: String) -> Result<()> {
    stdout
        .queue(MoveToNextLine(1))?
        .queue(Print(&output))?
        .queue(MoveToNextLine(1))?;
    stdout.flush()?;
    Ok(())
}

fn cursor_column() -> Result<u16> {
    let (col, _) = position()?;
    Ok(col)
}
