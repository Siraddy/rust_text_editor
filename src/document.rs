use crate::Position;
use crate::Row;
use std::fs;
use std::io::{Error, Write};


#[derive(Default)]
pub struct Document {
    rows            : Vec<Row>,
    pub file_name   : Option<String>,
    dirty           : bool,
}

impl Document {

    pub fn open(filename : &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows : Vec<Row> = Vec::new();
        
        for value in contents.lines() {
            rows.push(Row::create(value));
        }
        
        return Ok( Self { 
                    rows,
                    file_name   : Some(filename.to_string()),
                    dirty       : false,
                });
    }

    pub fn row(&self, index : usize) -> Option<&Row> {
        return self.rows.get(index);
    }

    pub fn is_empty(&self) -> bool {
        return self.rows.is_empty();
    }

    pub fn len(&self) -> usize {
        return self.rows.len();
    }

    //-------------------------------------------------//
    //-------------- Text Manipulation ----------------//
    //-------------------------------------------------//
    pub fn insert(&mut self, at : &Position, c : char) {
        if at.y > self.len() {
            return;
        }
        self.dirty = true;

        if c == '\n' {
            self.insert_newline(at);
            return;
        }

        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }

    pub fn delete(&mut self, at : &Position) {
        let len = self.len();
        
        if at.y >= len {
            return;
        }

        self.dirty = true;

        if at.x == self.rows.get_mut(at.y).unwrap().get_len() && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
        }
    }

    fn insert_newline(&mut self, at : &Position) {

        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }

        let new_row = self.rows.get_mut(at.y).unwrap().split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }

        return Ok(());
    }

    pub fn is_dirty(&self) -> bool {
        return self.dirty;
    }

}