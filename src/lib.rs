pub use lazy_static::lazy_static;
pub use regex::{bytes::Regex as Bregex, Regex};

#[macro_export]
macro_rules! count {
    ($head:tt, $($tail:tt),*) => {
        { 1 + $crate::count!($($tail),*) }
    };
    ($single:tt) => {
        { 1 }
    };
    () => {
        { 0 }
    };
}

#[macro_export]
macro_rules! exists {
    ($any:tt) => {
        true
    };
    () => {
        false
    };
}

#[macro_export]
macro_rules! or_default {
    ($optional:expr, $default:expr) => {
        $optional
    };
    (, $default:expr) => {
        $default
    };
}

#[macro_export]
macro_rules! byte_lexer {
    ($([$pattern:literal $($token:ident $($lexeme_marker:tt)?)?])+) => {
        use TokenKind::*;
        const PATTERN_COUNT: usize = $crate::count!($($pattern),*);
        const TOKEN_KINDS: [TokenKind; PATTERN_COUNT] = [$(
            $crate::or_default!($($token)?, Ignored)
        ),*];
        const SAVES_LEXEME: [bool; PATTERN_COUNT] = [$(
            $crate::exists!($($($lexeme_marker)?)?)
        ),*];
        $crate::lazy_static!(
            static ref PATTERN_REGEX: [$crate::Bregex; PATTERN_COUNT] = [$($crate::Bregex::new($pattern).unwrap()),*];
        );
        #[derive(Clone, Eq, PartialEq)]
        pub struct Token(pub TokenKind, pub Option<Vec<u8>>);
        impl std::fmt::Debug for Token {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&match self {
                    Self(kind, Some(lexeme)) => format!("Token({:?}, {:?})", kind, lexeme),
                    Self(kind, None) => format!("Token({:?})", kind)
                })
            }
        }
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum TokenKind {
            Ignored,
            $($($token,)?)*
        }
        pub fn lex(mut src: &[u8]) -> Result<Vec<Token>, usize> {
            let mut tokens = Vec::new();
            let original_src_len = src.len();
            'outer: while src.len() > 0 {
                for pattern_i in 0..PATTERN_COUNT {
                    let pattern_regex = &PATTERN_REGEX[pattern_i];
                    if let Some(lexeme_match) = pattern_regex.find(src) {
                        if lexeme_match.start() == 0 {
                            let lexeme_end = lexeme_match.end();
                            if lexeme_end == 0 {
                                return Err(original_src_len - src.len());
                            }
                            let lexeme = &src[..lexeme_end];
                            src = &src[lexeme_end..];
                            let token = Token(TOKEN_KINDS[pattern_i],
                                if SAVES_LEXEME[pattern_i] {
                                   Some(lexeme.to_vec())
                                } else {
                                    None
                                }
                            );
                            tokens.push(token);
                            continue 'outer;
                        }
                    }
                }
                return Err(original_src_len - src.len());
            }
            Ok(tokens)
        }
    };
    ($(($pattern:literal $($token:ident $($lexeme_marker:tt)?)?))+) => {
        $crate::byte_lexer! { $([$pattern $($token $($lexeme_marker)?)?])* }
    };
}

#[macro_export]
macro_rules! lexer {
    ($([$pattern:literal $($token:ident $($lexeme_marker:tt)?)?])+) => {
        use TokenKind::*;
        const PATTERN_COUNT: usize = $crate::count!($($pattern),*);
        const TOKEN_KINDS: [TokenKind; PATTERN_COUNT] = [$(
            $crate::or_default!($($token)?, Ignored)
        ),*];
        const SAVES_LEXEME: [bool; PATTERN_COUNT] = [$(
            $crate::exists!($($($lexeme_marker)?)?)
        ),*];
        $crate::lazy_static!(
            static ref PATTERN_REGEX: [$crate::Regex; PATTERN_COUNT] = [$($crate::Regex::new($pattern).unwrap()),*];
        );
        #[derive(Clone, Eq, PartialEq)]
        pub struct Token(pub TokenKind, pub Option<std::string::String>);
        impl std::fmt::Debug for Token {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&match self {
                    Self(kind, Some(lexeme)) => format!("Token({:?}, {})", kind, lexeme),
                    Self(kind, None) => format!("Token({:?})", kind)
                })
            }
        }
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum TokenKind {
            Ignored,
            $($($token,)?)*
        }
        pub fn lex(mut src: &str) -> Result<Vec<Token>, usize> {
            let mut tokens = Vec::new();
            let original_src_len = src.len();
            'outer: while src.len() > 0 {
                for pattern_i in 0..PATTERN_COUNT {
                    let pattern_regex = &PATTERN_REGEX[pattern_i];
                    if let Some(lexeme_match) = pattern_regex.find(src) {
                        if lexeme_match.start() == 0 {
                            let lexeme = lexeme_match.as_str();
                            let lexeme_end = lexeme.len();
                            if lexeme_end == 0 {
                                return Err(original_src_len - src.len());
                            }
                            src = &src[lexeme_end..];
                            let token = Token(TOKEN_KINDS[pattern_i],
                                if SAVES_LEXEME[pattern_i] {
                                   Some(lexeme.to_string())
                                } else {
                                    None
                                }
                            );
                            tokens.push(token);
                            continue 'outer;
                        }
                    }
                }
                return Err(original_src_len - src.len());
            }
            Ok(tokens)
        }
    };
    ($(($pattern:literal $($token:ident $($lexeme_marker:tt)?)?))+) => {
        $crate::lexer! { $([$pattern $($token $($lexeme_marker)?)?])* }
    };
}
