use crate::dom;
use crate::stylesheet;
use std::collections::HashMap;

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_next_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;

        cur_char
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut ret = String::new();

        while !self.eof() && test(self.next_char()) {
            ret.push(self.consume_next_char());
        }

        ret
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c: char| c.is_whitespace());
    }

    fn consume_comments(&mut self) {
        loop {
            self.consume_whitespace();

            if !self.starts_with("//") {
                break;
            }

            self.consume_while(|c| c != '\n');
        }
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c: char| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut ret = Vec::new();

        loop {
            self.consume_comments();
            self.consume_whitespace();

            if self.eof() || self.starts_with("</") {
                break;
            }

            ret.push(self.parse_node());
        }

        ret
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::Node::text(self.consume_while(|c| c != '<'))
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_next_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c: char| c != open_quote);
        assert!(self.consume_next_char() == open_quote);

        value
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_next_char() == '=');
        let value = self.parse_attr_value();

        (name, value)
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut ret = HashMap::new();

        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            ret.insert(name, value);
        }

        ret
    }

    fn parse_element(&mut self) -> dom::Node {
        assert!(self.consume_next_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_next_char() == '>');

        let children = self.parse_nodes();

        assert!(self.consume_next_char() == '<');
        assert!(self.consume_next_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_next_char() == '>');

        dom::Node::elem(tag_name, attrs, children)
    }

    pub fn parse(html: String) -> dom::Node {
        let mut nodes = Self {
            pos: 0,
            input: html,
        }
        .parse_nodes();

        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            dom::Node::elem("html".into(), HashMap::new(), nodes)
        }
    }

    // --------------------------------------------------------------------
    //                  S  T  Y  L  E  S  H  E  E  T
    // --------------------------------------------------------------------

    fn valid_identifier_char(&self, c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '_' | '-' => true,
            _ => false,
        }
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(|c| !c.is_whitespace() && c != '{')
    }

    fn parse_simple_selector(&mut self) -> stylesheet::SimpleSelector {
        let mut selector = stylesheet::SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };

        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_next_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_next_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.consume_next_char();
                }
                c if self.valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }

        selector
    }

    fn parse_selectors(&mut self) -> Vec<stylesheet::Selector> {
        let mut ret = Vec::new();
        loop {
            ret.push(stylesheet::Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_next_char();
                    self.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c),
            }
        }

        // selectors with id is more specified than (1, 0, 0)
        // selectors with classes, which are more specified than (0, 1, 0)
        // selectors with tag names, which are more specified than (0, 0, n)
        // selctors with universal selector ('*') (not even a stylesheet::Selector)
        ret.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        ret
    }

    fn parse_declaration_value(&self, s: String) -> stylesheet::Value {}

    fn parse_declarations(&mut self) -> Vec<stylesheet::Declaration> {
        let mut ret = Vec::new();
        assert!(self.consume_next_char() == '{');

        let s = self.consume_while(|c| c != '}');
        assert!(self.consume_next_char() == '}');

        s.split(';').for_each(|s| {
            let ss = s.split(':').collect::<Vec<&str>>();
            if let [name, value] = ss.as_slice() {
                ret.push(stylesheet::Declaration {
                    name: String::from(*name),
                    value: self.parse_declaration_value(*value),
                });
            }
        });

        ret
    }

    fn parse_rule(&mut self) -> stylesheet::Rule {
        stylesheet::Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consume_chars() {
        let mut p = Parser {
            pos: 0,
            input: String::from("hehe"),
        };

        assert_eq!(p.next_char(), 'h');
        p.consume_next_char();
        assert_eq!(p.next_char(), 'e');
    }

    #[test]
    fn starts_with() {
        let p = Parser {
            pos: 0,
            input: String::from("hehe"),
        };

        assert!(p.starts_with("h"));
        assert!(p.starts_with("he"));
        assert!(p.starts_with("heh"));
        assert!(p.starts_with("hehe"));
        assert!(p.starts_with("hehee") == false);
    }

    #[test]
    fn consume_comments() {
        let mut p = Parser {
            pos: 0,
            input: String::from(
                "
                // comment boiss
                skipped comment
                ",
            ),
        };

        p.consume_comments();
        assert!(p.starts_with("skipped comment"));
    }
}
