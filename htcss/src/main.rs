mod dom;
mod parser;
mod stylesheet;

use parser::Parser;

fn main() {
    let html = String::from(
        "\
            // hello
            <p size='12px'>hello</p>
            <p size='12px'>hello</p>
        ",
    );
    let parsed = Parser::parse(html);
    println!("{:#?}", parsed);
}
