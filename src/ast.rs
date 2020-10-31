use lrpar::Span;
use std::convert::TryFrom;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
pub enum Expr {
    CreateTable { name: TableName, def: TableDef },
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

#[derive(Debug, PartialEq)]
pub struct ColumnDef {
    pub name: Ident,
    pub data_type: DataType,
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
