use std::iter::from_fn;

pub struct Lexer {
    pub input: String,
    pub tokens: Vec<Token>,
    current_index: usize,
}

#[derive(Debug, PartialEq)]
enum Token {
    Atom(String),
    BracketAtom(String),
    OpenBranch,
    CloseBranch,
}

#[derive(Debug)]
pub enum LexerError {
    UnknownSymbol(char),
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            tokens: Vec::new(),
            current_index: 0usize,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        from_fn(|| self.tokenize_next().transpose())
            .take_while(|x| x.as_ref().ok().is_some())
            .map(|x| x.unwrap())
            .collect()
    }

    fn tokenize_next(&mut self) -> Result<Option<Token>, LexerError> {
        let current_char = match self.current_symbol() {
            Some(c) => c,
            None => return Ok(None),
        };

        let token = match current_char {
            '(' => Token::OpenBranch,
            ')' => Token::CloseBranch,
            c @ ('B' | 'C' | 'H' | 'N' | 'O' | 'F' | 'S' | 'P' | 'c' | 'n' | 'o' | 's') => {
                Token::Atom(c.clone().to_string())
            }
            '[' => {
                self.current_index += 1;
                let result = self
                    .input
                    .chars()
                    .skip(self.current_index)
                    .take_while(|&c| c != ']')
                    .collect::<String>();
                self.current_index += result.len();
                Token::BracketAtom(result)
            }
            c => return Err(LexerError::UnknownSymbol(c)),
        };
        self.current_index += 1;
        Ok(Some(token))
    }

    fn current_symbol(&self) -> Option<char> {
        self.input.chars().nth(self.current_index)
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.current_index + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_tokenize() {
        let input = "N([C])C(C)O".to_string();
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.tokenize(),
            vec![
                Token::Atom('N'.to_string()),
                Token::OpenBranch,
                Token::BracketAtom('C'.to_string()),
                Token::CloseBranch,
                Token::Atom('C'.to_string()),
                Token::OpenBranch,
                Token::Atom('C'.to_string()),
                Token::CloseBranch,
                Token::Atom('O'.to_string()),
            ]
        );
    }
}
