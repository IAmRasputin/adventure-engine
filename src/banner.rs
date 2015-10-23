extern crate rustbox;
extern crate time;

use Time::Duration;

struct Banner {
    default_l: String,
    default_r: String,

    msg_ms: Duration,
}

impl Banner {
    /// Sets the banner, or the bottom row of the screen, to display a message
    /// The two messages will be right and left aligned respectively.
    fn set_banner_default(&mut self, r_msg: String, l_msg: String) {
        let bot = self.ui.height() - 1;
        let width = self.ui.width();

        self.ui.print(0, bot,
                      rustbox::RB_NORMAL,
                      Color::Default,
                      Color::Default,
                      &r_msg);


        self.ui.print(width - l_msg.len(), bot,
                      rustbox::RB_NORMAL,
                      Color::Default,
                      Color::Default,
                      &l_msg);
    }
}
