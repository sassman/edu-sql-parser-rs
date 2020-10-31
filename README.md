## SQL Types

WIP: SQL Types transpiles `CREATE TABLE` SQL code into type definitions in other languages such as TypeScript or Rust. 

## Syntax

Basic syntax rules:
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

// note: original definition is <unicode_name> instead of <identifier>
<schema_name> ::= <identifier>
```

### Create Table Syntax

Currently Work in progress

```
// simplified version
CREATE [ <table_type> ] <table_name>
 [<table_contents_source>
 [<system_versioning_spec>]
 [<application_time_period_configuration>]
 [<bi_temporal_table_spec>]
 [<with_association_clause>]
 [<with_annotation_clause>]
 [<with_mask_clause>]
 [<logging_option>]
 [<auto_merge_option>]
 [<unload_priority_clause>]
 [<schema_flexibility_option>]
 [<partition_clause>]
 [<persistent_memory_spec_clause>]
 [<group_option_list>]
 [<location_clause>]
 [<replica_clause>]
 [<global_temporary_option>]
 [<series_clause>]
 [<unused_retention_period_option>]
 [<record_commit_timestamp_clause>]
 [COMMENT <comment_string>]
 [<numa_node_preference_clause>]
 [<load_unit>]

// simplified version
<table_type> ::= [ COLUMN ] TABLE
<table_name> ::= [<schema_name>.]<identifier>

// simplified version
<table_contents_source> ::=
 (<table_element>, ...) [<with_association_clause>]

<table_element> ::=
   <column_definition> [ <column_constraint> ]
 | <table_constraint>

<column_definition> ::= <column_name> { <data_type> | <lob_data_type> }
 [ <ddic_data_type> ]
 [ <default_value_clause> ]
 [ <clientside_encryption> ]
 [ <col_gen_as_expression> | <col_gen_as_ident> ]
 [ <col_calculated_field> ]
 [ <schema_flexibility> ]
 [ <fuzzy_search_index> ]
 [ <fuzzy_search_mode> ]
 [ <persistent_memory_spec_clause> ]
 [ COMMENT <comment_string> ]
 [ <load_unit> ]
 [ <numa_node_preference_clause> ]
  
<column_name> ::= <identifier>

<data_type> ::=
 DATE
 | TIME
 | SECONDDATE
 | TIMESTAMP
 | TINYINT
 | SMALLINT
 | INT
 | BIGINT
 | SMALLDECIMAL
 | REAL
 | DOUBLE
 | TEXT
 | BINTEXT
 | VARCHAR [ (<unsigned_integer>) ]
 | NVARCHAR [ (<unsigned_integer>) ]
 | ALPHANUM [ (<unsigned_integer>) ]
 | VARBINARY [ (<unsigned_integer>) ]
 | SHORTTEXT [ (<unsigned_integer>) ]
 | DECIMAL [ (<unsigned_integer> [, <unsigned_integer> ]) ]
 | FLOAT [ (<unsigned_integer>) ]
 | BOOLEAN

<default_value_clause> ::= DEFAULT <default_value_exp>

<default_value_exp> ::=
 NULL
 | <string_literal>
 | <signed_numeric_literal> <unsigned_numeric_literal>
 | <datetime_value_function>

<datetime_value_function> ::=
 CURRENT_DATE
 | CURRENT_TIME
 | CURRENT_TIMESTAMP
 | CURRENT_UTCDATE
 | CURRENT_UTCTIME
 | CURRENT_UTCTIMESTAMP
 
<comment_string> ::= <string_literal>

<column_constraint> ::=
 NULL
 | NOT NULL
 | { HIDDEN | NOT HIDDEN }
 | [ <constraint_name_definition> ] <unique_specification>
 | [ <constraint_name_definition> ] <references_specification>
 
<unique_specification> ::=
 UNIQUE [ <unique_tree_type_index> ]
 | PRIMARY KEY [ <unique_tree_type_index> | <unique_inverted_type_index> ]

<unique_tree_type_index> ::= { BTREE | CPBTREE }
<unique_inverted_type_index> ::= INVERTED [ <composite_type> ]
<composite_type> ::= { HASH | VALUE | INDIVIDUAL }

```

[hana/sql]: https://help.sap.com/doc/9b40bf74f8644b898fb07dabdd2a36ad/2.0.04/en-US/SAP_HANA_SQL_Reference_Guide_en.pdf

## Usage

there is a little interactive shell to enter SQL and get parsed AST expressions back.

```sh
❯ cargo run
>>> x
Parsing error at line 1 column 1. No repair sequences found.
>>> CREATE TABLE abc(A INT)
Expr (create table): abc
```

## License

- **[GNU GPL v3 license](https://www.gnu.org/licenses/gpl-3.0)**
- Copyright 2020 © [Sven Assmann][2].

[2]: https://www.d34dl0ck.me
[4]: https://github.com/sassman/sql-types-rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22
[5]: https://github.com/sassman/sql-types-rs/issues/new/choose
