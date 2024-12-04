# BNF
Below is the BNF this library implements. Note that "identifier" represents the lexical token`Token::Identifier`, and will treated like a terminal. Similarly with the "literal" "type" labels.
```text
<FUNCTION DEFINITION> -> type identifier (<FUNCTION PARAMETERS>){<COMPOUND STATEMENTS>}

<FUNCTION PARAMETERS> -> <FUNCTION PARAMETER><FUNCTION PARAMETERS'>
                       | ε
<FUNCTION PARAMETERS'> -> ,<FUNCTION PARAMETER><FUNCTION PARAMETERS'>
                        | ε
<FUNCTION PARAMETER> -> type identifier

<COMPOUND STATEMENTS> -> <STATEMENT>;<COMPOUND STATEMENTS>
                       | ε

<STATEMENT> -> <ASSIGNMENT STATEMENT>
             | <RETURN STATEMENT>

<ASSIGNMENT STATEMENT> -> identifier = <EXPRESSION>

<RETURN STATEMENT> -> return <EXPRESSION>

<EXPRESSION> -> <ARITHMETIC EXPRESSION>
              | <TYPECAST EXPRESSION>
<TYPECAST EXPRESSION> -> (type)identifier
<ARITHMETIC EXPRESSION> -> <TERM><TERM'>

<TERM> -> <FACTOR><FACTOR'>
<TERM'> -> +<TERM>
         | -<TERM>
         | ε

<FACTOR> -> identifier
          | literal
<FACTOR'> -> *<FACTOR>
           | /<FACTOR>
           | ε
```