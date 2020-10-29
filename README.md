## SQL Types

WIP: SQL Types transpiles `CREATE TABLE` SQL code into type definitions in other languages such as TypeScript or Rust. 

## Supported Syntax

the following terms are supported, the format is EBNF

```ebnf
<digit> ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9

<letter> ::= a | b | c | d | e | f | g | h | i | j | k | l | m | n | o | p | q | r | s | t | u | v | w | x | y | z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z

<any_character> ::= !!any character.

<dollar_sign> ::= $

<double_quotes> ::= "

<underscore> ::= _

<identifier> ::= <simple_identifier> | <special_identifier>

<simple_identifier> ::= {<letter> | <underscore>} [{<letter> | <digit> | <underscore> | <hash_symbol> | <dollar_sign>}...]

<special_identifier> ::= <double_quotes><any_character>...<double_quotes>

CREATE [ COLUMN ] TABLE <table_name> (<table_contents_source>, ...) <using_extended_storage_clause>

<table_name> ::= [<schema_name>.]<identifier>

```

## To be done

Following syntax tokens are yet to be implemented

```ebnf


<table_contents_source> ::= (<table_element>, ...) | [(<column_name>, ...)]
```

[hana/sql]: https://help.sap.com/doc/9b40bf74f8644b898fb07dabdd2a36ad/2.0.04/en-US/SAP_HANA_SQL_Reference_Guide_en.pdf

## Usage

there is a little interactive shell to enter SQL and get parsed AST expressions back.

```sh
❯ cargo run
>>> x
Parsing error at line 1 column 1. Repair sequences found:
   1: Insert create_table, Insert space
>>> CREATE TABLE abc
Expr (create table): abc
>>> CREATE TABLE "abc@#$%"
Expr (create table): "abc@#$%"
```

## License

- **[GNU GPL v3 license](https://www.gnu.org/licenses/gpl-3.0)**
- Copyright 2020 © [Sven Assmann][2].

[2]: https://www.d34dl0ck.me
[4]: https://github.com/sassman/sql-types-rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22
[5]: https://github.com/sassman/sql-types-rs/issues/new/choose
