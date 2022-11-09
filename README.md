# README 
A basic integer arithmetic calculator I have built with Rust for self-learning. 

Run with `cargo run`.

Starts a REPL to evaluate arithmetic expressions and produce the calculated result at the next line.

Supports arbitrarily large numbers, as well as variables.

So far only has addition, subtraction, multiplication and division. Keeps proper operator precedence.

Commands:
- `exit`: Stops the REPL
- `vars`: Prints all variables currently stored in memory

```
./simple-calculator
1232342353453*34545364587894567456
\> 42571715897137916732960503025568
123+0
\> 123
123+1324
\> 1447
23-1123
\> -1100
23-
Expected Number
10+(5/2)
\> 12
num = (10+(5/2))
\> 12
num * 12
\> 144
num2 = 99/2
\> 49
vars
\> num = 12
\> num2 = 49
exit
```
