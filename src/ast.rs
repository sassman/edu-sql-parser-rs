use crate::code_gen_ts::{Visitable, Visitor};
use lrpar::Span;
use std::convert::TryFrom;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
pub struct CreateTableStmt {
    pub name: TableName,
    pub def: TableDef,
}

impl Visitable for CreateTableStmt {
    fn accept(&self, visitor: &mut impl Visitor) {
        visitor.visit_create_table_stmt(self);
    }
}

#[derive(Debug, PartialEq)]
pub enum TableName {
    Name(Ident),
    SchemaWithName(Ident, Ident),
}

#[derive(Debug, PartialEq)]
pub struct TableDef {
    pub columns: Vec<ColumnDef>,
}

impl TableDef {
    pub fn new(columns: Vec<ColumnDef>) -> Self {
        Self { columns }
    }
}

impl Visitable for TableDef {
    fn accept(&self, visitor: &mut impl Visitor) {
        visitor.visit_table(self);
    }
}

#[derive(Debug, PartialEq)]
pub struct ColumnDef {
    pub name: Ident,
    pub data_type: DataType,
}

impl Visitable for ColumnDef {
    fn accept(&self, visitor: &mut impl Visitor) {
        visitor.visit_column(self);
    }
}

#[derive(Debug, PartialEq)]
pub struct Ident {
    pub span: Span,
}

impl Ident {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl Visitable for Ident {
    fn accept(&self, visitor: &mut impl Visitor) {
        visitor.visit_ident(self);
    }
}

impl From<&str> for Ident {
    fn from(s: &str) -> Self {
        Self::new(Span::new(0, s.len()))
    }
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum DataType {
    Date,
    Time,
    SecondDate,
    TinyInt,
    SmallInt,
    Int,
    Double,
    Text,
    BinText,
    VarChar(Len),
}

impl Visitable for DataType {
    fn accept(&self, visitor: &mut impl Visitor) {
        visitor.visit_data_type(self);
    }
}

#[derive(Debug, PartialEq)]
pub struct Len(pub usize);

impl TryFrom<&str> for Len {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.parse::<usize>() {
            Ok(value) => Ok(Len(value)),
            Err(_) => Err(Box::from(format!("{} cannot be parsed as usize", value))),
        }
    }
}
