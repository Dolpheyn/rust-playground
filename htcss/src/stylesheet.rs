pub struct Stylesheet {
    rules: Vec<Rule>,
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

pub enum Selector {
    Simple(SimpleSelector),
}

type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref selector) = *self;
        let a = selector.id.iter().count();
        let b = selector.tag_name.iter().count();
        let c = selector.class.len();

        (a, b, c)
    }
}

// Simple selector can be:
// tag name (e.g. h1, h2),
// id (e.g. #container),
// class (e.g. .container)
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

pub struct Declaration {
    name: String,
    value: Value,
}

pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    Colorvalue(Color),
}

enum Unit {
    Px,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
