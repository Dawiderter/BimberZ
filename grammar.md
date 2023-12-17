program                 ->      declaration* EOF

## Declarations
declaration             ->      statement

## Statements
statement               ->      expression_statement | print_statement | if_statement | Block

expression_statement    ->      expression "\n"
print_statement         ->      "print" expression "\n"
if_statement            ->      "if" expression block ("else" block)?
block                   ->      "{" declaration "}"

## Expressions
expression              ->      assignment

assignment              ->      variable "=" expression | logic_or
logic_or                ->      logic_and ( "or" logic_and )*
logic_and               ->      equality ( "and" equality )*
equality                ->      comparison ( ( "!=" | "==" ) comparison)*
comparison              ->      term ( ( ">" | ">=" | "<" | "<=") term )*
term                    ->      factor ( ( "-" | "+" ) factor )*
factor                  ->      unary ( ( "/" | "*" ) unary )*
unary                   ->      ( "!" | "-" ) unary | primary
primary                 ->      "true" | "false" | REAL | INTEGER | "(" expression ")" | variable
variable                ->      IDENTIFIER | member
member                  ->      IDENTIFIER ("." IDENTIFIER)*


arguments               ->      expression*
