# Notes

## To remember
- `_` is used before variables, functions and methods to silence the compiler, to ignore a component in a destructure or as a else-like in a match;
- Casting (`as`): `let integer = decimal as u8;`;
- Tuple assignment: `let (integer, boolean) = (1, true);`;
- Initialize array with the same values: `let y: [i32; 500] = [0; 500];`;
- Define a mutable variable: `let mut mutable_binding = 1;`;
- Closures can use type annotations: `|num: u32| -> u32 {...}`;
  - ```
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
    ```
- Aliasing: `type NanoSecond = u64;`;
- Labels for command loop: `'outer: loop {...`. To break: `break 'outer;`;
- Set a value from a loop directly: `let result = loop {...}`;
- For: `for n in 1..101 {...}`;
- `match` is like C `switch`, but better;
  - We can define a range: `match x {1 ..= 12 => ...}`;
  - We can use an `if`: `match x {Some(x) if x < 5 => ...}`;
  - We can or operator `|`: `match x {5 | 6 if external_variable => ...}`;
  - We can bind a new variable with `@`: `match x {... n @ 1 ..= 12 => ...}`;
- Diverging functions: functions with no return;
- Functional Features example: `let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();`;
- Assignment: can copy, move or borrow.
  - Move: pointer;
  - Borrowing: variable by reference. Mutable variables freezes and can be mutable or immutable borrowing;
- Smart pointers: `Box`, `Rc`, `RcCell`:
  - `Rc<T>`: allows multiple borrowing;
- Function pointer `fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32`.


## Interesting features
- Destructures let the code cleaner and powerful (and they can be used in a lot of ways: `struct`, `match`, `ref` etc.);
- We can use `enum` to create linked lists;
- Labels for command loop: `'outer: loop`;
- Set a value from a loop directly: `let result = loop {...`;
- The `return` command can be omitted in the last line and without `;`;
- Closures: functions that can be stored by a variable;
  - Interesting example: `9-Functions/8-closures-std-i`;
  - As parameters: `9-Functions/7-closures-as-parameters.rs`;
- Higher order functions: `9-Functions/10-higher-order-functions.rs`.
- Phantom type may be very useful, mainly for metrics;
- Lifetimes can help to optimize the use of memory;
- A very powerful macro system;
- Error handling with `Option<T>` and `Result<T, Err>` is very useful;
  - `Result<Option<i32>, ParseIntError>` looks like the best option for handling multiple errors types;
  - The possibility of easily divide an array with errors and correct results is really awesome (`18-ErrorHandling/14-interating-over-results-iv`).
- We can write bytes by their hexadecimal values or Unicode code points;
- String literals can span multiple lines;
- Raw strings `r"string"`;
- Binary strings `b"string"`;
- Quote a string with `#"quoted string"#`;
- Rc;
- Raw identifiers.


## Improve skills of:
- Generics bounds;
- `where`;
- Associated types.
- `Ord` and operators traits;
- Macro system;
- Pipes;
- Smart pointers.


### OOP Features
- `struct`: class;
  - To implement methods use: `impl <struct-name> { <functions>; }`
- Inheritance/Polymorphism: `traits`;
  - Defines functions which can be implemented by structs.


### Interesting traits
- From: `6-Conversion/0-from-and-into.rs`;
- Iterator.


## Printing
- `1-HelloWorld/3-prints.rs`.
