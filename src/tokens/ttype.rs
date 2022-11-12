#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TType {
    Encoding,
    #[allow(dead_code)]
    Comment, 
    String,
    Name,
    Number,
    Op,
    Newline,
    NL,
    Indent,
    Dedent,
    Async,
    Await,
    // TODO; add support for these
    #[allow(dead_code)]
    FStringStart,
    #[allow(dead_code)]
    FStringString,
    #[allow(dead_code)]
    FStringEnd,
    EndMarker,
}

#[cfg(test)]
mod test {
    use super::TType;

    #[test]
    fn basic() {
        let a = TType::Name;
        let b = TType::Name;
        assert_eq!(a, b);
    }
}