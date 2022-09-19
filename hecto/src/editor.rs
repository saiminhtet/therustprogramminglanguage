// use std::io::{self, stdout, Write};
//use termion::event::Key;
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;
use crate::Document;
use crate::Row;
use crate::Terminal;
use std::env;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    pub fn run(&mut self) {
        // let _stdout = stdout().into_raw_mode().unwrap();

        // for key in io::stdin().keys() {
        //     match key {
        //         Ok(key) => match key {
        //             Key::Char(c) => {
        //                 if c.is_control() {
        //                     println!("{:?}\r", c as u8);
        //                 } else {
        //                     println!("{:?} ({})\r", c as u8, c);
        //                 }
        //             }
        //             Key::Ctrl('q') => break,
        //             _ => println!("{:?}\r", key),
        //         },
        //         Err(err) => die(err),
        //     }
        // }

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
            if self.should_quit {
                Terminal::clear_screen();
                break;
            }
        }
    }

    pub fn default() -> Self {
        // Editor{}
        // Self { should_quit: false }
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            //document: Document::default(),
            document,
            // cursor_position: Position { x: 0, y: 0},
            cursor_position: Position::default(),
            offset: Position::default(),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // print!("\x1b[2J");
        // print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
        Terminal::cursor_hide();
        // Terminal::clear_screen();
        // Terminal::cursor_position(0,0);
        // Terminal::cursor_position(&Position { x: 0, y: 0 });
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye. \r");
        } else {
            self.draw_rows();
            // print!("{}", termion::cursor::Goto(1, 1));
            // Terminal::cursor_position(0,0);
            // Terminal::cursor_position(&self.cursor_position);
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        // io::stdout().flush()
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            // Key::Ctrl('q') => panic!("Program end"),
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up 
                | Key::Down 
                | Key::Left 
                | Key::Right 
                | Key::PageUp
                | Key::PageDown
                | Key::End
                | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let size = self.terminal.size();
        // let height= size.height.saturating_sub(1) as usize;
        let height = self.document.len();
        // let width = size.width.saturating_sub(1) as usize;
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            Key::Up => y = y.saturating_sub(1),
            // Key::Down => y = y.saturating_add(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            // Key::Right => x = x.saturating_add(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            },
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x =width;
        }
        self.cursor_position = Position { x, y }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    pub fn draw_row(&self, row: &Row) {
        // let start = 0;
        //let end = self.terminal.size().width as usize;
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    fn draw_rows(&self) {
        // for _ in 0..24 {
        //     println!("~\r");
        // }
        let height = self.terminal.size().height;
        // for _ in 0..self.terminal.size().height - 1 {
        //     Terminal::clear_current_line();
        //     println!("~\r");
        // }
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            // println!("~\r");
            //if row == height / 3 {
            // println!("Hector editor -- version {}\r", VERSION)
            // let welcome_message = format!("Hecto editor -- version {}", VERSION);
            // let width = std::cmp::min(self.terminal.size().width as usize, welcome_message.len());
            // println!("{}\r", &welcome_message[..width])
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {    
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
        }
    }

    // fn read_key() -> Result<Key, std::io::Error> {
    //     loop {
    //         if let Some(key) = io::stdin().lock().keys().next() {
    //             return key;
    //         }
    //     }
    // }

    fn die(e: std::io::Error) {
        // print!("{}", termion::clear::All);
        Terminal::clear_screen();
        panic!("{}", e);
    }
