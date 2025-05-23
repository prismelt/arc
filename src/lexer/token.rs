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
    CharacterStyle, // %[{style syntax}] ==> %\[.*?\]
    MetaData,       // <meta /> ==> <meta ([^\n]*) />
    UnorderedList,  // - ==> -\s?
    OrderedList,    // {number}. ==> ^\d+\.\s?
    Italic,         // ~ => ~
    Bold,           // **{any content} ** ==> \*\*[^*]*\*\*
    // Skip,                        // ' ' ==> \s+, not really a used token
    Definition,                     // @[words] 'expression' ==> @\[.*?\] ?'.*?'
    Heading,                        // #, ##, ###, #### ==> ^\#{1,4}
    BackSlashLeftParenthesisInline, // \( ==> \\\(
    RightParenthesis,               // ) ==> ) \)
    // BackSlashEndOfLine,          //  \ + \n ==> \\[\s]*\n, not really a used token
    LiteralRightParenthesis, // \) ==> \\\)
    String,                  // ((?:[^()\\*]|\*(?:[^*]|$))+)
    Link,                    // &[url] ==> &\[https?:\/\/[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(?:\/[^\s]*)*\]
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

// #[derive(Debug, PartialEq, Clone)]
// pub struct Token {
//     pub value: Option<String>,
//     pub kind: TokenKind,
// }

// #[derive(Debug, PartialEq, Clone)]
// pub enum TokenKind {
//     BackSlashLeftParenthesisInline,
//     LeftAngleBracket,
//     RightAngleBracket,
//     Equal,
//     EOF,
//     ForwardSlash,
//     BackSlash,
//     Octothorpe,
//     Tilde,
//     At,
//     Ampersand,
//     Percent,
//     DoubleAsterisk,
//     DoubleForwardSlash,
//     Comma,
//     Colon,
//     SingleQuote,
//     LeftBracket,
//     RightBracket,
//     LeftParenthesis,
//     RightParenthesis,
//     Hyphen,
//     Skip,
//     String,
//     Identifier(IdentifierKind),
//     EndOfLine,
// }

// #[derive(Debug, PartialEq, Clone)]
// pub enum IdentifierKind {
//     Meta,
// }

// impl IdentifierKind {
//     pub fn new(kind: &str) -> Self {
//         match kind {
//             "meta" => Self::Meta,
//             _ => panic!("Invalid identifier kind: {}", kind),
//         }
//     }
// }

// impl Token {
//     pub fn new(kind: TokenKind, value: Option<String>) -> Self {
//         Self { kind, value }
//     }

//     pub fn debug(&self) {
//         println!("{:?}", self)
//     }
// }
