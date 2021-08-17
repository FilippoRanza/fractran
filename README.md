# fractran
A FRACTRAN language interpreter

## Introduction
FRACTRAN is a Turing-complete esoteric programming language invented 
by John Conway in 1987. For more information check [Wikipedia page](https://en.wikipedia.org/wiki/FRACTRAN) and the [original paper](https://link.springer.com/chapter/10.1007/978-1-4612-4808-8_2).

## Installation

```fractran``` should be compiled and installed using [cargo](https://doc.rust-lang.org/cargo/).
In this Section I assume that the reader has basic knowledge about 
git and the usage of command line environment.

To compile ```fractran``` clone this repository and then build the binary using:.

```bash
cargo build --release
```
The executable should be in the ```target/release``` directory.

Otherwise you can directly compile and install ```fractran``` by running:
```bash
cargo install --git https://github.com/FilippoRanza/fractran
```

In this case ```cargo``` will automatically move the executable into 
the default installation directory. 

If you are on Unix remember to strip the executable. Go 
to the directory containing the ```fractran``` executable and run:
```bash
strip fractran
```

## Usage
```fractran``` takes its input source code from file given as command line argument.
Running a FRACTRAN program with ```fractran``` is very simple, just call:
```bash
fractran your-src.fractran
```

You can always read the help:
```bash
fractran --help
```

## Language

A FRACTRAN program is an ordered list of fractions and 
an initial value. ```fractran``` expects a file written as:
```
init NUMBER
list FRACTION_LIST
```
Where **FRACTION_LIST** is:
```
FRACTION_LIST := FRACTION FRACTION_LIST | FRACTION
```
And **FRACTION** is:
```
FRACTION := NUMBER NUMBER | NUMBER / NUMBER
```
The first **NUMBER** is the numerator, the second is the denominator.

**NUMBER** is an positive integer number (internally a 128 bit unsigned 
number). 

It is possible to use C style single line and multi line comments.


### Example 
The following code implements the multiplication.
``` 
/*
    FRACTRAN multiplication.
*/
init 72 // 72 = 2³ + 3²
list 
    455 33
    11 13
    1 11
    3 7
    11 2
    1 3
```

More examples can be found in the ```examples``` directory.
