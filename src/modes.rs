//comment for first github commit
use std::collections::HashMap;
use termion::event::Key;

//Mode Struct
//this struct stores the current operating mode of the text editor (read, edit, write, quit) as well as the key mappings for special instructions.
//by default the key mappings are the ones in the main editor.rs function "process key" and "move cursor"... those will be moved into this file eventually
//and will be able to be user configurable
//
//the various editor modes are:
//0 - read
//1 - edit
//2 - write
//3 - quit
pub struct Modes {
    editor_mode     : u32,
    editor_key_maps : HashMap<Key, String>,
}

impl Modes {

    //A preliminary default method which initializes a variable of type Mode
    pub fn default() -> Self {
        
        /*Initialize an initial key map to assign to our editor key maps */
        let mut initial_key_map : HashMap<Key, String> = HashMap::new();

        /*Inserting the editor mode key mappings  */
        initial_key_map.insert(Key::Ctrl('e'), "Edit".to_string());
        initial_key_map.insert(Key::Ctrl('r'), "Read".to_string());
        initial_key_map.insert(Key::Ctrl('w'), "Write".to_string());
        initial_key_map.insert(Key::Ctrl('q'), "Quit".to_string());

        /*Inserting the editor in-line cursor movement key mappings */
        initial_key_map.insert(Key::Alt('j'), "Move-Left".to_string());
        initial_key_map.insert(Key::Alt('k'), "Move-Right".to_string());
        initial_key_map.insert(Key::Alt('h'), "Head-Line".to_string());
        initial_key_map.insert(Key::Alt('l'), "Tail-Line".to_string());

        /*Inserting the editor document traversal cursor movement key mappings */
        initial_key_map.insert(Key::Alt('d'), "Move-Down".to_string());
        initial_key_map.insert(Key::Alt('f'), "Move-Forward".to_string());

        /*Inserting the editor document page cursor movement key mappings */
        initial_key_map.insert(Key::Alt('v'), "Page-Up".to_string());
        initial_key_map.insert(Key::Alt('n'), "Page-Down".to_string());

        return Modes {
            editor_mode     : 0,
            editor_key_maps : initial_key_map,
        }
    }


}