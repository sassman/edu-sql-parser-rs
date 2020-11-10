%start CreateTableStmt
%%
CreateTableStmt -> Result<CreateTableStmt>:
    'create_table' TableName '(' TableContentSource ')' {
        Ok(CreateTableStmt {
            name: $2?,
            def: TableDef::new($4?)
        })
    }
    ;

TableName -> Result<TableName>:
      SchemaName '.' Identifier { Ok(TableName::SchemaWithName($1?, $3?)) }
    | Identifier { Ok(TableName::Name($1?)) }
    ;

TableContentSource -> Result<Vec<ColumnDef>>:
      TableElement { Ok(vec![$1?]) }
    | TableElement ',' TableElement { Ok(vec![$1?, $3?]) }
    ;

TableElement -> Result<ColumnDef>:
      ColumnDefinition { $1 }
    ;

ColumnDefinition -> Result<ColumnDef>:
      ColumnName DataType { Ok(ColumnDef{ name: $1?, data_type: $2? }) }
    ;

ColumnName -> Result<Ident>: Identifier { $1 };

DataType -> Result<DataType>:
      'DATE' { Ok(DataType::Date) }
  |   'TIME' { Ok(DataType::Time) }
  |   'SECONDDATE' { Ok(DataType::SecondDate) }
  |   'TINYINT' { Ok(DataType::TinyInt) }
  |   'SMALLINT' { Ok(DataType::SmallInt) }
  |   'INT' { Ok(DataType::Int) }
  |   'DOUBLE' { Ok(DataType::Double) }
  |   'TEXT' { Ok(DataType::Text) }
  |   'BINTEXT' { Ok(DataType::BinText) }
  |   'VARCHAR' '(' 'digit' ')' { Ok(DataType::VarChar(Len::try_from($lexer.span_str($3.map_err(|_| "<evaluation aborted>")?.span()))?)) }
  ;

SchemaName -> Result<Ident>: Identifier { $1 };

Identifier -> Result<Ident>:
      SimpleIdentifier { $1 }
    | SpecialIdentifier { $1 }
    ;

SimpleIdentifier -> Result<Ident>:
      SimpleIdentifier 'letter' { Ok(Ident::new($span)) }
    | SimpleIdentifier '_'      { Ok(Ident::new($span)) }
    | SimpleIdentifier 'digit'  { Ok(Ident::new($span)) }
    | SimpleIdentifier '#'      { Ok(Ident::new($span)) }
    | SimpleIdentifier '$'      { Ok(Ident::new($span)) }
    | 'letter'                  { Ok(Ident::new($span)) }
    | '_'                       { Ok(Ident::new($span)) }
    ;

SpecialIdentifier -> Result<Ident>:
      '"' AnyCharacter '"'      { Ok(Ident::new($span)) }
    ;

/// Note: this needs to be extended for all specified
///       known keywords
AnyCharacter -> Result<Span>:
      'any_character' AnyCharacter  { Ok($span) }
    | 'letter' AnyCharacter         { Ok($span) }
    | 'digit' AnyCharacter          { Ok($span) }
    | '_' AnyCharacter              { Ok($span) }
    | '#' AnyCharacter              { Ok($span) }
    | '$' AnyCharacter              { Ok($span) }
    | 'any_character'               { Ok($span) }
    | 'letter'                      { Ok($span) }
    | 'digit'                       { Ok($span) }
    | '_'                           { Ok($span) }
    | '#'                           { Ok($span) }
    | '$'                           { Ok($span) }
    ;
%%
// Any imports here are in scope for all the grammar actions above.
use lrpar::Span;
use std::convert::TryFrom;
use crate::ast::*;
