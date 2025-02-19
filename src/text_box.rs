#[derive(Debug)]
pub enum CharType {
    Char(char),
    NewLine,
    LineHeadSpace,
}

#[derive(Debug, Default)]
pub struct TextBox<const TEXT_LENGTH_LIMIT: usize> {
    lines: [String; 3],
    remains: Vec<CharType>,
    new_line_count: usize,
}

impl<const TEXT_LENGTH_LIMIT: usize> TextBox<TEXT_LENGTH_LIMIT> {
    pub fn new<STRING: Into<String>>(text: STRING) -> Self {
        let mut remains = vec![CharType::Char('◆')];
        let mut size = 0;
        for c in text.into().chars() {
            match c {
                '\n' => {
                    remains.push(CharType::NewLine);
                    remains.push(CharType::Char('◆'));
                    size = 0;
                }
                '|' => {
                    remains.push(CharType::NewLine);
                    remains.push(CharType::LineHeadSpace);
                    size = 0;
                }
                c if size < TEXT_LENGTH_LIMIT => {
                    remains.push(CharType::Char(c));
                    size += 1;
                }
                c => {
                    remains.push(CharType::NewLine);
                    remains.push(CharType::LineHeadSpace);
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
            CharType::Char(c) => {
                if self.new_line_count == 0 {
                    self.lines_head_mut().push(c);
                } else if self.new_line_count == 1 {
                    self.lines_middle_mut().push(c);
                } else {
                    self.lines_last_mut().push(c);
                }
            }
            CharType::LineHeadSpace => {
                if self.new_line_count == 1 {
                    self.lines_middle_mut().push(' ');
                } else {
                    self.lines_last_mut().push(' ');
                }
                self.next();
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

    fn lines_head_mut(&mut self) -> &mut String {
        &mut self.lines[0]
    }

    fn lines_middle_mut(&mut self) -> &mut String {
        &mut self.lines[1]
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
        let mut text_box = TextBox::<3>::new("aaab\ncccdd\ne|f".to_string());

        assert_eq!(text_box.next().unwrap(), &["◆", "", ""]);
        assert_eq!(text_box.next().unwrap(), &["◆a", "", ""]);
        assert_eq!(text_box.next().unwrap(), &["◆aa", "", ""]);
        assert_eq!(text_box.next().unwrap(), &["◆aaa", "", ""]);
        assert_eq!(text_box.next().unwrap(), &["◆aaa", " b", ""]);
        assert_eq!(text_box.next().unwrap(), &["◆aaa", " b", "◆"]);
        assert_eq!(text_box.next().unwrap(), &["◆aaa", " b", "◆c"]);
        assert_eq!(text_box.next().unwrap(), &["◆aaa", " b", "◆cc"]);
        assert_eq!(text_box.next().unwrap(), &["◆aaa", " b", "◆ccc"]);
        assert_eq!(text_box.next().unwrap(), &[" b", "◆ccc", ""]);
        assert_eq!(text_box.next().unwrap(), &[" b", "◆ccc", " d"]);
        assert_eq!(text_box.next().unwrap(), &[" b", "◆ccc", " dd"]);
        assert_eq!(text_box.next().unwrap(), &["◆ccc", " dd", ""]);
        assert_eq!(text_box.next().unwrap(), &["◆ccc", " dd", "◆"]);
        assert_eq!(text_box.next().unwrap(), &["◆ccc", " dd", "◆e"]);
        assert_eq!(text_box.next().unwrap(), &[" dd", "◆e", ""]);
        assert_eq!(text_box.next().unwrap(), &[" dd", "◆e", " f"]);
    }
}
