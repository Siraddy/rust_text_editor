use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string  : String,
    len     : usize,
}

impl Row {

    pub fn create(slice : &str) -> Self {
        return Self {
            string  : String::from(slice),
            len     : slice.graphemes(true).count(),
        };
    }

    pub fn get_len(&self) -> usize {
        return self.len;
    }

    pub fn is_empty(&self) -> bool {
        if self.len == 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn render(&self, start : usize, end : usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        //return self.string.get(start..end).unwrap_or_default().to_string();
        let mut result = String::new();
        for grapheme in self.string[..].graphemes(true).skip(start).take(end - start) {
            //result.push_str(grapheme);
            if grapheme != "\t" {
                result.push_str(grapheme);
            } else {
                result.push_str("    ");
            }
        }

        return result;
    }

    pub fn insert(&mut self, at : usize, c : char) {
        if at >= self.get_len() {
            self.string.push(c);
            self.len = self.len + 1;
            return;
        }

        let mut result : String = String::new();
        let mut length = 0;

        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length = length + 1;
            if index == at {
                length = length + 1;
                result.push(c);
            }

            result.push_str(grapheme);
        }

        self.len = length;
        self.string = result;
    }

    pub fn delete(&mut self, at : usize) {
        if at >= self.get_len() {
            return;
        }

        let mut result : String = String::new();
        let mut length = 0;

        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != at {
                length = length + 1;
                result.push_str(grapheme);
            }
        }

        self.len = length;
        self.string = result;
    }

    pub fn append(&mut self, new : &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len = self.len + new.len;
    }

    pub fn split(&mut self, at : usize) -> Self {
        let mut row : String = String::new();
        let mut length = 0;

        let mut splitted_row : String = String::new();
        let mut splitted_length = 0;

        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < at {
                length = length + 1;
                row.push_str(grapheme);
            } else {
                splitted_length = splitted_length + 1;
                splitted_row.push_str(grapheme);
            }
        }

        self.string = row;
        self.len = length;

        return Self {
            string  : splitted_row,
            len     : splitted_length,
        };


    }

    pub fn as_bytes(&self) -> &[u8] {
        return self.string.as_bytes();
    }

}

