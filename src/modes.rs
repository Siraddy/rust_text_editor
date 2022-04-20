//comment for first github commit
use std::collections::HashMap;
use termion::{event::Key, cursor};

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
    cursor_mode     : u32,
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
            cursor_mode     : 0,
            editor_key_maps : initial_key_map,
        }
    }

    //Get Editor Mode
    //this function returns the current editor mode (currently it will return 0, 1, 2, 3), this can be updated in the future
    pub fn get_editor_mode(&self) -> u32 {
        return self.editor_mode;
    }

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

    //Set Editor Mode
    //this function takes in a keyboard command (for now either - Ctrl-e, Ctrl-r, Ctrl-w, Ctrl-q) and sets the editor mode variable to match
    //first we have to accept keyboard input, check it against the dictionary values, if value matches the editor mode settings, then reset the 
    //editor mode, else do nothing

    pub fn process_hashmap_key_press(&mut self, key : Key) {
        let result = self.get_value_from_map(&key);
        let mut mode    : u32 = self.editor_mode;
        let mut cursor  : u32 = self.cursor_mode;

        match result {
            Some(res) => {
                match res.as_ref() {
                    "Read"  => mode = 0,
                    "Edit"  => mode = 1,
                    "Write" => mode = 2,
                    "Quit"  => mode = 3,

                    "Move-Left"  => cursor = 0,
                    "Move-Right" => cursor = 1,
                    "Head-Line"  => cursor = 2,
                    "Tail-Line"  => cursor = 3,

                    "Move-Down"     => cursor = 4,
                    "Move-Forward"  => cursor = 5,
                    
                    "Page-Up"   => cursor = 6,
                    "Page-Down" => cursor = 7,
                    
                    _ => return,
                }
            }

            None => return,
        }

        self.editor_mode = mode;
        self.cursor_mode = cursor;
    }




}