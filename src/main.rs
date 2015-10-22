extern crate rustbox;
extern crate time;

mod editor;

use time::Duration;
use std::default::Default;
use rustbox::{Color, RustBox, Key};
use editor::Editor;


fn main() {
    let mut editor = Editor::new();

    while !editor.quit() {
        editor.get_events();
        editor.write();
        editor.set_banner("AdventureEngine 0.0.1".to_string(),
                          "".to_string(),
                          "Press ESC to quit".to_string());
    }
}
