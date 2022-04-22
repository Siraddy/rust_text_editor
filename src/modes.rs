//comment for first github commit
use std::collections::HashMap;
use termion::{event::Key, cursor};

//Mode Struct
//this struct stores the current operating mode of the text editor (read, edit, write, quit) as well as the key mappings for special instructions.
//by default the key mappings are the ones in the main editor.rs function "process key" and "move cursor"... those will be moved into this file eventually
//and will be able to be user configurable
//
//the various editor modes are:
//0 - read, 1 - edit, 2 - write, 3 - quit
//
//cursor mode on the other hand also stores integers
#[derive(Clone)]
pub struct Modes {
    pub editor_mode     : (String, bool),
    pub cursor_mode     : (String, bool),
    pub editor_key_maps : HashMap<Key, String>,
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
            editor_mode     : ("Read".to_string(), false),
            cursor_mode     : ("".to_string(), false),
            editor_key_maps : initial_key_map,
        }
    }

    //Get Editor Mode
    //this function returns the current editor mode (currently it will return 0, 1, 2, 3), this can be updated in the future

    //Check Key-Maps Key
    //this function returns true or false based on whether a key is present in the hash map or not
    pub fn check_key_in_map(&self, key : &Key) -> bool {
        return self.editor_key_maps.contains_key(key);
    }

    //Get Value From HashMap
    //this function return the value stored against a particular key in the hashmap
    pub fn get_value_from_map(&self, key : &Key) -> Option<&String> {
        if self.check_key_in_map(key) == true {
            return self.editor_key_maps.get(key);
        } else {
            return None;
        }
        
    }

    pub fn get_editor_mode(&mut self) -> (&String, bool) {
        let mode : &String = &self.editor_mode.0;
        let flag : bool = self.editor_mode.1;
        return (mode, flag);
    }

    pub fn get_cursor_mode(&mut self) -> (&String, bool) {
        let mode : &String = &self.cursor_mode.0;
        let flag : bool = self.cursor_mode.1;
        return (mode, flag);
    }

    pub fn set_editor_mode(&mut self, mode : &String, flag : bool) {
        self.editor_mode.0 = mode.to_string();
        self.editor_mode.1 = flag;
    }

    pub fn set_cursor_mode(&mut self, mode : &String, flag : bool) {
        self.cursor_mode.0 = mode.to_string();
        self.cursor_mode.1 = flag;
    }

    //Set Editor Mode and Cursor Mode
    //this function takes in a keyboard command (for now either - Ctrl-e, Ctrl-r, Ctrl-w, Ctrl-q) and sets the editor mode variable to match
    //first we have to accept keyboard input, check it against the dictionary values, if value matches the editor mode settings, then reset the 
    //editor mode, else do nothing
    pub fn process_hashmap_key_press(&mut self, key : &Key) {
        let result: Option<String> = self.get_value_from_map(&key).cloned();

        match result {
            Some(res) => {
                match res.as_str() {
                    "Edit" | "Read" | "Write" | "Quit" => {
                        self.set_editor_mode(&res, true);
                        self.set_cursor_mode(&"None".to_string(), false);

                        return;
                    }

                    "Move-Left" |"Move-Right" | "Head-Line" | "Tail-Line" | "Move-Down" | "Move-Forward" | "Page-Up" | "Page-Down" => {
                        self.set_cursor_mode(&res, true);
                        self.set_editor_mode(&"None".to_string(), false);

                        return;
                    }

                    _ => {
                        ();
                        return;
                    }
                }
            }

            None => {
                ();
                return;
            }
        }
    }

}