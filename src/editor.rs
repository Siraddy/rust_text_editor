use crate::Document;
use crate::Row;
use crate::Terminal;
use crate::Modes;

use termion::event::Key;
use std::env;
use termion::color;
use std::time::Duration;
use std::time::Instant;

const STATUS_FG_COLOR : color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR : color::Rgb = color::Rgb(239, 239, 239);
const VERSION : &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES : u8 = 3;

#[derive(Default)]
pub struct Position {
    pub x : usize,
    pub y : usize,
}

struct Status_Message {
    text    : String,
    time    : Instant,
}

impl Status_Message {
    fn create(message : String) -> Self {
        return Self {
            time    : Instant::now(),
            text    : message,
        }
    }
}

pub struct Editor {
    should_quit     : bool,
    terminal        : Terminal,
    cursor_position : Position,
    offset          : Position,
    status_message  : Status_Message,  
    document        : Document,
    quit_times      : u8,
    modes           : Modes,
    should_edit     : bool,

}

impl Editor {

    //------------------------------------------------------------------------//
    //------------------ Default Editor Implementation -----------------------//
    //------------------------------------------------------------------------//
    pub fn default() -> Self {

        let args : Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-W = write | HELP: Alt-Q = quit");

        let document = if args.len() > 1 {
            let file_name = &args[1];
            let doc = Document::open(&file_name);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("ERR: could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };

        return Editor{ 
            should_quit     : false,
            terminal        : Terminal::default().expect("Failed to initilize terminal"),
            cursor_position : Position::default(),
            offset          : Position::default(),
            status_message  : Status_Message::create(initial_status),
            document,
            quit_times      : QUIT_TIMES,
            modes           : Modes::default(),
            should_edit     : false,
         };
    }

    //------------------------------------------------------------------------//
    //---------------- Run Text Editor (Wrapper Function_) -------------------//
    //------------------------------------------------------------------------//
    pub fn run(&mut self) {

        loop {
            if let Err(error) = self.refresh_screen() {
                self.die(error);
            }

            if self.should_quit == true {
                break;
            }

            if let Err(error) = self.process_keypress() {
                self.die(error);
            }
        }
    }

    //------------------------------------------------------------------------//
    //-------------------- Screen Management Functions -----------------------//
    //------------------------------------------------------------------------//

    //Welcome Message:
    //This function helps draw the welcome message users see when first opening the text editor (without a document loaded)
    //How this is done is by the following steps:
    //1. get the width of the terminal
    //2. get the length of the welcome message
    //3. to center the welcome message, we must place it such that the middle of the welcome message is in the middle of the terminal
    //4. the padding required to accomplish step 3 is by subtracting the width of the terminal by the length of the message and dividing by 2 
    //   what this does is allow us to have a centered message for *any* message we want as the algorithm to center it will stay the same
    //5. ensure that the message does not exceed the terminal width by truncating it
    fn draw_welcome_message(&self) {
        let mut welcome_message : String = format!("Text Editor -- version {}", VERSION);
        let width   : usize = self.terminal.size().width as usize;
        let len     : usize = welcome_message.len();
        let padding : usize = width.saturating_sub(len) / 2;
        let spaces  : String = " ".repeat(padding.saturating_sub(1));
        
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    //Draw Row
    //This function helps print a single row onto the screen based on the terminal size and the row size. Because the length of the string stored in a single row is not
    //depandant on the size of the terminal, we must *only* print the characters of the row that are visible based on the current position of the terminal (in terms of pan)
    //This means we must calcuate the current start position (based on the offset), and the current end position (offset + width of terminal), and only print the characters
    //within that range.
    fn draw_row(&self, row : &Row) {
        let width   : usize = self.terminal.size().width as usize;
        let start   : usize = self.offset.x;
        let end     : usize = self.offset.x + width;
        let row     : String = row.render(start, end);
        println!("{}\r", row);
    }

    //Draw Rows
    //what this does is take the draw row concept and apply it to the entire terminal (height wise)... 
    //it starts at the top of the terminal and makes its way down, drawing a line on on each available line on the terminal
    //it also takes into accound the current y offset (page offset) so that as a user scrolls, they see the correct data on 
    //terminal. First, the line is cleared, and then the new information is printed... the line to be printed is determined by
    //indexing the document vector at the specified y offset + cursor position 
    fn draw_rows(&self) {
        let height  = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() == true && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    //Draw Status Bar
    //This function draws the status bar at the bottom of the terminal which displays the file-name, dirty status, total lines, and the current line
    //We want the file name, dirty status, and total lines are on the left side vs the current line number on the right. To achieve this we need to 
    //find the length of the left hand string, and then the right hand string, and add padding spaces in between in order to seperate the two.
    //We will also set the background color of the status bar to white in order to make it clear to the user that it is not part of the editor
    fn draw_status_bar(&self) {
        //let spaces = " ".repeat(self.terminal.size().width as usize);
        let mut status          : String;
        let width               : usize = self.terminal.size().width as usize;
        let modified_indicator  : &str = if self.document.is_dirty() == true { "(modified)" } else { "(up to date)" };
        let line_indicator      : String = format!("{}/{}", self.cursor_position.y.saturating_add(1), self.document.len());
        let mut file_name       : String = "[No Name]".to_string();

        /*I don't fully get why its structured like this... */
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }

        status = format!("{} - {} lines {}", file_name, self.document.len(), modified_indicator);
        let len : usize = status.len() + line_indicator.len();

        /*I don't fully get why its structured like this -> why do we need the & for the push string method */
        if width > len {
            status.push_str(&" ".repeat(width - len));
        }

        status = format!("{}{}", status, line_indicator);
        status.truncate(width);

        Terminal::set_fg_color(STATUS_FG_COLOR);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        println!("{}\r", status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    //Draw Message Bar
    //this function draws the HELP message bar when the editor is first started... the message bar is removed after typing (in around 5 seconds) 
    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;

        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }

    //Refresh Screen
    //This function "renders" the screen... essentially the screen you see is not a static until something happens, instead it is continously refreshed
    //even if nothing changes. Thus it draws all the rows of the terminal, followed by the status bar, followed by the message bar... while it is drawing the
    //rows, it hides the cursor, and once it has finished drawing everything it displays the cursor again... this cursor off-on feature is not noticable however
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());

        if self.should_quit == true {
            Terminal::clear_screen();
            println!("Goodbye. \r");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position { x: self.cursor_position.x.saturating_sub(self.offset.x), y: self.cursor_position.y.saturating_sub(self.offset.y) });
        }

        Terminal::cursor_show();
        return Terminal::flush();
    }

    //Scroll
    //this function helps determine which lines of the document to display... as the terminal has a fixed height, as soon as the document has more lines
    //then the terminal can display, a scrolling function is necessary... the way the scroll function works is by checking the document length (number of rows)
    //vs the height of the terminal. It also checks the length of the row vs the width of the terminal... if the entire document or the entire row cannot fit
    //on the terminal, an "offset" is calculated based on the cursor position... if the cursor is moved passed the boundry of the terminal, the offset is update 
    //and the rows rendered on the terminal changed (as we moved the cursor down, the terminal appears to scroll - this is caused by the fact that the rows being renderd
    //change)  
    fn scroll(&mut self) {
        let Position { x, y} = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;

        let mut offset = &mut self.offset;
        
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Save as: ").unwrap_or(None);
            if new_name.is_none() {
                self.status_message = Status_Message::create("Save aborted.".to_string());
                return;
            }

            self.document.file_name = new_name;
        }

        if self.document.save().is_ok() {
            self.status_message = Status_Message::create("File saved successfully".to_string());
        } else {
            self.status_message = Status_Message::create("Error writing file!".to_string())
        }
    }

    //Quit
    //moves the quit logic out of the process keypress function and into its own function. This way we can edit the process keypress (move the quit handling into modes)
    //while retaining the quit logic... this will let us perform the quit-times handling without large, clunky code in an if statement
    fn quit(&mut self) {
        if self.quit_times > 0 && self.document.is_dirty() == true {
            self.status_message = Status_Message::create(format!("WARNING! File has unsaved changes. Press Alt-Q {} more times to quit.", self.quit_times));
            self.quit_times = self.quit_times - 1;
            return;
        }

        self.should_quit = true;
        return;
    }

    fn reset_quit(&mut self) {
        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_message = Status_Message::create(String::new());
        }

        return;
    }

    //------------------------------------------------------------------------//
    //-------------- Detect And Process Key Pressed --------------------------//
    //------------------------------------------------------------------------//
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key : Key = Terminal::read_key()?;

        self.modes.process_hashmap_key_press(&pressed_key);
        let editor_mode : (&String, bool) = self.modes.get_editor_mode();
        let cursor_mode : (&String, bool) = self.modes.get_cursor_mode();

        if editor_mode.1 == true {
            let mode : &str = editor_mode.0.as_str();
            match mode {
                "Edit"  => self.should_edit = true,
                "Read"  => self.should_edit = false,
                "Write" => self.save(),
                "Quit"  => self.quit(),
                _ => (),
            }
            self.move_cursor("Move-Left");  //why are we getting a immutable borrow on cursor mode but not editor mode
        } else if cursor_mode.1 == true {
            let mode : &str = cursor_mode.0.as_str();
            self.move_cursor(mode);
        } else {
            if self.should_edit == true {
                match pressed_key {
                    Key::Char(c) => {
                        self.document.insert(&self.cursor_position, c);
                        self.move_cursor("Move-Right");
                    }
                    Key::Backspace => {
                        if self.cursor_position.y > 0 || self.cursor_position.x > 0 {
                            self.move_cursor("Move-Right");
                            self.document.delete(&self.cursor_position);
                        }
                    }
                    Key::Delete => {
                        if self.cursor_position.x > 0 || self.cursor_position.y >= 0 {
                            self.document.delete(&self.cursor_position);
                        }
                    }
                    _ => (),
                }
            }
        }

        self.scroll();
        self.reset_quit();
        return Ok(());
    }

    // f = move cursor to next line (as in forward a line)
    // d = move cursor to previous line (as in downward a line)
    //
    // j = move cursor to previous character 
    // k = move cursor to next character
    //
    // l = move cursor to end of line (as in end of Line)
    // h = move cursor to begining of line (as in Head of line)
    //
    // Enter = move to next page
    // Backspace = move to previous page

    fn move_cursor(&mut self, mode : &str) {
        let Position{ mut x, mut y} = self.cursor_position;

        let terminal_height = self.terminal.size().height as usize;
        let terminal_width = self.terminal.size().width as usize;

        let height : usize = self.document.len();
        let width = if let Some(row) = self.document.row(y) {
            row.get_len()
        } else {
            0
        };
        
        match mode {
            "Move-Forward" => {
                if y < height {
                    y = y.saturating_add(1);
                }
            } 
            "Move-Down" => y = y.saturating_sub(1),
            "Move-Right" => {
                if x < width {
                    x = x + 1;
                } else if y < height {
                    y = y + 1;
                    x = 0;
                }
            }
            "Move-Left" => {
                if x > 0 {
                    x = x - 1;
                } else if y > 0 {
                    y = y - 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.get_len();
                    } else {
                        x = 0;
                    }
                }
            }
            "Tail-Line" => {
                x = if x.saturating_add(terminal_width) < width {
                    x + terminal_width as usize
                } else {
                    width
                };
            }
            "Head-Line" => {
                x = if x > terminal_width {
                    x - terminal_width
                } else {
                    0
                };
            }
            "Page-Down" => {
                y = if y.saturating_add(terminal_height) < height {
                    y + terminal_height as usize
                } else {
                    height
                };
            }
            "Page-Up" => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                };
            }
            _ => (),
        }

        return self.cursor_position = Position{x, y};
    }

    fn prompt(&mut self, prompt : &str) -> Result<Option<String>, std::io::Error> {
        let mut result = String::new();

        loop {
            self.status_message = Status_Message::create(format!("{}{}", prompt, result));
            self.refresh_screen()?;

            match Terminal::read_key()? {
                Key::Backspace => {
                    if !result.is_empty() {
                        result.truncate(result.len() - 1);
                    }
                }
                Key::Char('\n') => {
                    break;
                }
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => {
                    ();
                }
            }
        }

        self.status_message = Status_Message::create(String::new());
        if result.is_empty() {
            return Ok(None);
        }

        return  Ok(Some(result));
    }

    //------------------------------------------------------------------------//
    //------------------------------ Kill Editor -----------------------------//
    //------------------------------------------------------------------------//
    fn die(&self, e : std::io::Error) {
        Terminal::clear_screen();
        panic!("{}", e);
    }

}



