use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyModifiers},
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};

/// Prompt suitable for when you need the user to input a string.
///
/// # Example
///
/// ```no_run
/// use selthi::Input;
///
/// let ans: Option<String> = Input::new("What's your name?").prompt();
///
/// match ans {
///     Some(name) => println!("Hello {}!", name),
///     None => println!("There was an error, please try again"),
/// }
/// ```
#[derive(Debug)]
pub struct Input<'a> {
    pub message: &'a str,
    pub minimum_chars: u16,
}

impl<'a> Input<'a> {
    pub const COLOR: Color = Color::Yellow;
    pub const DEFAULT_MINIMUM_CHAR: u16 = 0;

    /// Creates an [Input] with the provided message.
    pub fn new(message: &'a str) -> Self {
        Self {
            message,
            minimum_chars: Self::DEFAULT_MINIMUM_CHAR,
        }
    }

    /// Sets the minimum number of characters required for the response.
    pub fn with_minimum_chars(mut self, min_chars: u16) -> Self {
        self.minimum_chars = min_chars;
        self
    }

    /// Returns the string that the user typed.
    pub fn prompt(self) -> Option<String> {
        let mut stdout = stdout();
        let mut answer = String::new();
        let mut quit = false;

        terminal::enable_raw_mode().unwrap();

        self.draw_text(&mut stdout, &answer);

        while !quit {
            while poll(Duration::ZERO).unwrap() {
                let space = 1;
                let message_len = (self.message.chars().count() + space) as u16;
                let cursor_pos = crossterm::cursor::position().unwrap().0;

                match read().unwrap() {
                    Event::Key(event) => match event.code {
                        KeyCode::Enter => {
                            if answer.trim().chars().count() as u16 >= self.minimum_chars {
                                stdout.queue(ResetColor).unwrap();
                                stdout.queue(Clear(ClearType::All)).unwrap();
                                stdout.queue(cursor::MoveTo(0, 0)).unwrap();
                                terminal::disable_raw_mode().unwrap();

                                return Some(answer.trim().to_string());
                            }
                        }
                        KeyCode::Esc => {
                            quit = true;
                        }
                        KeyCode::Left => {
                            if cursor_pos > message_len {
                                stdout.queue(cursor::MoveLeft(1)).unwrap();
                            }
                        }
                        KeyCode::Right => {
                            let whole_text = message_len + answer.chars().count() as u16;
                            let cursor_in_end_of_text = cursor_pos == whole_text;

                            if !cursor_in_end_of_text {
                                stdout.queue(cursor::MoveRight(1)).unwrap();
                            }
                        }
                        KeyCode::Backspace | KeyCode::Delete => {
                            let cursor_in_begin_of_answer = cursor_pos == message_len;

                            if !answer.is_empty() && !cursor_in_begin_of_answer {
                                self.delete_char(&mut answer);
                                stdout.queue(cursor::MoveLeft(1)).unwrap();
                                stdout.queue(cursor::SavePosition).unwrap();
                                self.draw_text(&mut stdout, &answer);
                                stdout.queue(cursor::RestorePosition).unwrap();
                            }
                        }
                        KeyCode::Char(x) => {
                            if event.modifiers.contains(KeyModifiers::CONTROL) && x == 'c' {
                                quit = true
                            }

                            stdout.queue(cursor::MoveRight(1)).unwrap();
                            stdout.queue(cursor::SavePosition).unwrap();
                            self.insert_char(x, &mut answer);
                            self.draw_text(&mut stdout, &answer);
                            stdout.queue(cursor::RestorePosition).unwrap();
                        }
                        _ => {}
                    },

                    _ => quit = true,
                }
            }
            stdout.flush().unwrap();
            sleep(Duration::from_millis(33))
        }
        stdout.queue(ResetColor).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(cursor::MoveTo(0, 0)).unwrap();
        terminal::disable_raw_mode().unwrap();

        None
    }

    fn draw_text(&self, stdout: &mut impl Write, user_input: &str) {
        stdout.queue(cursor::MoveTo(0, 0)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();

        stdout.queue(SetForegroundColor(Self::COLOR)).unwrap();
        stdout.write_all(self.message.as_bytes()).unwrap();
        stdout.queue(ResetColor).unwrap();
        stdout.queue(cursor::MoveRight(1)).unwrap();
        stdout.write_all(user_input.as_bytes()).unwrap();
    }

    fn insert_char(&self, ch: char, answer: &mut String) {
        let space = 1;
        let message_len = (self.message.chars().count() + space) as u16;

        let cursor_pos = crossterm::cursor::position().unwrap().0;

        let char_before_cursor = (cursor_pos - message_len).saturating_sub(1);

        answer.insert(char_before_cursor as usize, ch);
    }

    fn delete_char(&self, user_input: &mut String) {
        let space = 1;
        let message_len = self.message.chars().count() + space;

        let cursor_pos = crossterm::cursor::position().unwrap().0 as usize;

        let char_before_cursor = cursor_pos - message_len - 1;

        user_input.remove(char_before_cursor);
    }
}
