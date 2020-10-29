%start Expr
%%
Expr -> Result<Expr>:
      CreateTable { $1 }
    ;

CreateTable -> Result<Expr>:
      'create_table' 'space' Identifier { Ok(Expr::CreateTable($3?)) }
    ;

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

use std::error::Error;
use lrpar::Span;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
pub enum Expr {
    CreateTable(Ident)
}

#[derive(Debug, PartialEq)]
pub struct Ident {
    pub span: Span
}

impl Ident {
    fn new(span: Span) -> Self {
        Self { span }
    }
}