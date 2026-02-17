#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Plus,
    Minus,
    Left,
    Right,
    Open,
    Close,
    Print,
    Read,

    /// Comment symbol
    Ignored,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: usize,
}

impl Token {
    pub fn new(kind: TokenKind, position: usize) -> Self {
        Self { kind, position }
    }
}

pub struct Lexer<'src> {
    src: &'src str,
    pos: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self { src, pos: 0 }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.src.len() {
            None
        } else {
            let c = self.src.as_bytes()[self.pos];

            let token = match c {
                b'+' => Some(Token::new(TokenKind::Plus, self.pos)),
                b'-' => Some(Token::new(TokenKind::Minus, self.pos)),
                b'<' => Some(Token::new(TokenKind::Left, self.pos)),
                b'>' => Some(Token::new(TokenKind::Right, self.pos)),
                b'[' => Some(Token::new(TokenKind::Open, self.pos)),
                b']' => Some(Token::new(TokenKind::Close, self.pos)),
                b'.' => Some(Token::new(TokenKind::Print, self.pos)),
                b',' => Some(Token::new(TokenKind::Read, self.pos)),
                _ => Some(Token::new(TokenKind::Ignored, self.pos)),
            };
            self.pos += 1;
            token
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_new() {
        let token = Token::new(TokenKind::Plus, 0);
        assert_eq!(token.kind, TokenKind::Plus);
        assert_eq!(token.position, 0);
    }

    #[test]
    fn lexer_lexing() {
        let lexer = Lexer::new("+-<>[]., ");
        let tokens = lexer.collect::<Vec<Token>>();
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Plus, 0),
                Token::new(TokenKind::Minus, 1),
                Token::new(TokenKind::Left, 2),
                Token::new(TokenKind::Right, 3),
                Token::new(TokenKind::Open, 4),
                Token::new(TokenKind::Close, 5),
                Token::new(TokenKind::Print, 6),
                Token::new(TokenKind::Read, 7),
                Token::new(TokenKind::Ignored, 8),
            ]
        );
    }
}
