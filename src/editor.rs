extern crate rustbox;
extern crate time;

use time::Duration;
use std::default::Default;
use rustbox::{Color, RustBox, Key, Event};

pub struct Editor {
    cursor_x: usize,
    cursor_y: usize,
    cursor_life_ms: i32,
    cursor_visible: bool,

    quit: bool,

    text: Vec<Vec<char>>, //Sorted by lines.

    ui: RustBox,
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
            text: vec![vec![]], 
            ui: rb,
        }
    }

    /// Checks for an event, and applies relevant changes to self.  
    /// If there is none, do nothing.
    pub fn get_events(&mut self) {
        // Again, this doesn't HAVE to be 3 milliseconds.
        // But it's a sane enough default for now.
        // TODO
        match self.ui.peek_event(Duration::milliseconds(3), false) {
            Ok(Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Esc) => { 
                        self.quit = true;
                    },
                    
                    Some(Key::Backspace) => { 
                        self.backspace();
                    },

                    Some(Key::Enter) => {
                        self.newline();
                    },

                    Some(Key::Tab) => {
                        self.tab();
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
                    },


                    _ => {}
                }
            },
            Err(e) => panic!("get_events error: {}", e),
            _ => {}
        }
    }

    /// Moves the cursor forward, moving it down a line if it passes an \n
    fn cursor_fwd(&mut self) {
        let max_x = self.text[self.cursor_y].len();
        let max_y = self.text.len() - 1;
        
        // Basically, don't move down past the last line.
        match (self.cursor_x < max_x, self.cursor_y < max_y) {
            (true, _) => {
                self.cursor_x += 1;
            },

            (false, true) => {
                self.cursor_x = 0;
                self.cursor_y += 1;
            },

            (false, false) => {
                return;
            },
        }

    }

    /// Moves the cursor backward, moving it down a line if it passes an \n
    fn cursor_back(&mut self) {
        let min_x = 0;
        let min_y = 0;
        
        match (self.cursor_x > min_x, self.cursor_y > min_y) {
            (true, _) => {
                self.cursor_x -= 1;
            },

            (false, true) => {
                self.cursor_y -= 1;
                self.cursor_x = self.text[self.cursor_y].len();
            },

            (false, false) => {
                return;
            },
        }
    }

    /// Moves the cursor up a line, keeping its x value if possible
    fn cursor_up(&mut self) {
        let min_y = 0;
        
        // Basically, don't move up past the first line.
        match self.cursor_y > min_y {
            true => {
                self.cursor_y -= 1;
                let max_x = self.text[self.cursor_y].len();
                if self.cursor_x > max_x {
                    self.cursor_x = max_x;
                }
            },

            false => {
                return;
            },
        }
    }

    /// Moves the cursor down a line, keeping its x value if possible
    fn cursor_down(&mut self) {
        let max_y = self.text.len() - 1;

        match self.cursor_y < max_y {
            true => {
                self.cursor_y += 1;
                let max_x = self.text[self.cursor_y].len();
                if self.cursor_x > max_x {
                    self.cursor_x = max_x;
                }
            },

            false => {
                return;
            },
        }
    }

    /// Removes the character before the cursor. Compare code with cursor_back
    fn backspace(&mut self) {
        match (self.cursor_x == 0, self.cursor_y == 0) {
            (true, true) => return,
            (true, false) => { self.cursor_back() },
            _ => {
                self.cursor_back();
                self.text[self.cursor_y].remove(self.cursor_x);
            },
        }
    }

    fn type_char(&mut self, c: char) {
        self.text[self.cursor_y].insert(self.cursor_x, c);
        self.cursor_fwd();
    }

    fn newline(&mut self) {
        self.text.insert(self.cursor_y + 1, vec![]);
        self.cursor_down();
    }
    
    fn tab(&mut self) {
        for i in 0..4 { 
            self.text[self.cursor_y].insert(self.cursor_x, ' ');
            self.cursor_fwd();
        }
    }

    /// Takes the character data, and prints it to the screen.
    pub fn write(&self) {
        self.ui.present();
        self.ui.clear();

        self.ui.set_cursor(self.cursor_x as isize, self.cursor_y as isize);

        // These values represent which spot we're currently writing on.
        // They have nothing to do with the cursor.
        let mut x = 0;
        let mut y = 0;

        for line in self.text.clone() {
            for c in line {
                self.ui.print_char(x, y, 
                                   rustbox::RB_NORMAL,
                                   Color::Default,
                                   Color::Default,
                                   c);
                x += 1;
            }
            x = 0;
            y += 1;
        }

    }

    /// Sees if the "quit" flag has been tripped.
    pub fn quit(&self) -> bool { self.quit }

    /// Sets the banner, or the bottom row of the screen, to display a message
    /// The three messages will be right, center, and left aligned respectively.
    pub fn set_banner(&mut self, r_msg: String, c_msg: String, l_msg: String) {
        let bot = self.ui.height() - 1;
        let width = self.ui.width();

        self.ui.print(0, bot,
                      rustbox::RB_NORMAL,
                      Color::Default,
                      Color::Default,
                      &r_msg);

        self.ui.print(width/2, bot,
                      rustbox::RB_NORMAL,
                      Color::Default,
                      Color::Default,
                      &c_msg);

        self.ui.print(width - l_msg.len(), bot,
                      rustbox::RB_NORMAL,
                      Color::Default,
                      Color::Default,
                      &l_msg);
    }



}


