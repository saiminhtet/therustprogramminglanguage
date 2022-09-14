// use std::io::{self, stdout, Write};
//use termion::event::Key;
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;
use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Position {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
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
                break;
            }
        }
    }

    pub fn default() -> Self {
        // Editor{}
        // Self { should_quit: false }
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position {x: 0, y: 0},
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // print!("\x1b[2J");
        // print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(0,0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye. \r");
        } else {
            self.draw_rows();
            // print!("{}", termion::cursor::Goto(1, 1));
            Terminal::cursor_position(0,0);
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
            _ => (),
        }
        Ok(())
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
    fn draw_rows(&self) {
        // for _ in 0..24 {
        //     println!("~\r");
        // }
        let height = self.terminal.size().height;
        // for _ in 0..self.terminal.size().height - 1 {
        //     Terminal::clear_current_line();
        //     println!("~\r");
        // }
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            println!("~\r");
            if row == height / 3 {
                // println!("Hector editor -- version {}\r", VERSION)
                // let welcome_message = format!("Hecto editor -- version {}", VERSION);
                // let width = std::cmp::min(self.terminal.size().width as usize, welcome_message.len());
                // println!("{}\r", &welcome_message[..width])
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
