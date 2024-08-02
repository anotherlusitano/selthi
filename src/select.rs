use std::{
    cmp::min,
    io::{stdout, Write},
    ops::Div,
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::{self, Hide, SetCursorStyle, Show},
    event::{poll, read, Event, KeyCode, KeyModifiers},
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
#[cfg(feature = "with_images")]
use ueberzug::{Scalers, UeConf, Ueberzug};

/// Prompt suitable for when you need the user to select one option among many.
/// With the option to show images among the options.
///
/// # Example
///
/// ```no_run
/// use selthi::Select;
///
/// let options: Vec<&str> = vec!["Rust", "C", "C++", "Javascript",
///     "Java", "C#", "Python", "Haskell", "Lisp", "Erlang"
/// ];
///
/// let ans: Option<&str> = Select::new("What's your favorite programming language?", options).prompt();
///
/// match ans {
///     Some(language) => println!("{} rocks!", language),
///     None => println!("There was an error, please try again"),
/// }
/// ```
#[derive(Debug)]
pub struct Select<'a> {
    pub options: Vec<&'a str>,
    pub images_path: Option<Vec<&'a str>>,
    pub message: &'a str,
    pub help_message: Option<&'a str>,
    pub page_size: usize,
    pub cursor_index: u16,
    current_option: usize,
    pub vim_mode: bool,
}

impl<'a> Select<'a> {
    pub const COLOR: Color = Color::Yellow;
    pub const DEFAULT_PAGE_SIZE: usize = 7;
    pub const DEFAULT_VIM_MODE: bool = false;
    // The default cursor start at 1
    // because of the message occupies 1 row
    pub const DEFAULT_STARTING_CURSOR: u16 = 1;
    pub const DEFAULT_CURRENT_OPTION: usize = 0;
    pub const DEFAULT_HELP_MESSAGE: Option<&'a str> =
        Some("[Move with the arrows and click enter to select]");

    /// Creates a [Select] with the provided message and options, along with default configuration values.
    pub fn new(message: &'a str, options: Vec<&'a str>) -> Self {
        Self {
            options,
            images_path: None,
            message,
            help_message: Self::DEFAULT_HELP_MESSAGE,
            page_size: Self::DEFAULT_PAGE_SIZE,
            cursor_index: Self::DEFAULT_STARTING_CURSOR,
            current_option: Self::DEFAULT_CURRENT_OPTION,
            vim_mode: Self::DEFAULT_VIM_MODE,
        }
    }

    /// Sets the help message of the prompt.
    pub fn with_help_message(mut self, message: &'a str) -> Self {
        self.help_message = Some(message);
        self
    }

    /// Removes the set help message.
    pub fn without_help_message(mut self) -> Self {
        self.help_message = None;
        self
    }

    /// Sets the page size aka max options show on the screen.
    pub fn with_page_size(mut self, page_size: usize) -> Self {
        self.page_size = page_size;
        self
    }

    /// Enables or disables vim_mode.
    pub fn with_vim_mode(mut self, vim_mode: bool) -> Self {
        self.vim_mode = vim_mode;
        self
    }

    #[cfg(feature = "with_images")]
    /// Sets the images to show when selecting.
    pub fn with_images(mut self, images_path: Vec<&'a str>) -> Self {
        self.images_path = Some(images_path);
        self
    }

    fn draw_options(&mut self, stdout: &mut impl Write, page: (usize, usize)) {
        let mut row: u16 = 0;

        stdout.queue(cursor::MoveTo(0, row)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();

        stdout.queue(SetForegroundColor(Self::COLOR)).unwrap();
        stdout.write_all(self.message.as_bytes()).unwrap();
        stdout.queue(ResetColor).unwrap();

        for option in &self.options[page.0 - 1..page.1] {
            let selected_option = format!("> {}", option);
            let unselected_option = format!("  {}", option);

            row += 1;

            stdout.queue(cursor::MoveTo(0, row)).unwrap();

            if row == self.cursor_index {
                stdout.queue(SetForegroundColor(Self::COLOR)).unwrap();
                stdout.write_all(selected_option.as_bytes()).unwrap();
                stdout.queue(ResetColor).unwrap();
            } else {
                stdout.write_all(unselected_option.as_bytes()).unwrap();
            }
        }

        stdout.queue(cursor::MoveTo(0, row + 1)).unwrap();

        if let Some(help_message) = self.help_message {
            stdout.queue(SetForegroundColor(Self::COLOR)).unwrap();
            stdout.write_all(help_message.as_bytes()).unwrap();
            stdout.queue(ResetColor).unwrap();
        }

        stdout.queue(cursor::MoveTo(0, self.cursor_index)).unwrap();
    }

    #[cfg(feature = "with_images")]
    fn draw_image(&self, ueberzug: &Ueberzug, size: (u16, u16)) {
        match &self.images_path {
            Some(images_path) => {
                let last_image: usize = images_path.len() - 1;
                let previous_option = self.current_option.saturating_sub(1);
                let next_option = self.current_option.saturating_add(1).min(last_image);

                let previous_image = images_path[previous_option];
                let next_image = images_path[next_option];

                let image_path = images_path[self.current_option];
                let (width, height) = size;
                let img_width = width / 2;
                let padding_right = 2;

                ueberzug.clear(previous_image);
                ueberzug.clear(next_image);

                ueberzug.draw(&UeConf {
                    identifier: image_path,
                    path: image_path,
                    x: img_width - padding_right,
                    y: height / 4,
                    width: Some(width / 2),
                    height: Some(height / 2),
                    scaler: Some(Scalers::FitContain),
                    ..Default::default()
                });
            }
            None => (),
        }
    }

    /// Returns the selected option by the user.
    pub fn prompt(mut self) -> Option<&'a str> {
        #[cfg(feature = "with_images")]
        let (mut width, mut height) = terminal::size().unwrap();
        self.page_size = min(self.page_size, self.options.len());
        let mut page = (1, self.page_size);
        let mut stdout = stdout();
        let mut quit = false;

        #[cfg(feature = "with_images")]
        let ueberzug = ueberzug::Ueberzug::new();

        terminal::enable_raw_mode().unwrap();
        stdout.queue(SetCursorStyle::SteadyBar).unwrap();
        stdout.queue(Hide).unwrap();
        self.draw_options(&mut stdout, page);

        #[cfg(feature = "with_images")]
        self.draw_image(&ueberzug, (width, height));

        while !quit {
            while poll(Duration::ZERO).unwrap() {
                match read().unwrap() {
                    #[cfg(feature = "with_images")]
                    Event::Resize(nw, nh) => {
                        width = nw;
                        height = nh;

                        self.draw_image(&ueberzug, (width, height));
                    }
                    Event::Key(event) => {
                        #[cfg(target_os = "windows")]
                        if event.kind != KeyEventKind::Press {
                            break;
                        }

                        match event.code {
                            KeyCode::Up => {
                                self.option_up(&mut stdout, &mut page);
                                self.draw_options(&mut stdout, page);
                            }
                            KeyCode::Char('k') => {
                                if self.vim_mode {
                                    self.option_up(&mut stdout, &mut page);
                                    self.draw_options(&mut stdout, page);
                                }
                            }
                            KeyCode::Down => {
                                self.option_down(&mut stdout, &mut page);
                                self.draw_options(&mut stdout, page);
                            }
                            KeyCode::Char('j') => {
                                if self.vim_mode {
                                    self.option_down(&mut stdout, &mut page);
                                    self.draw_options(&mut stdout, page);
                                }
                            }
                            KeyCode::Enter => {
                                stdout.queue(ResetColor).unwrap();
                                stdout.queue(Show).unwrap();
                                stdout.queue(Clear(ClearType::All)).unwrap();
                                stdout.queue(cursor::MoveTo(0, 0)).unwrap();
                                terminal::disable_raw_mode().unwrap();

                                return Some(self.options[self.current_option]);
                            }
                            KeyCode::Esc => {
                                quit = true;
                            }
                            KeyCode::Char(x) => {
                                if event.modifiers.contains(KeyModifiers::CONTROL) && x == 'c' {
                                    quit = true
                                }
                            }
                            _ => {}
                        }
                    }
                    Event::FocusLost => (),
                    Event::FocusGained => (),
                    _ => quit = true,
                }
                #[cfg(feature = "with_images")]
                self.draw_image(&ueberzug, (width, height));
            }
            stdout.flush().unwrap();
            sleep(Duration::from_millis(33))
        }
        stdout.queue(ResetColor).unwrap();
        stdout.queue(SetCursorStyle::DefaultUserShape).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(cursor::MoveTo(0, 0)).unwrap();
        terminal::disable_raw_mode().unwrap();

        None
    }

    fn option_up(&mut self, stdout: &mut impl Write, page: &mut (usize, usize)) -> &mut Self {
        let top_page = self.cursor_index == 1;
        let top_page_without_first_option = top_page && page.0 != 1;

        if top_page_without_first_option {
            page.0 -= 1;
            page.1 -= 1;
            self.current_option -= 1;
        } else if top_page {
            return self;
        } else {
            self.cursor_index -= 1;
            self.current_option -= 1;
            stdout.queue(cursor::MoveTo(0, self.cursor_index)).unwrap();
        }

        self
    }

    fn option_down(&mut self, stdout: &mut impl Write, page: &mut (usize, usize)) -> &mut Self {
        if self.cursor_index == self.page_size as u16 {
            return self;
        }

        let is_cursor_at_middle_page = self.cursor_index == self.page_size.div(2) as u16;
        let are_there_more_options = page.1 < self.options.len();

        if is_cursor_at_middle_page && are_there_more_options {
            page.0 += 1;
            page.1 += 1;
            self.current_option += 1;
        } else {
            self.cursor_index += 1;
            self.current_option += 1;
            stdout.queue(cursor::MoveTo(0, self.cursor_index)).unwrap();
        }

        self
    }
}
