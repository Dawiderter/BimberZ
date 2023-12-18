program                 ->      declaration* EOF

## Declarations
declaration             ->      statement

## Statements
statement               ->      expression_statement | print_statement | if_statement | for_statement | while_statement | block

expression_statement    ->      expression "\n"
print_statement         ->      "print" expression "\n"
if_statement            ->      "if" expression block ("else" block)?
for_statement           ->      "for" IDENTIFIER in expression block
while_statement         ->      "while" expression block
block                   ->      "{" declaration "}"

## Expressions
expression              ->      assignment

assignment              ->      variable "=" expression | ternary
ternary                 ->      logic_or ( "if" logic_or "else" logic_or)?
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
