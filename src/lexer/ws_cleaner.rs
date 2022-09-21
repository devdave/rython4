use std::fmt::format;

pub fn cleaner(source: String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut iter = source.chars();
    let mut temp = "".to_string();

    while let Some(sym) = iter.next() {

        if sym == '\r' {
            let mut ahead = iter.clone();
            if let Some(peek) = ahead.next() {
                if peek == '\n' {
                    iter = ahead;
                    temp.push(peek);
                    lines.push(temp.clone());
                    temp = "".to_string();
                } else {
                    temp.push('\n');
                    lines.push(temp.clone());
                    temp = "".to_string();
                }
            } else {  //EOF
                temp.push('\n');
                lines.push(temp.clone());
                temp = "".to_string();
            }

        } else if sym == '\n' {
            temp.push(sym);
            lines.push(temp.clone());
            temp = "".to_string();

        } else {
            temp.push(sym);
        }
    }

    if temp.len() > 0 {
        lines.push(format!("{}\n", temp));
    }

    return lines;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn osx() {
        let fixture = "Hello\rWorld\rThis\rIs a Mac string".to_string();
        let result = cleaner(fixture);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "Hello\n".to_string());
        assert_eq!(result[3], "Is a Mac string\n".to_string());

    }

    #[test]
    fn windows() {
        let fixture = "Hello\r\nWorld\r\nThis\r\nIs a windows string".to_string();
        let result = cleaner(fixture);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "Hello\n".to_string());
        assert_eq!(result[3], "Is a windows string\n".to_string());
    }

    #[test]
    fn linux() {
        let fixture = "Hello\nWorld\nThis\nIs a linux string".to_string();
        let result = cleaner(fixture);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "Hello\n".to_string());
        assert_eq!(result[3], "Is a linux string\n".to_string());
    }
}