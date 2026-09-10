### Studio 04

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Compile and run your program and as the answer to this exercise please show the output that was produced.*
```shell
Values: 45 45 45
```

3) *Compile and run your program and as the answer to this exercise please describe the error that occurred and explain
briefly why it is happening.*
```shell
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> src/main.rs:6:16
  |
6 |     let aa = y == z;
  |                ^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
  = note: required for `&{integer}` to implement `PartialEq<&&{integer}>`
help: consider dereferencing here
  |
6 |     let aa = y == *z;
  |                   +

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustreferences` (bin "rustreferences") due to 1 previous error
```
This error is happening because we are attempting to compare a reference and a reference to a reference, which is an
undefined operation.

4) *Compile and run your program and as the answer to this exercise please show the output the program produced.*
```shell
Values: 45 45 45
Comparison: true
```

5) *Compile and run your program and as the answer to this exercise please explain briefly why the program produced the
same output as in the previous exercise.* \
In the previous exercise, a pointer to x was compared with a dereferenced pointer to y, where y points to x, meaning the
two values point to/reference the same memory address. This exercise compares the underlying value of x to itself, which
will always be true regardless of x's value.

6) *Compile and run your program and as the answer to this exercise please describe the error that occurred and explain
briefly why it is happening.* 
```shell
error[E0506]: cannot assign to `x` because it is borrowed
  --> src/main.rs:23:5
   |
15 |     let y = &x;
   |             -- `x` is borrowed here
...
23 |     x = 33;
   |     ^^^^^^ `x` is assigned to here but it was already borrowed
24 |     println!("New value: {} {} {}", x, y, z);
   |                                        - borrow later used here
```
This error is occurring because there is already a reference to `x`. Therefore, the referent cannot be mutated for as
long as `y`'s lifetime, which is extended by the `println!` statement.

7) *Compile and run your program and as the answer to this exercise please describe the error that occurred and explain
briefly why it is happening.* 
```shell
error[E0506]: cannot assign to `*y` because it is borrowed
  --> src/main.rs:23:5
   |
16 |     let z = &y;
   |             -- `*y` is borrowed here
...
23 |     *y = 33;
   |     ^^^^^^^ `*y` is assigned to here but it was already borrowed
24 |     println!("New value: {} {} {}", x, y, z);
   |                                           - borrow later used here

error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
  --> src/main.rs:24:37
   |
15 |     let mut y = &mut x;
   |                 ------ mutable borrow occurs here
...
24 |     println!("New value: {} {} {}", x, y, z);
   |                                     ^  - mutable borrow later used here
   |                                     |
   |                                     immutable borrow occurs here
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
```

The first error occurs because `z` borrows `y`, meaning only immutable references can exist for `z`'s lifetime.
Therefore, `y` cannot be mutated (i.e., borrowed as mutable). Additionally, `x` cannot be borrowed as mutable because of
the `println!` statement that extends its lifetime and borrows it as immutable, causing an immutable and mutable borrow
within the same lifetime.

8) *Compile and run your program and as the answer to this exercise please show the output that the program produced.*
```shell
Value: 45
New value: 33
```

9) *Compile and run your program and as the answer to this exercise please show the output that the program produced.*
```shell
aliases: hello world
now aliases: hey there
str: hey there
```
