use core::panic;

// Lets parse Polish Notation Grammar.

/// Create a parser and parses the source into an Ast.
pub fn parse(source: String) -> Option<Ast> {
    let mut parser = Parser::new(source);
    parser.parse()
}

struct Parser {
    col: usize,
    input: String,
}

impl Parser {
    fn new(input: String) -> Self {
        Self { col: 0, input }
    }

    fn peak(&self) -> char {
        //dbg!(self.col);
        self.input.chars().nth(self.col).unwrap()
    }

    fn is_eol(&self) -> bool {
        self.col == self.input.len()
    }

    fn next(&mut self) -> char {
        let ret = self.input.chars().nth(self.col).unwrap();
        self.col += 1;
        ret
    }

    /// Creates a string and consume chars from the input
    /// while each char adheres to the pattern given in the `test`
    /// function.
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut res = String::new();
        while !self.is_eol() && test(self.peak()) {
            res.push(self.next());
        }

        res
    }

    fn consume_number(&mut self) -> String {
        self.consume_while(|c| c.is_numeric())
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn parse(&mut self) -> Option<Ast> {
        let mut ast = None;

        if self.is_eol() {
            return ast;
        }

        self.consume_whitespace();
        //dbg!(&self.input[self.col..]);

        if self.peak().is_numeric() {
            let number = self.consume_number();
            ast = Some(Ast::Value(number.parse().unwrap()));
        } else {
            let operator = match self.next() {
                '+' => Operator::Add,
                '-' => Operator::Subtract,
                '*' => Operator::Multiply,
                '/' => Operator::Divide,
                _ => panic!("Aiyo"),
            };

            ast = Some(Ast::Expression(Expr {
                operator,
                children: vec![self.parse().unwrap(), self.parse().unwrap()],
            }));
        }

        //dbg!(&ast);
        ast
    }
}

// 8
// Ast::Value(8) -> return 8
//
//
//  + 3 5
//      Ast::Expression(Expr {
//        operator: Operator::Add
//        children: [
//          Ast::Value(3),
//          Ast::Value(5),
//        ]})        -> return 8
//
//
//  + + 3 5 3
//      Ast::Expression(Expr {
//        operator: Operator::Add
//        children: [
//          Ast::Expression(Expr {
//              operator: Operator::Add,
//              children: [
//                Ast::Value(3),
//                Ast::Value(5),
//              ]
//          }),                     -> return 8
//          Ast::Value(3),
//        ]})                       -> return 11

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct Expr {
    operator: Operator,
    children: Vec<Ast>,
}

#[derive(Debug)]
pub enum Ast {
    Value(isize),
    Expression(Expr),
}

impl Ast {
    pub fn eval(&self) -> isize {
        let expr = match self {
            Ast::Expression(expr) => expr,
            Ast::Value(n) => {
                return *n;
            }
        };
        let mut operands = expr.children.iter().map(|n| n.eval());

        match expr.operator {
            Operator::Add => operands.sum(),
            Operator::Subtract => operands.next().unwrap() - operands.next().unwrap(),
            Operator::Multiply => operands.next().unwrap() * operands.next().unwrap(),
            Operator::Divide => operands.next().unwrap() / operands.next().unwrap(),
        }
    }
}
