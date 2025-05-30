#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    // info: this string is only included if the tokenKind contains a string
    pub value: Option<String>,
    pub kind: TokenKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    EndOfLine,
    EOF,
    // info: char style cannot be greedy, no ] can be inside the style syntax
    CharacterStyle,                 // %[{style syntax}] ==> %\[.*?\]
    MetaData,                       // <meta /> ==> <meta ([^\n]*) />
    UnorderedList,                  // - ==> -\s?
    OrderedList,                    // {number}. ==> ^\d+\.\s?
    Italic,                         // ~ => ~
    Bold,                           // **{any content} ** ==> \*\*[^*]*\*\*
    Definition,                     // @[words] 'expression' ==> @\[.*?\] ?'.*?'
    Heading,                        // #, ##, ###, #### ==> ^\#{1,4}
    BackSlashLeftParenthesisInline, // \( ==> \\\(
    RightParenthesis,               // ) ==> ) \)
    LiteralRightParenthesis,        // \) ==> \\\)
    String,                         // ((?:[^()\\*]|\*(?:[^*]|$))+)
    Link, // &[url] ==> &\[https?:\/\/[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(?:\/[^\s]*)*\]
    Table,
    InlineMath,
    BlockMath,
    HorizontalLine,
}

impl Token {
    pub fn new(kind: TokenKind, value: Option<String>) -> Self {
        Self { kind, value }
    }
}
