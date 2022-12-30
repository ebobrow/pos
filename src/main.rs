use std::fs;

// TODO: don't want backtrace?
use eyre::{Result, WrapErr};
use lex::TokenType;
use rand::Rng;
use rustyline::{Config, EditMode};

mod lex;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match &args[..] {
        [] => repl()?,
        [filename] => run_file(filename.to_string())?,
        _ => println!("Usage: pos [FILE]"),
    }

    Ok(())
}

fn run_file(filename: String) -> Result<()> {
    let contents = fs::read_to_string(&filename)
        .wrap_err_with(|| format!("Failed to read file `{filename}`"))?;
    go(contents)?;

    Ok(())
}

fn repl() -> Result<()> {
    // TODO: ambitious but auto completion
    let mut rl = rustyline::Editor::<()>::with_config(
        Config::builder()
            .edit_mode(EditMode::Vi)
            .auto_add_history(true)
            .build(),
    )?;
    while let Ok(line) = rl.readline("POS> ") {
        // TODO: don't kill repl on err
        go(line)?;
    }

    let salutations = ["Parts of Speech", "Parser of Syntax"];
    println!(
        "*** {} ***",
        salutations[rand::thread_rng().gen_range(0..salutations.len())]
    );

    Ok(())
}

fn go(src: String) -> Result<()> {
    let mut lexer = lex::Lexer::new(src);
    loop {
        let tok = lexer.scan_token()?;
        if tok.ty() == &TokenType::EOF {
            return Ok(());
        }
        println!("{:#?}", tok);
    }
}
