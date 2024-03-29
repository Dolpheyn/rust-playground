mod eval;
mod parse;
mod repl;

use crossterm::{terminal, Result};
use eval::eval;
use parse::parse;
use repl::{get_input, print_eval, print_prompt, print_ver, ReplInput};
use std::io::{self, Stdout};

fn run(stdout: &mut Stdout) -> Result<()> {
    terminal::enable_raw_mode()?;
    print_ver(stdout)?;

    'repl: loop {
        print_prompt(stdout)?;

        let input = get_input(stdout)?;
        let source;

        match input {
            ReplInput::String(input) => source = input,
            ReplInput::Skip => continue,
            ReplInput::Exit => break 'repl,
        };

        // Here is where we want to start parsing and evaluating.
        //
        // let ast = parse(source)?;
        // let res = eval(ast)?;

        let ast = parse(source.clone()).unwrap();
        let res = eval(ast);
        print_eval(stdout, res.to_string())?;
    }

    terminal::disable_raw_mode()?;
    Ok(())
}

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    run(&mut stdout)
}
