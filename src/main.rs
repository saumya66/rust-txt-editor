#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() {
    let mut editor: Editor = Editor::new();
    editor.run();
}
