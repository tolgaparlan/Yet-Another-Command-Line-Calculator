![main branch](https://github.com/tolgaparlan/Yet-Another-Command-Line-Calculator/actions/workflows/ci.yml/badge.svg)

# README 
A basic integer arithmetic calculator I have built with Rust for self-learning. 

Run with `cargo run`.

Starts a REPL to evaluate arithmetic expressions and produce the calculated result at the next line.

Supports arbitrarily large numbers, as well as variables.

So far only has addition, subtraction, multiplication and division. Keeps proper operator precedence.

Commands:
- `exit`: Stops the REPL
- `vars`: Prints all variables currently stored in memory
- `dec`: Changes the display mode to decimal (default)
- `hex`: Changes the display mode to hexadecimal representation
- `bin`: Changes the display mode to binary representation

Command strings cannot be used as variable names.

Special Variables:
- `$`: Special variable to hold the last result 

```
1232342353453*34545364587894567456
\> 42571715897137916732960503025568
$ / 123
\> 346111511358844851487483764435
23-
Invalid Expresssion
10+(5/2)
\> 12
num = $
\> 12
num * 12
\> 144
vars
\> num = 0xC
\> $ = 0x90
hex
vars
\> num = 12
\> $ = 144
exit
```
