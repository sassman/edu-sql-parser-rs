use crate::ast::{ColumnDef, CreateTableStmt, DataType, Ident, Len, TableDef, TableName};
use lrpar::NonStreamingLexer;
use std::borrow::Borrow;
use std::io::Write;
use std::marker::PhantomData;

pub trait Visitable {
    fn accept(&self, visitor: &mut impl Visitor);
}

pub trait Visitor {
    fn visit_create_table_stmt(&mut self, expr: &CreateTableStmt);
    fn visit_table(&mut self, tab: &TableDef);
    fn visit_column(&mut self, col: &ColumnDef);
    fn visit_ident(&mut self, id: &Ident);
    fn visit_data_type(&mut self, datatype: &DataType);
    fn visit_len(&mut self, len: &Len);
}

pub struct TypeScriptGen<'i, T: NonStreamingLexer<'i, u32>> {
    out: Box<dyn Write>,
    lexer: Box<T>,
    phantom: PhantomData<&'i T>,
}

impl<'i, T> TypeScriptGen<'i, T>
where
    T: NonStreamingLexer<'i, u32>,
{
    fn new(out: Box<dyn Write>, lexer: Box<T>) -> Self {
        TypeScriptGen {
            out,
            lexer,
            phantom: PhantomData,
        }
    }

    fn print<X: ToString>(&mut self, str: X) {
        &self
            .out
            .write_all(str.to_string().as_bytes())
            .expect("Cannot write to output");
    }

    fn id_to_str(&self, id: &Ident) -> &str {
        self.lexer.span_str(id.span)
    }
}

impl<'i, T> Visitor for TypeScriptGen<'i, T>
where
    T: NonStreamingLexer<'i, u32>,
{
    fn visit_create_table_stmt(&mut self, expr: &CreateTableStmt) {
        let name = match &expr.name {
            TableName::Name(n) => self.lexer.span_str(n.span),
            TableName::SchemaWithName(_, n) => self.lexer.span_str(n.span),
        };

        self.print(format!(
            "// generated from table `{}`\ntype {} = {}",
            name, name, "{"
        ));

        expr.def.accept(self);

        self.print("\n};\n");
    }

    fn visit_table(&mut self, tab: &TableDef) {
        let mut sep = "";
        for c in tab.columns.iter() {
            if sep.len() > 0 {
                self.print(sep);
            }
            c.accept(self);
            sep = ",";
        }
    }

    fn visit_column(&mut self, col: &ColumnDef) {
        self.print("\n  // relates to field `");
        col.name.accept(self);
        self.print(format!("` of type {:?}", col.data_type));
        self.print("\n  ");
        col.name.accept(self);
        self.print(": ");
        col.data_type.accept(self);
    }

    fn visit_ident(&mut self, id: &Ident) {
        self.print(self.id_to_str(id).to_string());
    }

    fn visit_data_type(&mut self, datatype: &DataType) {
        let t = match datatype {
            DataType::TinyInt | DataType::SmallInt | DataType::Int | DataType::Double => "number",
            DataType::Date
            | DataType::Time
            | DataType::SecondDate
            | DataType::Text
            | DataType::BinText
            | DataType::VarChar(_) => "string",
        };
        self.print(t);
    }

    fn visit_len(&mut self, len: &Len) {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::Result;
    use crate::ast::TableName;
    use lrlex::lrlex_mod;
    use lrpar::lrpar_mod;

    lrlex_mod!("sql.l");
    lrpar_mod!("sql.y");

    #[test]
    fn should_generate_a_typescript_structure() -> Result<()> {
        let stmt = "CREATE TABLE abc (a INT, b VARCHAR(10))";
        let lexer_def = sql_l::lexerdef();
        let lexer = lexer_def.lexer(stmt);
        let (res, errs) = sql_y::parse(&lexer);
        let ast = res.unwrap().unwrap();

        // let ast = CreateTableStmt {
        //     name: TableName::Name(Ident::from("abc")),
        //     def: TableDef {
        //         columns: vec![ColumnDef {
        //             name: Ident::from("a"),
        //             data_type: DataType::Int,
        //         }],
        //     },
        // };

        // let ast = res.unwrap().unwrap();

        let stdout = std::io::stdout();
        let b = Box::new(stdout);
        let lexer_box = Box::new(lexer);
        let mut code_generator = TypeScriptGen::new(b, lexer_box);

        ast.accept(&mut code_generator);

        Ok(())
    }
}
