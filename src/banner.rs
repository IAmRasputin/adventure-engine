extern crate rustbox;
extern crate time;

use time::{Duration, SteadyTime};
use rustbox::{Color, RustBox};

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
    msg: String,

    visible: Visible,
}

impl Banner {
    pub fn new(left: String, right: String) -> Banner {
        Banner {
            default_l: left,
            default_r: right,
            msg_timeout: SteadyTime::now(),
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

                if SteadyTime::now() - self.msg_timeout > Duration::seconds(4) {
                    self.visible = Visible::Default;
                }
                
            },

            _ => {},
        }
    }

    pub fn message(&mut self, msg: String) {
        self.msg_timeout = SteadyTime::now();
        self.msg = msg;
        self.visible = Visible::Msg;
    }
}
