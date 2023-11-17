# lambda-parser

## Run code
install rust
```bash
rustc main.rs
```
```bash
./main
```

## Informations

Here is the specified grammar for the concrete syntax of terms in the Simply Typed Lambda Calculus of this work. In summary, the differences regarding the syntax written in class are as follows (note that the changes basically make the syntax more easily readable and remove ambiguities):

The "if" statements are terminated by "endif," as in "if true then 1 else 0 endif."
Instead of "eh_zero," we write "ehzero."
Instead of "T -> U," we write "(T -> U)."
Instead of "λx:T.t," we write "lambda x : T . t end."
Instead of "t u," we write "(t u)."
In the grammar below, we use the following conventions:

To make the presentation less cluttered, instead of "<symbol>," we write "SYMBOL."
For clarity, each white space that should occur in the object language (i.e., in the language of the simply typed lambda calculus) is denoted by the symbol " ". This also means that white spaces present in the rules below, as in DIGITO_NAO_ZERO SEQ_DIGITOS, exist only for the readability of the grammar, not representing spaces in the object language.


```bash
----------------------------------------------------------------------------
TERMO ::= true
| false
| if TERMO then TERMO else TERMO endif
| N´UMERO
5
| suc
| pred
| ehzero
| VARIAVEL
| ( TERMO TERMO )
| lambda VARIAVEL : TIPO . TERMO end
----------------------------------------------------------------------------
NUMERO ::= DIGITO | DIGITO_N~AO_ZERO SEQ_DIGITOS
DIGITO ::= 0 | DIGITO_NAO_ZERO
DIGITO_NAO_ZERO ::= 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
SEQ_DIGITOS ::= DIGITO | DIGITO SEQ_DIGITOS
----------------------------------------------------------------------------
VARIAVEL (*) ::= LETRA | LETRA SEQ_ALFA_NUM
LETRA ::= a | b | c | d | e | f | g | h | i
| j | k | l | m | n | o | p | q | r
| s | t | u | v | w | x | y | z
| A | B | C | D | E | F | G | H | I
| J | K | L | M | N | O | P | Q | R
| S | T | U | V | W | X | Y | Z
SEQ_ALFA_NUM ::= ALFA_NUM | ALFA_NUM SEQ_ALFA_NUM
ALFA_NUM ::= LETRA | D´IGITO
----------------------------------------------------------------------------
TIPO ::= Bool | Nat | ( TIPO -> TIPO )
----------------------------------------------------------------------------
```
