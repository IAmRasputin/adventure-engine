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
        if !editor.quit() {
            editor.get_events();
            editor.write();
        }
    }
}
