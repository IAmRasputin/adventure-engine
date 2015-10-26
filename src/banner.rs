extern crate rustbox;
extern crate time;

use time::{Duration, SteadyTime};
use rustbox::{Color, RustBox, Event, Key};

enum Visible {
    Default,
    Msg,
    Input,
    Hidden,
}

pub struct Banner {
    default_l: String,
    default_r: String,

    msg_timeout: SteadyTime,
    duration_ms: Duration,
    msg: String,

    visible: Visible,
}

impl Banner {
    pub fn new(left: String, right: String) -> Banner {
        Banner {
            default_l: left,
            default_r: right,
            msg_timeout: SteadyTime::now(),
            duration_ms: Duration::milliseconds(0),
            msg: String::new(),
            visible: Visible::Default,
        }
    }

    /// Sets the banner, or the bottom row of the screen, to display a message
    /// The two messages will be right and left aligned respectively.
    pub fn display_banner(&mut self, ui: &mut RustBox) {
        let bot = ui.height() - 1;
        let width = ui.width();

        //if self.visible == Visible::Msg &&
        //   SteadyTime::now() - self.msg_timeout > Duration::seconds(4) {
        //    self.visible == Visible::Default;
        //}


        match self.visible {
            Visible::Default => {
                ui.print(0, bot,
                         rustbox::RB_NORMAL,
                         Color::Default,
                         Color::Default,
                         &self.default_l);


                ui.print(width - self.default_r.len(), bot,
                         rustbox::RB_NORMAL,
                         Color::Default,
                         Color::Default,
                         &self.default_r);
            },

            Visible::Msg => {
                ui.print(0, bot,
                         rustbox::RB_NORMAL,
                         Color::Default,
                         Color::Default,
                         &self.msg);

                if SteadyTime::now() - self.msg_timeout > self.duration_ms {
                    self.visible = Visible::Default;
                }
                
            },

            _ => {},
        }
    }

    pub fn message(&mut self, msg: String, duration: i64) {
        self.msg_timeout = SteadyTime::now();
        self.duration_ms = Duration::milliseconds(duration);
        self.msg = msg;
        self.visible = Visible::Msg;
    }

    pub fn input(&mut self, msg: String, ui: &mut RustBox) 
                 -> Result<String, String> {
        let bot = ui.height() - 1;
        let mut x = msg.len();

        ui.print(0, bot,
                 rustbox::RB_NORMAL,
                 Color::Default,
                 Color::Default,
                 &msg);

        // This isn't part of the normal input/clear/write loop, so we have to
        // clear the banner manually, so we don't clear the entire screen by
        // accident.
        for i in x..ui.width() {
            ui.print_char(i, bot,
                          rustbox::RB_NORMAL,
                          Color::Default,
                          Color::Default,
                          ' ');
        }

        let mut done = false;
        let mut response = String::new();
        let mut out : Result<String, String> = Err("Unknown error".to_string());

        while !done {
            ui.set_cursor(x as isize, bot as isize);
            ui.print(msg.len(), bot,
                     rustbox::RB_NORMAL,
                     Color::Default,
                     Color::Default,
                     &response);
            ui.present();

            match ui.poll_event(false) {
                Ok(Event::KeyEvent(key)) => {
                    match key {
                        Some(Key::Esc) => {
                            out =  Err("Aborted".to_string());
                            done = true;
                        },

                        Some(Key::Enter) => {
                            out = Ok(response.clone());
                            done = true;
                        },

                        Some(Key::Backspace) => {
                            response.remove(x - msg.len());
                            x -= 1;
                        },

                        Some(Key::Char(c)) => {
                            response.insert(x - msg.len(), c);
                            x += 1;
                        },

                        Some(Key::Left) => {
                            if x > msg.len() {
                                x -= 1;
                            }
                        },
                        
                        Some(Key::Right) => {
                            if x < msg.len() + response.len() {
                                x += 1;
                            }
                        },

                        _ => {},
                    }
                },

                Err(e) => {
                    out = Err("Error getting key event".to_string());
                    done = true;
                },
                _ => {},
            }
        }

        out
    }
}
