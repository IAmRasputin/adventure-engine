extern crate rustbox;
extern crate time;

mod editor;
mod banner;

use editor::Editor;

fn main() {
    let mut editor = Editor::new();

    while !editor.quit() {
        editor.get_events();
        editor.write();
    }
}
