

pub struct Tokenizer {
    source: Vec<char>,
    pointer: usize
}

const SINGLES: &[char] = &['(', ')', '{', '}',
                           '*', '.', ',', '+',
                           '-', ';'];
const DOUBLES: &[(char, char)] = &[('=', '='), ('!', '='),
                                   ('>', '='), ('<', '='),
                                   ('|', '|'), ('&', '&'),
                                   ('+', '+'), ('-', '-')];

impl Tokenizer {

    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            pointer: 0
        }
    }

    pub fn next_token(&mut self) -> Option<String> {
        match self.next_token_length() {
            Some(len) => {
                let token = String::from_iter(&self.source[self.pointer..self.pointer + len]);
                self.pointer += len;
                Some(token)
            },
            None => None
        }
    }

    fn next_token_length(&self) -> Option<usize> {
        let curr_char_opt = self.source.get(self.pointer);
        if curr_char_opt.is_some() {
            let curr_char = *curr_char_opt.unwrap();

            if let Some(next_char) = self.source.get(self.pointer + 1) {
                if DOUBLES.iter().filter(|p| p.0 == curr_char).any(|p| p.1 == *next_char) {
                    return Some(2);
                }
            }

            if SINGLES.contains(&curr_char) {
                return Some(1);
            }

            if matches!(curr_char, '/') {
                let next_char_opt = self.source.get(self.pointer + 1);
                if next_char_opt.is_some() && *next_char_opt.unwrap() == '/' {
                    let mut count = 1;
                    let mut head_char = self.source.get(self.pointer + 1).unwrap();
                    while !matches!(head_char, '\r' | '\n') && self.pointer + count < self.source.len() {
                        count += 1;
                        head_char = self.source.get(self.pointer + count).unwrap_or(&' ');
                    }
                    return Some(count);
                }
            }

            if matches!(curr_char, '"') {
                let mut count = 1;
                let mut head_char = self.source.get(self.pointer + 1).unwrap();
                while !matches!(head_char, '\r' | '\n' | '"') && self.pointer + count < self.source.len() {
                    count += 1;
                    head_char = self.source.get(self.pointer + count).unwrap_or(&' ');
                }
                if head_char == &'"' {
                    count += 1;
                }

                return Some(count);
            }

            if curr_char.is_ascii_digit() {
                let next_char_opt = self.source.get(self.pointer + 1);
                if next_char_opt.is_some() {
                    let mut count = 1;
                    let mut head_char = self.source.get(self.pointer + 1).unwrap();
                    while head_char.is_ascii_digit() || head_char == &'.' && self.pointer + count < self.source.len() {
                        count += 1;
                        head_char = self.source.get(self.pointer + count).unwrap_or(&' ');
                    }

                    return Some(count);
                }

                return Some(1);
            }

            if curr_char.is_alphabetic() || curr_char == '_' {
                let next_char_opt = self.source.get(self.pointer + 1);
                if next_char_opt.is_some() {
                    let mut count = 1;
                    let mut head_char = self.source.get(self.pointer + 1).unwrap();
                    while head_char.is_ascii_alphanumeric() || head_char == &'_' && self.pointer + count < self.source.len() {
                        count += 1;
                        head_char = self.source.get(self.pointer + count).unwrap_or(&' ');
                    }

                    return Some(count);
                }

                return Some(1);
            }

            Some(1)
         }
        else {
            None
        }
    }
}
