# cythan-compiler

A Rust implementation of the Cythan V3 Compiler

## How to program in Cythan Stage 1

### Simplest example

```rust
0  12  2  31  556
```

As you probably know Cythan is a list of numbers, in the case of Cythan Stage 1 numbers are separated by a space.

  

### Variables

In Cythan stage 1, compile time variables are accepted.

  

#### To define a variable:

```rust
var1 = (4  10)
```

  

#### To use a variable:

```rust
0  20 var1 30
```

  

__Note:__  *Variables in Cythan Stage 1 have no scope and are accessible from everywhere*

  

### Functions

In Cythan Stage 1 function are computed at compile time like macros.

  

#### To define a function:

```rust
func1 {
     12  20  10
}
```

  

#### To call a function:

```rust
func1()
```

  

#### With arguments:

```rust
func1 {
    12  20  10  self.0  self.3..10  self.20..40?5
}

func1(10  2  3  45  20  10)
```

  

As you probably seen `self` keyword is used to represent arguments.

  

#### Self cheat-sheet:

`self.x..y` returns all numbers from `x` to `y` if one don't exists, it will not be returned

`self.x..y?z` returns all numbers from `x` to `y` if one don't exists, it will be replaced by `z`

`self.x?z` returns the number `x` if it doesn't exists, it will be replaced by `z`

`self.x` returns the number `x` if it doesn't exists, it will not be returned

`self.x..` returns all numbers from `x` to the end of the arguments if one don't exists, it will not be returned

`self.x..?z` returns all numbers from `x` to the end of the arguments if one don't exists, it will be replaced by `z`

  

## How to compile Cythan Stage 1

From a string:

```rust
use cythan_compiler::*;

tokenizer::Context::new().compute(&tokenizer::generate_tokens(string))
```

  

From a file:

```rust
use cythan_compiler::*;

tokenizer::Context::new().compute(&tokenizer::generate_tokens(std::fs::read_to_string("file.ct").unwrap()))
```

  

Get only executable tokens from a file:

```rust
use cythan_compiler::*;

let tokens = tokenizer::generate_tokens(std::fs::read_to_string("file.ct").unwrap();
```

  

## Run in a cythan machine

  

To run your generated code into a Cythan machine use [Cythan Rust Library](https://github.com/Cypooos/Cythan-v3)