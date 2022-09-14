// use std::io::{self, stdout, Read};
// use std::io::{self, stdout};
 // use termion::event::Key;
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;
#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
use editor:: Editor;
pub use terminal::Terminal;
// fn to_ctrl_byte(c: char) -> u8 {
//     let byte = c as u8;
//     byte & 0b0001_1111
// }

// fn die(e: std::io::Error) {
//     panic!("{}", e);
// }



fn main() {
    // let editor = Editor::default();
    // editor.run();
    Editor::default().run();

    // println!("Hello, world!");
    // let _stdout = stdout().into_raw_mode().unwrap();

    // for b in io::stdin().bytes() {
    // let c = b.unwrap() as char;
    // println!("{}", c);
    // let b = b.unwrap();
    // let c = b as char;
    // if c.is_control() {
    //     println!("{:?} \r", b);
    //     println!("{:#b}", b);
    // } else {
    //     println!("{:?} ({})\r", b, c);
    // }

    // if b == to_ctrl_byte('q') {
    //     break;
    // }

    // if c == 'q' {
    //    break;
    // }

    // match b {
    //     Ok(b) => {
    //         let c = b as char;
    //         if c.is_control() {
    //             println!("{:?} \r", b);
    //         } else {
    //             println!("{:?} ({})\r", b, c);
    //         }

    //         if b == to_ctrl_byte('q') {
    //             break;
    //         }
    //     }
    //     Err(err) => die(err),
    // }
    // }

    // for key in io::stdin().keys {
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
    //             _ => println!("{:?}\r",key),
    //         },
    //         Err(err) => die(err),
    //     }
    // }
}
