
use regex::Regex;
use std::string::String;

use unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;


#[derive(Clone)]
pub struct CodeLine {
    pub line: String,
    pub text: Vec<char>,
    len: usize,
    pos: usize,
}

impl CodeLine {
    pub fn new(input: String) -> Self {
        let temp = Self {
            len: input.clone().chars().collect::<Vec<_>>().len(),
            text: input.clone().chars().collect::<Vec<_>>(),
            line: input,
            pos: 0,
        };


        //assert_eq!(temp.text.len(), temp.line.len());
        return temp;
    }

    pub fn return_match(&mut self, pattern: Regex) -> Option<(usize, String)> {
        //Return the new cursor position

        //TODO is there a faster/more efficient way to do this?
        if self.pos <= self.text.len() {
            let remaining: String = self.text.iter().skip(self.pos).collect();

            if let Some(result) = pattern.find(remaining.as_str()) {
               let retstr = result.as_str().to_string();
                self.pos += retstr.len();
                return Some((self.pos, retstr));
            }

        } else {
            return None;
        }

        // let remaining: String = self.line.graphemes(true).skip(self.pos).collect();


        None

    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.len.saturating_sub(self.pos)
    }


    pub fn rewind(&mut self) {
        self.pos = self.pos.saturating_sub(1);
    }


    pub fn peek_char(&mut self) -> Option<char> {
        if self.text.len() > self.pos {
            return Some(self.text[self.pos]);
        } else {
            return None;
        }

        // return self.line[self.pos..].chars().nth(0);
    }

    pub fn peek_ahead_char(&mut self, skip: usize) -> Option<char> {
        if self.pos + skip < self.len {
            return self.line[(self.pos+skip)..].chars().nth(0);
        }

        None
    }

    pub fn get_char(&mut self) -> Option<char> {
        let result = self.peek_char();
        self.pos = self.pos.saturating_add(1);
        return result;
    }


}

#[cfg(test)]
mod test {


    use super::*;

    #[test]
    fn basic() {
        let line = CodeLine::new("Hello World\n".to_string());

        assert_eq!(line.remaining(), 12);
    }

    #[test]
    fn collect_numbers() {
        let mut line = CodeLine::new("12345abc\n".to_string());
        let re = Regex::new(r"\A\d+").expect("regex");
        let outcome = line.return_match(re);
        assert!(outcome != None);

        if let Some((new_pos, retval)) = outcome {
            assert_eq!(new_pos, 5);
            assert_eq!(retval, "12345");
        } else {
            panic!("Failed to match numbers!");
        }

    }

    #[test]
    fn collect_numbers_and_then_peek() {
        let mut line = CodeLine::new("12345abc\n".to_string());
        let re = Regex::new(r"\A\d+").expect("regex");
        let outcome = line.return_match(re);
        assert!(outcome != None);

        assert_eq!(outcome, Some((5 as usize, "12345".to_string())));

        assert_eq!(line.peek_char(), Some('a'));
        assert_eq!(line.get_char(), Some('a'));
        assert_eq!(line.get_char(), Some('b'));
        assert_eq!(line.get_char(), Some('c'));
        assert_eq!(line.get_char(), Some('\n'));
        assert_eq!(line.get_char(), None);
        assert_eq!(line.remaining(), 0);




    }

}