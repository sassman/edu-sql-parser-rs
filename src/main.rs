#[cfg(test)]
#[macro_use]
extern crate assert_matches;

use std::io::{self, BufRead, Write};

use lrlex::lrlex_mod;
use lrpar::{lrpar_mod, NonStreamingLexer};

// Using `lrlex_mod!` brings the lexer for `*.l` into scope. By default the
// module name will be `*_l` (i.e. the file name, minus any extensions,
// with a suffix of `_l`).
lrlex_mod!("sql.l");
// Using `lrpar_mod!` brings the parser for `*.y` into scope. By default the
// module name will be `*_y` (i.e. the file name, minus any extensions,
// with a suffix of `_y`).
lrpar_mod!("sql.y");

use sql_y::Expr;

fn main() {
    // Get the `LexerDef` for the `sql` language.
    let lexerdef = sql_l::lexerdef();
    let stdin = io::stdin();
    loop {
        print!(">>> ");
        io::stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                // Now we create a lexer with the `lexer` method with which
                // we can lex an input.
                let lexer = lexerdef.lexer(l);
                // Pass the lexer to the parser and lex and parse the input.
                let (res, errs) = sql_y::parse(&lexer);
                if errs.is_empty() {
                    if let Some(Ok(r)) = res {
                        eval(&lexer, r)
                    }
                } else {
                    for e in errs {
                        println!("{}", e.pp(&lexer, &sql_y::token_epp));
                    }
                }
            }
            _ => break,
        }
    }
}

fn eval<'input>(lexer: &dyn NonStreamingLexer<'input, u32>, e: Expr) {
    match e {
        Expr::CreateTable(t) => {
            let str = lexer.span_str(t.span);
            println!("Expr (create table): {}", str);
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    fn parse_sql(sql: &str) -> Result<Expr> {
        let lexerdef = sql_l::lexerdef();
        let lexer = lexerdef.lexer(sql);

        let (res, errs) = sql_y::parse(&lexer);
        let mut msgs = vec![];
        for e in errs {
            msgs.push(format!("{}", e.pp(&lexer, &sql_y::token_epp)));
        }
        assert_eq!(msgs.len(), 0, "{}", msgs.join("\n"));

        match res {
            Some(res) => res,
            _ => panic!("Ooops!"),
        }
    }

    #[test]
    fn should_parse_create_table() -> Result<()> {
        assert_matches!(parse_sql("CREATE TABLE x")?, Expr::CreateTable(_));
        Ok(())
    }

    #[test]
    fn should_parse_create_table_with_special_identifier() -> Result<()> {
        assert_matches!(
            parse_sql("CREATE TABLE \"_abc!@#$%^\"")?,
            Expr::CreateTable(_)
        );
        Ok(())
    }
}
