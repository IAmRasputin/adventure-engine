extern crate rustbox;
extern crate time;

mod editor;

use time::Duration;
use std::default::Default;
use rustbox::{Color, RustBox, Key};
use editor::Editor;


fn main() {
    let mut editor = Editor::new();

    loop {
        rustbox.present();
        rustbox.clear();

        rustbox.set_cursor(cursor_pos.0, cursor_pos.1);

        match rustbox.peek_event(Duration::milliseconds(3), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Esc) => { break; },
                    Some(Key::Backspace) => { 
                         
                    },
                    Some(Key::Up) => {
                        cursor_pos.1 -= 1;
                    },
                    Some(Key::Down) => {
                        cursor_pos.1 += 1;
                    },
                    Some(Key::Left) => {
                        cursor_pos.0 -= 1;
                    },
                    Some(Key::Right) => {
                        cursor_pos.0 += 1;
                    },
                    Some(Key::Char(c)) => { 

                    }
                    _ => {}
                }
            },
            Err(e) => panic!("InputLoop error: {}", e),
            _ => { }
        }
    }

}
