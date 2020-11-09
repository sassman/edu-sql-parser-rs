mod ast;
mod code_gen_ts;

use std::io::{self, BufRead, Write};

use lrlex::lrlex_mod;
use lrpar::{lrpar_mod, NonStreamingLexer};

use crate::ast::{CreateTableStmt, TableName};

// Using `lrlex_mod!` brings the lexer for `*.l` into scope. By default the
// module name will be `*_l` (i.e. the file name, minus any extensions,
// with a suffix of `_l`).
lrlex_mod!("sql.l");
// Using `lrpar_mod!` brings the parser for `*.y` into scope. By default the
// module name will be `*_y` (i.e. the file name, minus any extensions,
// with a suffix of `_y`).
lrpar_mod!("sql.y");

pub use sql_l::*;
pub use sql_y::*;

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

fn eval<'input>(lexer: &dyn NonStreamingLexer<'input, u32>, e: CreateTableStmt) {
    match e {
        CreateTableStmt { name, def: _ } => match name {
            TableName::Name(name) => println!("Expr (create table): {}", lexer.span_str(name.span)),
            TableName::SchemaWithName(schema, name) => println!(
                "Expr create table: {} in schema: {}",
                lexer.span_str(name.span),
                lexer.span_str(schema.span)
            ),
        },
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::*;
    use std::collections::HashMap;

    macro_rules! assert_table_name {
        ($sql:expr, $expected:expr) => {
            let lexerdef = sql_l::lexerdef();
            let lexer = lexerdef.lexer($sql);

            let (res, errs) = sql_y::parse(&lexer);
            let mut msgs = vec![];
            for e in errs {
                msgs.push(format!("{}", e.pp(&lexer, &sql_y::token_epp)));
            }
            assert_eq!(msgs.len(), 0, "{}", msgs.join("\n"));

            let e = match res {
                Some(res) => res,
                _ => panic!("Ooops!"),
            };

            match e {
                Ok(CreateTableStmt {
                    name: TableName::Name(name),
                    def: TableDef { columns: _ },
                }) => {
                    let name = lexer.span_str(name.span);
                    assert_eq!(name, $expected, "Table name did not match.");
                }
                Ok(CreateTableStmt {
                    name: TableName::SchemaWithName(_schema, name),
                    def: TableDef { columns: _ },
                }) => {
                    let name = lexer.span_str(name.span);
                    assert_eq!(name, $expected, "Table name did not match.");
                }
                _ => panic!("Result was not a TableName::Name(_)"),
            };
        };
    }

    #[test]
    fn should_parse_a_table_with_1_column() {
        assert_table_name!(
            "CREATE TABLE \"s!@##%1\".\"_abc!@#$%^\" (A INT)",
            "\"_abc!@#$%^\""
        );
        assert_table_name!(
            "CREATE TABLE schema.\"_abc!@#$%^\" (A INT)",
            "\"_abc!@#$%^\""
        );
        assert_table_name!("CREATE TABLE \"_abc!@#$%^\" (A INT)", "\"_abc!@#$%^\"");
        assert_table_name!("create table \"_abc!@#$%^\" (A int)", "\"_abc!@#$%^\"");
        assert_table_name!("CREATE TABLE t1(A INT)", "t1");
        assert_table_name!("CREATE TABLE t1 (A INT)", "t1");
        assert_table_name!("CREATE TABLE t1  \n (A INT)", "t1");
        assert_table_name!("CREATE TABLE t1  \n (  A INT)", "t1");
        assert_table_name!("CREATE TABLE t1  \n (  A INT  )", "t1");
        assert_table_name!("CREATE TABLE t1  \n (\n  A INT \n )", "t1");
        assert_table_name!("CREATE TABLE t1  \n (\n  A  INT \n )", "t1");
    }

    #[test]
    fn should_parse_a_table_with_2_columns() {
        assert_table_name!("CREATE TABLE t1 (A INT,B DOUBLE)", "t1");
        assert_table_name!("CREATE TABLE t1 (A INT, B DOUBLE)", "t1");
        assert_table_name!("CREATE TABLE t1 (A  INT,B  DOUBLE)", "t1");
        assert_table_name!("CREATE TABLE t1 (A  \n INT,  \n B  DOUBLE)", "t1");
        assert_table_name!("CREATE TABLE t1 (  A INT, B DOUBLE)", "t1");
        assert_table_name!("CREATE TABLE t1 (  A INT, B DOUBLE  )", "t1");
        assert_table_name!("CREATE TABLE t1 (\n  A INT, B DOUBLE \n )", "t1");
        assert_table_name!(
            "CREATE TABLE t1\n (\n  A  \n  INT, \n  B \n  DOUBLE \n )",
            "t1"
        );
    }

    #[test]
    fn should_support_all_hana_data_types() {
        let types: HashMap<&str, DataType> = vec![
            ("DATE", DataType::Date),
            ("TIME", DataType::Time),
            ("SECONDDATE", DataType::SecondDate),
            ("TINYINT", DataType::TinyInt),
            ("SMALLINT", DataType::SmallInt),
            ("INT", DataType::Int),
            ("DOUBLE", DataType::Double),
            ("TEXT", DataType::Text),
            ("BINTEXT", DataType::BinText),
            ("VARCHAR(10)", DataType::VarChar(Len(10))),
            ("VARCHAR (10)", DataType::VarChar(Len(10))),
            ("VARCHAR ( 10)", DataType::VarChar(Len(10))),
            ("VARCHAR ( 10 )", DataType::VarChar(Len(10))),
            ("VARCHAR (10 )", DataType::VarChar(Len(10))),
            ("VARCHAR(10 )", DataType::VarChar(Len(10))),
        ]
        .into_iter()
        .collect();
        for (t, _) in types.iter() {
            let t = *t;
            let sql = format!("CREATE TABLE t1 (A {})", t);
            // TODO verify the parsed data types
            assert_table_name!(sql.as_str(), "t1");
        }
    }
}
