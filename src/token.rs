pub enum Token<'a> {
    Eof,
    Ident,
    Colon,
    Int(&'a str),
    Char(char),
    String(&'a str),
    Comma,
    LineComment,
    NewLine,
}