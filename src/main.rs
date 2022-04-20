mod row;
mod document;
mod editor;
mod terminal;
mod modes;

pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;
use modes::Modes;


// fn cntrl_character(c : char) -> u8 {
//     let byte : u8 = c as u8;
//     return byte & 0b0001_1111;
// }

fn main() {

    let mut editor = Editor::default();
    editor.run();
    //Editor::default().run();
}


