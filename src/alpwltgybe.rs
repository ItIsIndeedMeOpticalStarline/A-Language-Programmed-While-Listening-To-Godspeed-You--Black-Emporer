/* --------------------------- *//*

    ALBWLTGY-BE Specifications:

            --::TYPES::--
    - bite represents an 8 bit integer
    - nom represents a 16 bit integer
    - chomp represents a 32 bit integer

    - drift represents a 32 bit float

    - charms represents an array of characters

    - sub: represents a subroutine
    - comp: represents an eval ? 1 : 0
    - loop: represents a while value != 0
    - end: ends the loop or subroutine
    - name: names an index on the stack
    - skip: skips x number of instructions if the top value of the stack is equal to zero
    - flip: reverses the stack
    - give up: ends the program

            --::STANDARD::--
    - sub -1: used by the interpreter. if you use this your program will not compile
    
    - sub 0: pops then prints the top value on the stack as an ascii character
    - sub 1: pops then prints the top value on the stack as a number
    - sub 2: pops then prints the top value on the stack if it is a charms

    - sub 5: pops then sums the top two values on the stack (val2 + val1) and places a single value back on the stack
    - sub 6: pops then subtracts the top two values on the stack (val2 - val1) and places a single value back on the stack
    - sub 7: pops then multiplies the top two values on the stack (val2 * val1) and places a single value back on the stack
    - sub 8: pops then divides the top two values on the stack (val2 / val1) and places a single value back on the stack
    - sub 9: pops then mods the top two values on the stack (val2 % val1) and places a single value back on the stack

*//* -------------------------- */

#[derive(PartialEq)]
pub enum GybeTkn
{
    NUM,
    IDEN,

    QUOTE,
    SEMI,
    PERIOD,

    ILLEGAL
}