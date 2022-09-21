use regex::Regex;
use std::string::String;
use unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;


#[derive(Clone)]
pub struct CodeLine {
    line: String,
    len: usize,
    pos: usize,
}

impl CodeLine {
    pub fn new(input: String) -> Self {
        Self {
            len: input.len(),
            line: input,
            pos: 0,
        }
    }

    pub fn return_match(&mut self, pattern: Regex) -> Option<(usize, String)> {
        //Return the new cursor position

        //TODO is there a faster/more efficient way to do this?
        let mut remaining: String = self.line.graphemes(true).skip(self.pos).collect();

        if let Some(result) = pattern.find(remaining.as_str()) {
           let retstr = result.as_str().to_string();
            self.pos += retstr.len();
            return Some((self.pos, retstr));
        }
        None

    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.len.saturating_sub(self.pos)
    }

    pub fn get_line(&self) -> String {
        self.line.clone()
    }

    pub fn peek(&self) -> Option<&str> {
        return self.line.graphemes(true).nth(self.pos);
    }

    pub fn get(&mut self) -> Option<&str> {
        let retval = self.line.graphemes(true).nth(self.pos);
        self.pos = self.pos + 1;
        return retval;
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

        assert_eq!(line.peek(), Some("a"));
        assert_eq!(line.get(), Some("a"));
        assert_eq!(line.get(), Some("b"));
        assert_eq!(line.get(), Some("c"));
        assert_eq!(line.get(), Some("\n"));
        assert_eq!(line.get(), None);
        assert_eq!(line.remaining(), 0);




    }

}