#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TType {
    Encoding,
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
