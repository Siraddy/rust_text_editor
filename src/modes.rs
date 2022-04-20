//comment for first github commit
use std::collections::HashMap;
use termion::event::Key;

//Mode Struct
//this struct stores the current operating mode of the text editor (read, edit, write, quit) as well as the key mappings for special instructions.
//by default the key mappings are the ones in the main editor.rs function "process key" and "move cursor"... those will be moved into this file eventually
//and will be able to be user configurable
pub struct Modes {
    editor_mode     : u32,
    editor_key_maps : HashMap<String, Key>,
}

impl Modes {

    //A preliminary default method which initializes a variable of type Mode
    pub fn default() -> Self {
        
        /*Initialize an initial key map to assign to our editor key maps */
        let mut initial_key_map : HashMap<String, Key> = HashMap::new();

        /*Inserting the editor mode key mappings  */
        initial_key_map.insert("Edit".to_string(), Key::Ctrl('e'));
        initial_key_map.insert("Read".to_string(), Key::Ctrl('r'));
        initial_key_map.insert("Write".to_string(), Key::Ctrl('w'));
        initial_key_map.insert("Quit".to_string(), Key::Ctrl('q'));

        /*Inserting the editor in-line cursor movement key mappings */
        initial_key_map.insert("Move-Left".to_string(), Key::Alt('j'));
        initial_key_map.insert("Move-Right".to_string(), Key::Alt('k'));
        initial_key_map.insert("Head-Line".to_string(), Key::Alt('h'));
        initial_key_map.insert("Tail-Line".to_string(), Key::Alt('l'));

        /*Inserting the editor document traversal cursor movement key mappings */
        initial_key_map.insert("Move-Down".to_string(), Key::Alt('d'));
        initial_key_map.insert("Move-Forward".to_string(), Key::Alt('f'));

        /*Inserting the editor document page cursor movement key mappings */
        initial_key_map.insert("Page-Up".to_string(), Key::Alt('v'));
        initial_key_map.insert("Page-Down".to_string(), Key::Alt('n'));

        return Modes {
            editor_mode     : 0,
            editor_key_maps : initial_key_map,
        }
    }


}