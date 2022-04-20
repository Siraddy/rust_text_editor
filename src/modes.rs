//comment for first github commit
use std::collections::HashMap;

//Mode Struct
//this struct stores the current operating mode of the text editor (read, edit, write, quit) as well as the key mappings for special instructions.
//by default the key mappings are the ones in the main editor.rs function "process key" and "move cursor"... those will be moved into this file eventually
//and will be able to be user configurable
struct Mode {
    editor_mode     : u32,
    editor_key_maps : HashMap<String, char>,
}

impl Mode {

    //A preliminary default method which initializes a variable of type Mode
    pub fn default() -> Self {
        return Mode {
            editor_mode     : 0,
            editor_key_maps : HashMap::new(),
        }
    }


}