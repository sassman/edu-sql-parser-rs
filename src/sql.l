%%
[Cc][Rr][Ee][Aa][Tt][Ee]\s[Tt][Aa][Bb][Ll][Ee]\s+           "create_table"
[Dd][Aa][Tt][Ee]                            "DATE"
[Tt][Ii][Mm][Ee]                            "TIME"
[Ss][Ee][Cc][Oo][Nn][Dd][Dd][Aa][Tt][Ee]    "SECONDDATE"
[Tt][Ii][Nn][Yy][Ii][Nn][Tt]                "TINYINT"
[Ss][Mm][Aa][Ll][Ll][Ii][Nn][Tt]            "SMALLINT"
[Ii][Nn][Tt]                                "INT"
[Dd][Oo][Uu][Bb][Ll][Ee]                    "DOUBLE"
[Tt][Ee][Xx][Tt]                            "TEXT"
[Bb][Ii][Nn][Tt][Ee][Xx][Tt]                "BINTEXT"
[Vv][Aa][Rr][Cc][Hh][Aa][Rr]                "VARCHAR"

[0-9]+          "digit"
[a-zA-Z]+       "letter"
[_]             '_'
[$]             '$'
[#]             '#'
["]             '"'
[.]             '.'
[(]             '('
[)]             ')'
[,]             ','

[\s\n\t]        ;
.               "any_character"
