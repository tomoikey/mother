#[derive(Debug)]
pub enum CharType {
    Char(char),
    NewLine,
}

#[derive(Debug, Default)]
pub struct TextBox<const TEXT_LENGTH_LIMIT: usize> {
    lines: [String; 3],
    remains: Vec<CharType>,
    new_line_count: usize,
}

impl<const TEXT_LENGTH_LIMIT: usize> TextBox<TEXT_LENGTH_LIMIT> {
    pub fn new(text: String) -> Self {
        let mut remains = Vec::new();

        let mut size = 0;
        for c in text.chars() {
            match c {
                '\n' => {
                    remains.push(CharType::NewLine);
                    size = 0;
                },
                c if size < TEXT_LENGTH_LIMIT => {
                    remains.push(CharType::Char(c));
                    size += 1;
                }
                c => {
                    remains.push(CharType::NewLine);
                    remains.push(CharType::Char(c));
                    size = 0;
                }
            }
        }

        Self {
            remains,
            ..Default::default()
        }
    }

    pub fn next(&mut self) -> Option<&[String; 3]> {
        if self.remains.is_empty() {
            return None;
        }

        match self.remains.remove(0) {
            CharType::Char(c) if self.new_line_count == 0 => {
                self.lines_head_mut().push(c);
            }
            CharType::Char(c) if self.new_line_count == 1 => {
                self.lines_middle_mut().push(c);
            }
            CharType::Char(c) => {
                self.lines_last_mut().push(c);
            }
            CharType::NewLine if self.new_line_count < 2 => {
                self.new_line_count += 1;
                self.next();
            }
            CharType::NewLine => {
                self.shift();
                self.new_line_count += 1;
            }
        }

        Some(&self.lines)
    }

    fn lines_head(&self) -> &str {
        self.lines[0].as_str()
    }

    fn lines_head_mut(&mut self) -> &mut String {
        &mut self.lines[0]
    }

    fn lines_middle(&self) -> &str {
        self.lines[1].as_str()
    }

    fn lines_middle_mut(&mut self) -> &mut String {
        &mut self.lines[1]
    }

    fn lines_last(&self) -> &str {
        self.lines[2].as_str()
    }

    fn lines_last_mut(&mut self) -> &mut String {
        &mut self.lines[2]
    }

    fn shift(&mut self) {
        self.lines[0] = self.lines[1].clone();
        self.lines[1] = self.lines[2].clone();
        self.lines[2].clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_box() {
        let mut text_box = TextBox::<3>::new("aaab\ncccdd\ne".to_string());

        assert_eq!(text_box.next().unwrap(), &["a", "", ""]);
        assert_eq!(text_box.next().unwrap(), &["aa", "", ""]);
        assert_eq!(text_box.next().unwrap(), &["aaa", "", ""]);
        assert_eq!(text_box.next().unwrap(), &["aaa", "b", ""]);
        assert_eq!(text_box.next().unwrap(), &["aaa", "b", "c"]);
        assert_eq!(text_box.next().unwrap(), &["aaa", "b", "cc"]);
        assert_eq!(text_box.next().unwrap(), &["aaa", "b", "ccc"]);
        assert_eq!(text_box.next().unwrap(), &["b", "ccc", ""]);
        assert_eq!(text_box.next().unwrap(), &["b", "ccc", "d"]);
        assert_eq!(text_box.next().unwrap(), &["b", "ccc", "dd"]);
        assert_eq!(text_box.next().unwrap(), &["ccc", "dd", ""]);
        assert_eq!(text_box.next().unwrap(), &["ccc", "dd", "e"]);
    }
}