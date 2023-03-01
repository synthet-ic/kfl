//! <https://docs.python.org/3/reference/grammar.html>

use repr::Pat;

fn main() {
    let NAME = Pat::new("");
    let STRING = Pat::new(r"\w");
    let NUMBER = Pat::new(r"[0-9]+");
    let AWAIT = "await";
    let strings = STRING * (1..);
    let comparison
        = bitwise_or & compare_op_bitwise_or_pair * 1..
        | bitwise_or;
    let inversion
        = "not" & self 
        | comparison;
    let conjunction
        = inversion & ("and" & inversion ) * 1..
        | inversion;
    let disjunction
        = conjunction & ("or" & conjunction ) * 1..
        | conjunction;
    let expression
        = disjunction & "if" & disjunction & "else" & self 
        | disjunction
        | lambdef;
    let assignment_expression = NAME & ":=" ~ expression;
    let named_expression
        = assignment_expression
        | expression & !":=";
    let slice
        = expression? & ':' & expression? & (':' & expression?)?;
        | named_expression;
    let slices
        = slice & !','
        | ','.(slice | starred_expression)+ ','?;
    let primary
        = self & '.' & NAME
        | self & genexp
        | self & '(' & arguments? & ')'
        | self & '[' & slices & ']'
        | atom;
    let await_primary
        = AWAIT & primary
        | primary;
    let power
        = await_primary & "**" & factor
        | await_primary;
    let factor
        = '+' & self
        | '-' & self
        | '~' & self
        | power;
    let term
        = self & '*' & factor
        | self & '/' & factor
        | self & "//" & factor
        | self & '%' & factor
        | self & '@' & factor
        | factor;
    let sum
        = self & '+' & term
        | self & '-' & term
        | term;
    let shift_expr
        = self & "<<" & sum
        | self & ">>" & sum
        | sum;
    let bitwise_and
        = self & '&' & shift_expr
        | shift_expr;
    let bitwise_xor
        = self & '^' & bitwise_and
        | bitwise_and;
    let bitwise_or
        = self & '|' & bitwise_xor
        | bitwise_xor;
    let star_named_expression
        = '*' & bitwise_or
        | named_expression;
    let star_named_expressions
        = star_named_expression & (',' & star_named_expression) * .. & ','?;
    let list = '[' & star_named_expressions? & ']';
    let for_if_clauses = for_if_clause * 1..;
    let listcomp = '[' & named_expression & for_if_clauses & ']';
    let tuple
        = '('
        & (
            star_named_expression
            & ','
            & star_named_expressions?
        )?
        & ')';
    let set = '{' & star_named_expressions & '}';
    let atom =
          NAME
        | "True"
        | "False"
        | "None"
        | &strings
        | &NUMBER
        | tuple | group | genexp
        | list | listcomp
        | dict | set | dictcomp | setcomp
        | "...";
}
