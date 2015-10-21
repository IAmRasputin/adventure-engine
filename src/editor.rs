extern crate rustbox;
extern crate time;

use time::Duration;
use std::default::Default;
use rustbox::{Color, RustBox, Key, Event};

pub struct Editor {
    cursor_x: i32,
    cursor_y: i32,
    cursor_life_ms: i32,
    cursor_visible: bool,

    quit: bool,

    text: Vec<char>,

    ui: RustBox
}

impl Editor {
    pub fn new() -> Editor {
        let rb = match RustBox::init(Default::default()) {
            Ok(x) => x,
            Err(e) => panic!("Editor failed to load: {}", e),
        };

        // I could probably take these in as params, but the defaults
        // should work for now.
        // TODO
        Editor {
            cursor_x: 0,
            cursor_y: 0,
            cursor_life_ms: 1000,
            cursor_visible: true,
            quit: false, // Except for this one.  This needs to be false.
            text: vec![], 
            ui: rb,
        }
    }

    /// Checks for an event, and applies relevant changes to self.  
    /// If there is none, do nothing.
    pub fn get_events(&mut self) {
        // Again, this doesn't HAVE to be 3 milliseconds.
        // But it's a sane enough default for now.
        // TODO
        match rustbox.peek_event(Duration::milliseconds(3), false) {
            Ok(Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Esc) => { 
                        self.quit = true;
                    },
                    
                    Some(Key::Backspace) => { 
                        self.backspace();
                    },

                    Some(Key::Enter) => {
                        self.type_char('\n');
                    },


                    Some(Key::Up) => {
                        self.cursor_up();
                    },
                    Some(Key::Down) => {
                        self.cursor_down();
                    },
                    Some(Key::Left) => {
                        self.cursor_back();
                    },
                    Some(Key::Right) => {
                        self.cursor_fwd();
                    },
                    

                    Some(Key::Char(c)) => { 
                        self.type_char(c);
                    }
                    _ => {}
                }
            },
            Err(e) => panic!("get_events error: {}", e),
            _ => {}
        }
    }

    /// Translates the current cursor location into an index in "self.text"
    fn cursor_to_index(&self) -> i32 {
        let mut i = 0;
        let mut x = 0;
        let mut y = 0;

        for c in self.text {
            if (x, y) == (self.cursor_x, self.cursor_y) {
                break;
            } else {
                if c == '\n' {
                    y += 1;
                    x = 0;
                } else if c == '\t' {
                    x += 4;
                } else {
                    x += 1;
                }

                i += 1;
            }
        }

        i
    }

    /// Translates an index to the on-screen x,y position
    fn index_to_cursor(&self, index: i32) -> (i32, i32) {
        let mut i = 0;
        let mut x = 0;
        let mut y = 0;

        for c in self.text {
           if i < index { 
               if c == '\n' {
                   y += 1;
                   x = 0;
               } else if c == '\t' {
                   x += 4;
               } else {
                   x += 1;
               }

               i += 1;
           }
        }

        (x, y)
    }

    /// Moves the cursor forward, moving it down a line if it passes an \n
    fn cursor_fwd(&mut self) {
        let index = self.cursor_to_index();

        // Make sure we don't overstep our boundaries.
        if index+1 == self.text.len() {
            return;
        }

        if self.text[index + 1] == '\n' {
            self.cursor_x = 0;
            self.cursor_y += 1;
        } else {
            self.cursor_x += 1;
        }
    }

    /// Moves the cursor backward, moving it down a line if it passes an \n
    fn cursor_back(&mut self) {
        let index = self.cursor_to_index() - 1;

        // Make sure we don't overstep our boundaries.
        if index <= 0 {
            return;
        } else {
            let (x, y) = self.index_to_cursor(index);
            (self.cursor_x, self.cursor_y) = (x, y);
        }
    }

    /// Moves the cursor up a line, keeping its x value if possible
    fn cursor_up(&mut self) {
        // Make sure we don't overstep our boundaries.
        if self.cursor_y == 0 {
            return;
        }

        let target_row = self.cursor_y - 1;
        let target_col = self.cursor_x;

        while self.cursor_y != target || self.cursor_x > target_col {
            self.cursor_back();
        }
    }

    /// Moves the cursor down a line, keeping its x value if possible
    /// TODO
    fn cursor_down(&mut self) {
        // Make sure we don't overstep our boundaries.
        if self.cursor_y == 0 {
            return;
        }

        let target_row = self.cursor_y - 1;
        let target_col = self.cursor_x;

        while self.cursor_y != target || self.cursor_x > target_col {
            self.cursor_back();
        }
    }

    fn backspace(&mut self) {
        let index = self.cursor_to_index();
        self.text.remove(index);
    }

    // TODO:
    //      type_char
    //      write
}


