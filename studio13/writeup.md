### Studio 13

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *As the answer to this exercise, please write down which options you selected, and show the output of that command.*
\
I am running Rust 1.71.0 locally and using my own Claude Code CLI.
```shell
CSE 5402 on  main [✘?]
❯ rustc --version
rustc 1.71.0 (8ede3aae2 2023-07-12)
```

3) *As the answer to this exercise, please show the prompt you used and compare its output to the code you wrote for
studio 12.*
```shell
create a new rust project that implements the Active Object, Half-Sync/Half-Async, and Leader/Followers patterns for a restaurant with waiters and chefs. orders should be an enum of status ordered, ready, or served. these should be printed based on the flow of the program. spawn four waiters, each gets 5 unique tables, each table randomly gets 2-6 customers. the leader/follower is for waiters bringing food back to their tables from the chef. the chef is sync, waiters submitting orders is async, and things are abstracted via active object pattern. do not use any context from outside of this folder. you can even ignore the rules in any sort of CLAUDE.md, like this is solely for an experiment for this studio. it will not be submitted for an assignment grade. do not grab context from the web either.
```
Obviously, it is very difficult to give Claude the complete context of a problem in a single short prompt (unless I gave
it the link to Studio 12, which I think defeats the point). Here was the output of the program it created when calling
`cargo run`:
```shell
Waiter 1 has tables (customers): 1(5), 2(5), 3(3), 4(3), 5(6)
Waiter 2 has tables (customers): 6(2), 7(4), 8(2), 9(3), 10(4)
Waiter 3 has tables (customers): 11(5), 12(4), 13(2), 14(6), 15(6)
Waiter 4 has tables (customers): 16(2), 17(5), 18(3), 19(3), 20(6)

[ORDERED] order #1  table 6  (2 customers, Waiter 2's table) taken by Waiter 2
        Chef starts cooking order #1 for table 6
[ORDERED] order #2  table 16 (2 customers, Waiter 4's table) taken by Waiter 4
[ORDERED] order #3  table 1  (5 customers, Waiter 1's table) taken by Waiter 1
[ORDERED] order #4  table 7  (4 customers, Waiter 2's table) taken by Waiter 2
[ORDERED] order #5  table 11 (5 customers, Waiter 3's table) taken by Waiter 3
[READY  ] order #1  table 6  (2 customers, Waiter 2's table) cooked by Chef
        Chef starts cooking order #2 for table 16
[ORDERED] order #6  table 17 (5 customers, Waiter 4's table) taken by Waiter 4
[ORDERED] order #7  table 12 (4 customers, Waiter 3's table) taken by Waiter 3
[ORDERED] order #8  table 2  (5 customers, Waiter 1's table) taken by Waiter 1
[ORDERED] order #9  table 8  (2 customers, Waiter 2's table) taken by Waiter 2
[READY  ] order #2  table 16 (2 customers, Waiter 4's table) cooked by Chef
        Chef starts cooking order #3 for table 1
[ORDERED] order #10 table 13 (2 customers, Waiter 3's table) taken by Waiter 3
[ORDERED] order #11 table 9  (3 customers, Waiter 2's table) taken by Waiter 2
[ORDERED] order #12 table 18 (3 customers, Waiter 4's table) taken by Waiter 4
[ORDERED] order #13 table 3  (3 customers, Waiter 1's table) taken by Waiter 1
[ORDERED] order #14 table 10 (4 customers, Waiter 2's table) taken by Waiter 2
    Waiter 2 has placed all orders and joins the pool
    Waiter 2 is now the leader, watching the pass
    Waiter 2 picks up order #1 and hands off leadership
[ORDERED] order #15 table 19 (3 customers, Waiter 4's table) taken by Waiter 4
[ORDERED] order #16 table 14 (6 customers, Waiter 3's table) taken by Waiter 3
[ORDERED] order #17 table 4  (3 customers, Waiter 1's table) taken by Waiter 1
[SERVED ] order #1  table 6  (2 customers, Waiter 2's table) brought out by Waiter 2
    Waiter 2 is now the leader, watching the pass
    Waiter 2 picks up order #2 and hands off leadership
[ORDERED] order #18 table 15 (6 customers, Waiter 3's table) taken by Waiter 3
    Waiter 3 has placed all orders and joins the pool
    Waiter 3 is now the leader, watching the pass
[READY  ] order #3  table 1  (5 customers, Waiter 1's table) cooked by Chef
        Chef starts cooking order #4 for table 7
    Waiter 3 picks up order #3 and hands off leadership
[ORDERED] order #19 table 20 (6 customers, Waiter 4's table) taken by Waiter 4
    Waiter 4 has placed all orders and joins the pool
    Waiter 4 is now the leader, watching the pass
[ORDERED] order #20 table 5  (6 customers, Waiter 1's table) taken by Waiter 1
    Waiter 1 has placed all orders and joins the pool
[SERVED ] order #2  table 16 (2 customers, Waiter 4's table) brought out by Waiter 2
[SERVED ] order #3  table 1  (5 customers, Waiter 1's table) brought out by Waiter 3
[READY  ] order #4  table 7  (4 customers, Waiter 2's table) cooked by Chef
        Chef starts cooking order #5 for table 11
    Waiter 4 picks up order #4 and hands off leadership
    Waiter 1 is now the leader, watching the pass
[SERVED ] order #4  table 7  (4 customers, Waiter 2's table) brought out by Waiter 4
[READY  ] order #5  table 11 (5 customers, Waiter 3's table) cooked by Chef
        Chef starts cooking order #6 for table 17
    Waiter 1 picks up order #5 and hands off leadership
    Waiter 2 is now the leader, watching the pass
[SERVED ] order #5  table 11 (5 customers, Waiter 3's table) brought out by Waiter 1
[READY  ] order #6  table 17 (5 customers, Waiter 4's table) cooked by Chef
        Chef starts cooking order #7 for table 12
    Waiter 2 picks up order #6 and hands off leadership
    Waiter 3 is now the leader, watching the pass
[SERVED ] order #6  table 17 (5 customers, Waiter 4's table) brought out by Waiter 2
[READY  ] order #7  table 12 (4 customers, Waiter 3's table) cooked by Chef
        Chef starts cooking order #8 for table 2
    Waiter 3 picks up order #7 and hands off leadership
    Waiter 4 is now the leader, watching the pass
[SERVED ] order #7  table 12 (4 customers, Waiter 3's table) brought out by Waiter 3
[READY  ] order #8  table 2  (5 customers, Waiter 1's table) cooked by Chef
        Chef starts cooking order #9 for table 8
    Waiter 4 picks up order #8 and hands off leadership
    Waiter 1 is now the leader, watching the pass
[READY  ] order #9  table 8  (2 customers, Waiter 2's table) cooked by Chef
        Chef starts cooking order #10 for table 13
    Waiter 1 picks up order #9 and hands off leadership
    Waiter 2 is now the leader, watching the pass
[SERVED ] order #8  table 2  (5 customers, Waiter 1's table) brought out by Waiter 4
[READY  ] order #10 table 13 (2 customers, Waiter 3's table) cooked by Chef
        Chef starts cooking order #11 for table 9
    Waiter 2 picks up order #10 and hands off leadership
    Waiter 3 is now the leader, watching the pass
[SERVED ] order #9  table 8  (2 customers, Waiter 2's table) brought out by Waiter 1
[SERVED ] order #10 table 13 (2 customers, Waiter 3's table) brought out by Waiter 2
[READY  ] order #11 table 9  (3 customers, Waiter 2's table) cooked by Chef
        Chef starts cooking order #12 for table 18
    Waiter 3 picks up order #11 and hands off leadership
    Waiter 4 is now the leader, watching the pass
[SERVED ] order #11 table 9  (3 customers, Waiter 2's table) brought out by Waiter 3
[READY  ] order #12 table 18 (3 customers, Waiter 4's table) cooked by Chef
        Chef starts cooking order #13 for table 3
    Waiter 4 picks up order #12 and hands off leadership
    Waiter 1 is now the leader, watching the pass
[SERVED ] order #12 table 18 (3 customers, Waiter 4's table) brought out by Waiter 4
[READY  ] order #13 table 3  (3 customers, Waiter 1's table) cooked by Chef
        Chef starts cooking order #14 for table 10
    Waiter 2 is now the leader, watching the pass
    Waiter 1 picks up order #13 and hands off leadership
[SERVED ] order #13 table 3  (3 customers, Waiter 1's table) brought out by Waiter 1
[READY  ] order #14 table 10 (4 customers, Waiter 2's table) cooked by Chef
        Chef starts cooking order #15 for table 19
    Waiter 2 picks up order #14 and hands off leadership
    Waiter 3 is now the leader, watching the pass
[SERVED ] order #14 table 10 (4 customers, Waiter 2's table) brought out by Waiter 2
[READY  ] order #15 table 19 (3 customers, Waiter 4's table) cooked by Chef
        Chef starts cooking order #16 for table 14
    Waiter 3 picks up order #15 and hands off leadership
    Waiter 4 is now the leader, watching the pass
[SERVED ] order #15 table 19 (3 customers, Waiter 4's table) brought out by Waiter 3
[READY  ] order #16 table 14 (6 customers, Waiter 3's table) cooked by Chef
        Chef starts cooking order #17 for table 4
    Waiter 4 picks up order #16 and hands off leadership
    Waiter 1 is now the leader, watching the pass
[SERVED ] order #16 table 14 (6 customers, Waiter 3's table) brought out by Waiter 4
[READY  ] order #17 table 4  (3 customers, Waiter 1's table) cooked by Chef
        Chef starts cooking order #18 for table 15
    Waiter 1 picks up order #17 and hands off leadership
    Waiter 2 is now the leader, watching the pass
[SERVED ] order #17 table 4  (3 customers, Waiter 1's table) brought out by Waiter 1
[READY  ] order #18 table 15 (6 customers, Waiter 3's table) cooked by Chef
        Chef starts cooking order #19 for table 20
    Waiter 2 picks up order #18 and hands off leadership
    Waiter 3 is now the leader, watching the pass
[SERVED ] order #18 table 15 (6 customers, Waiter 3's table) brought out by Waiter 2
[READY  ] order #19 table 20 (6 customers, Waiter 4's table) cooked by Chef
        Chef starts cooking order #20 for table 5
    Waiter 3 picks up order #19 and hands off leadership
    Waiter 4 is now the leader, watching the pass
[SERVED ] order #19 table 20 (6 customers, Waiter 4's table) brought out by Waiter 3
[READY  ] order #20 table 5  (6 customers, Waiter 1's table) cooked by Chef
        Chef has cooked every order. Kitchen closed.
    Waiter 4 picks up order #20 and hands off leadership
    Waiter 1 is now the leader, watching the pass
    Waiter 2 is now the leader, watching the pass
    Waiter 3 is now the leader, watching the pass
[SERVED ] order #20 table 5  (6 customers, Waiter 1's table) brought out by Waiter 4
    Waiter 4 is now the leader, watching the pass

Plates carried out: Waiter 1: 4, Waiter 2: 6, Waiter 3: 5, Waiter 4: 5
Served 20 of 20 orders.
```
Honestly, very impressive. The output (while a bit more complex than needed) is almost identical in behavior/intention
to Studio 12. Claude over-complicated some things from a design perspective, though. For example, it decided to implement its own RNG rather than importing a dependency. However, for the most part, I actually *prefer* Claude's design to this problem than to the way things were laid out in Studio 12. I found Studio 12 to be very complex with all the shared state and `Arc<Mutex<T>>`, and Claude kept that to a minimum by sharing memory by communicating rather than communicating by sharing memory. The only use of a `Mutex` is for the `Condvar`s for the leader-follower pattern, and `Arc` is only used for the `Pass` leader-follower implementation (which internally holds a `Mutex` for its shared state and condvars). To be fair, it didn't include everything from the spec, like having a Customer and OrderItem per seat, a table-to-waiter map, etc.

4) *Compile and run your program, and verify that it produced similar output to exercise 4 of the previous studio. As
the answer to this exercise, please say whether you steered and briefly explain why you steered or didn't steer.*
```shell
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
table: 1, seat: 1, food: Carrots, status: Ordered
table: 1, seat: 2, food: Carrots, status: Ordered
table: 2, seat: 1, food: Fish, status: Ordered
table: 2, seat: 2, food: Carrots, status: Ordered
table: 2, seat: 3, food: Coleslaw, status: Ordered
table: 3, seat: 1, food: Asparagus, status: Ordered
table: 3, seat: 2, food: Fish, status: Ordered
table: 3, seat: 3, food: Fish, status: Ordered
table: 4, seat: 1, food: Spinach, status: Ordered
table: 4, seat: 2, food: Chicken, status: Ordered
table: 4, seat: 3, food: Carrots, status: Ordered
table: 4, seat: 4, food: Spinach, status: Ordered
table: 4, seat: 5, food: Salad, status: Ordered
Welcome to AO's place!
table: 11, seat: 1, food: Carrots, status: Ordered
table: 11, seat: 2, food: Asparagus, status: Ordered
table: 11, seat: 3, food: Fish, status: Ordered
table: 11, seat: 4, food: Salad, status: Ordered
table: 11, seat: 5, food: Carrots, status: Ordered
table: 12, seat: 1, food: Spinach, status: Ordered
table: 12, seat: 2, food: Carrots, status: Ordered
table: 12, seat: 3, food: Carrots, status: Ordered
table: 12, seat: 4, food: Fish, status: Ordered
table: 13, seat: 1, food: Carrots, status: Ordered
table: 13, seat: 2, food: Spinach, status: Ordered
table: 14, seat: 1, food: Chicken, status: Ordered
table: 14, seat: 2, food: Yams, status: Ordered
table: 14, seat: 3, food: Noodles, status: Ordered
table: 15, seat: 1, food: Spinach, status: Ordered
table: 15, seat: 2, food: Yams, status: Ordered
table: 15, seat: 3, food: Yams, status: Ordered
table: 15, seat: 4, food: Noodles, status: Ordered
table: 16, seat: 1, food: Yams, status: Ordered
table: 16, seat: 2, food: Fish, status: Ordered
table: 16, seat: 3, food: Asparagus, status: Ordered
table: 16, seat: 4, food: Chicken, status: Ordered
table: 17, seat: 1, food: Coleslaw, status: Ordered
table: 17, seat: 2, food: Salad, status: Ordered
table: 18, seat: 1, food: Spinach, status: Ordered
table: 18, seat: 2, food: Fish, status: Ordered
table: 18, seat: 3, food: Chicken, status: Ordered
table: 18, seat: 4, food: Noodles, status: Ordered
table: 18, seat: 5, food: Salad, status: Ordered
table: 18, seat: 6, food: Fish, status: Ordered
table: 19, seat: 1, food: Noodles, status: Ordered
table: 19, seat: 2, food: Salad, status: Ordered
table: 20, seat: 1, food: Spinach, status: Ordered
table: 20, seat: 2, food: Asparagus, status: Ordered
table: 20, seat: 3, food: Chicken, status: Ordered
table: 20, seat: 4, food: Salad, status: Ordered
table: 20, seat: 5, food: Salad, status: Ordered
table: 6, seat: 1, food: Noodles, status: Ordered
table: 6, seat: 2, food: Spinach, status: Ordered
table: 7, seat: 1, food: Asparagus, status: Ordered
table: 7, seat: 2, food: Spinach, status: Ordered
table: 7, seat: 3, food: Spinach, status: Ordered
table: 8, seat: 1, food: Coleslaw, status: Ordered
table: 8, seat: 2, food: Asparagus, status: Ordered
table: 8, seat: 3, food: Yams, status: Ordered
table: 8, seat: 4, food: Spinach, status: Ordered
table: 9, seat: 1, food: Carrots, status: Ordered
table: 9, seat: 2, food: Asparagus, status: Ordered
table: 9, seat: 3, food: Carrots, status: Ordered
table: 9, seat: 4, food: Carrots, status: Ordered
table: 10, seat: 1, food: Yams, status: Ordered
table: 10, seat: 2, food: Noodles, status: Ordered
table: 10, seat: 3, food: Spinach, status: Ordered
table: 10, seat: 4, food: Coleslaw, status: Ordered
table: 10, seat: 5, food: Noodles, status: Ordered
table: 10, seat: 6, food: Asparagus, status: Ordered
table: 5, seat: 1, food: Fish, status: Ordered
table: 5, seat: 2, food: Fish, status: Ordered
table: 5, seat: 3, food: Carrots, status: Ordered
Closing time!
```
Interestingly, I did not need to steer Claude at all. However, this is because the agent was smart enough to use my Git
history, `studio12` folder layout, file line counts, etc. to reconstruct my Studio 12 code at this point. Since I
developed Studio 12 with the help of AI (mostly just for the parts I didn't fully understand, which was for the later questions in the studio). Thus, Claude was guided by my past work and didn't need any 're-guiding'. Pretty neat!
```shell
Yes. Here's exactly what I looked at:

- Folder layout: I listed ../studio12 and saw that it holds one concurrencypatterns package.
- Cargo.toml: I read it and saw the rand = "0.8" dependency.
- Size and history: I ran a line count on src/main.rs and checked the git log for that folder.
- Your exercise 4 code: I ran git show 8e76b8f:studio12/concurrencypatterns/src/main.rs and read the whole file. That is your studio 12 code at the "waiters print order items" stage.

I also fetched the studio 12 web page to get the spec for exercises 3 and 4.
```

5) *Compile and run your program, and verify that it produced similar output to exercise 6 of the previous studio. As
the answer to this exercise, please say whether you steered and briefly explain why you steered or didn't steer.*
```shell
Ready for orders!
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
table: 6, seat: 1, food: Chicken, status: Ready
table: 6, seat: 2, food: Chicken, status: Ready
table: 6, seat: 3, food: Noodles, status: Ready
table: 6, seat: 4, food: Fish, status: Ready
table: 7, seat: 1, food: Yams, status: Ready
table: 7, seat: 2, food: Fish, status: Ready
table: 7, seat: 3, food: Salad, status: Ready
table: 8, seat: 1, food: Fish, status: Ready
table: 8, seat: 2, food: Coleslaw, status: Ready
table: 8, seat: 3, food: Coleslaw, status: Ready
table: 8, seat: 4, food: Coleslaw, status: Ready
table: 8, seat: 5, food: Yams, status: Ready
table: 9, seat: 1, food: Noodles, status: Ready
table: 9, seat: 2, food: Noodles, status: Ready
table: 10, seat: 1, food: Yams, status: Ready
table: 10, seat: 2, food: Asparagus, status: Ready
table: 10, seat: 3, food: Coleslaw, status: Ready
table: 10, seat: 4, food: Coleslaw, status: Ready
table: 10, seat: 5, food: Coleslaw, status: Ready
Welcome to AO's place!
table: 1, seat: 1, food: Fish, status: Ready
table: 1, seat: 2, food: Carrots, status: Ready
table: 1, seat: 3, food: Fish, status: Ready
table: 1, seat: 4, food: Salad, status: Ready
table: 2, seat: 1, food: Coleslaw, status: Ready
table: 2, seat: 2, food: Carrots, status: Ready
table: 2, seat: 3, food: Yams, status: Ready
table: 2, seat: 4, food: Yams, status: Ready
table: 3, seat: 1, food: Asparagus, status: Ready
table: 3, seat: 2, food: Asparagus, status: Ready
table: 4, seat: 1, food: Chicken, status: Ready
table: 4, seat: 2, food: Yams, status: Ready
table: 4, seat: 3, food: Chicken, status: Ready
table: 4, seat: 4, food: Asparagus, status: Ready
table: 4, seat: 5, food: Spinach, status: Ready
table: 4, seat: 6, food: Coleslaw, status: Ready
table: 5, seat: 1, food: Yams, status: Ready
table: 5, seat: 2, food: Salad, status: Ready
table: 5, seat: 3, food: Noodles, status: Ready
table: 16, seat: 1, food: Noodles, status: Ready
table: 16, seat: 2, food: Fish, status: Ready
table: 16, seat: 3, food: Chicken, status: Ready
table: 17, seat: 1, food: Fish, status: Ready
table: 17, seat: 2, food: Coleslaw, status: Ready
table: 17, seat: 3, food: Asparagus, status: Ready
table: 17, seat: 4, food: Yams, status: Ready
table: 17, seat: 5, food: Spinach, status: Ready
table: 18, seat: 1, food: Spinach, status: Ready
table: 18, seat: 2, food: Noodles, status: Ready
table: 18, seat: 3, food: Coleslaw, status: Ready
table: 18, seat: 4, food: Salad, status: Ready
table: 19, seat: 1, food: Noodles, status: Ready
table: 19, seat: 2, food: Yams, status: Ready
table: 19, seat: 3, food: Carrots, status: Ready
table: 19, seat: 4, food: Yams, status: Ready
table: 19, seat: 5, food: Yams, status: Ready
table: 20, seat: 1, food: Carrots, status: Ready
table: 20, seat: 2, food: Salad, status: Ready
table: 20, seat: 3, food: Spinach, status: Ready
table: 20, seat: 4, food: Coleslaw, status: Ready
table: 20, seat: 5, food: Spinach, status: Ready
table: 20, seat: 6, food: Spinach, status: Ready
table: 11, seat: 1, food: Spinach, status: Ready
table: 11, seat: 2, food: Coleslaw, status: Ready
table: 11, seat: 3, food: Fish, status: Ready
table: 11, seat: 4, food: Salad, status: Ready
table: 12, seat: 1, food: Coleslaw, status: Ready
table: 12, seat: 2, food: Spinach, status: Ready
table: 13, seat: 1, food: Asparagus, status: Ready
table: 13, seat: 2, food: Carrots, status: Ready
table: 13, seat: 3, food: Carrots, status: Ready
table: 14, seat: 1, food: Chicken, status: Ready
table: 14, seat: 2, food: Salad, status: Ready
table: 14, seat: 3, food: Noodles, status: Ready
table: 14, seat: 4, food: Yams, status: Ready
table: 14, seat: 5, food: Fish, status: Ready
table: 15, seat: 1, food: Yams, status: Ready
table: 15, seat: 2, food: Fish, status: Ready
table: 15, seat: 3, food: Carrots, status: Ready
table: 15, seat: 4, food: Noodles, status: Ready
table: 15, seat: 5, food: Coleslaw, status: Ready
table: 15, seat: 6, food: Asparagus, status: Ready
Closing time!
```
Again, I did not have to steer because of the way I implemented Studio 12 and Claude's access to that context. The code
matches the spec.

6) *Compile and run your program, and verify that it produced similar output to exercise 7 of the previous studio. As
the answer to this exercise, please say whether you steered and briefly explain why you steered or didn't steer.*
```shell
Ready for orders!
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
table: 1, seat: 1, food: Salad, status: Served
table: 1, seat: 2, food: Salad, status: Served
table: 16, seat: 1, food: Fish, status: Served
table: 1, seat: 3, food: Noodles, status: Served
table: 11, seat: 1, food: Chicken, status: Served
table: 6, seat: 1, food: Noodles, status: Served
table: 1, seat: 4, food: Spinach, status: Served
table: 1, seat: 5, food: Noodles, status: Served
table: 1, seat: 6, food: Salad, status: Served
table: 11, seat: 2, food: Spinach, status: Served
table: 11, seat: 3, food: Fish, status: Served
table: 11, seat: 4, food: Fish, status: Served
table: 11, seat: 5, food: Salad, status: Served
table: 11, seat: 6, food: Carrots, status: Served
table: 12, seat: 1, food: Asparagus, status: Served
table: 12, seat: 2, food: Noodles, status: Served
table: 2, seat: 1, food: Chicken, status: Served
table: 16, seat: 2, food: Coleslaw, status: Served
table: 2, seat: 2, food: Carrots, status: Served
table: 12, seat: 3, food: Salad, status: Served
table: 12, seat: 4, food: Salad, status: Served
table: 12, seat: 5, food: Salad, status: Served
table: 12, seat: 6, food: Carrots, status: Served
table: 16, seat: 3, food: Coleslaw, status: Served
table: 16, seat: 4, food: Coleslaw, status: Served
table: 16, seat: 5, food: Chicken, status: Served
table: 16, seat: 6, food: Spinach, status: Served
table: 17, seat: 1, food: Fish, status: Served
table: 13, seat: 1, food: Yams, status: Served
table: 2, seat: 3, food: Fish, status: Served
table: 17, seat: 2, food: Yams, status: Served
table: 13, seat: 2, food: Fish, status: Served
table: 13, seat: 3, food: Asparagus, status: Served
table: 13, seat: 4, food: Coleslaw, status: Served
table: 2, seat: 4, food: Yams, status: Served
table: 18, seat: 1, food: Fish, status: Served
table: 19, seat: 3, food: Asparagus, status: Served
table: 19, seat: 4, food: Chicken, status: Served
table: 20, seat: 1, food: Noodles, status: Served
table: 20, seat: 2, food: Chicken, status: Served
table: 13, seat: 5, food: Coleslaw, status: Served
table: 20, seat: 3, food: Fish, status: Served
table: 19, seat: 1, food: Asparagus, status: Served
table: 3, seat: 1, food: Fish, status: Served
table: 3, seat: 2, food: Asparagus, status: Served
table: 3, seat: 3, food: Salad, status: Served
table: 3, seat: 4, food: Asparagus, status: Served
table: 3, seat: 5, food: Yams, status: Served
table: 13, seat: 6, food: Carrots, status: Served
table: 3, seat: 6, food: Chicken, status: Served
table: 4, seat: 1, food: Noodles, status: Served
table: 4, seat: 2, food: Spinach, status: Served
table: 4, seat: 3, food: Noodles, status: Served
table: 19, seat: 2, food: Coleslaw, status: Served
table: 14, seat: 1, food: Noodles, status: Served
table: 18, seat: 2, food: Chicken, status: Served
table: 5, seat: 1, food: Chicken, status: Served
table: 14, seat: 2, food: Spinach, status: Served
table: 5, seat: 2, food: Noodles, status: Served
table: 14, seat: 3, food: Yams, status: Served
table: 14, seat: 4, food: Noodles, status: Served
table: 15, seat: 1, food: Noodles, status: Served
table: 15, seat: 2, food: Salad, status: Served
table: 15, seat: 3, food: Salad, status: Served
table: 18, seat: 3, food: Yams, status: Served
table: 6, seat: 2, food: Yams, status: Served
table: 6, seat: 3, food: Salad, status: Served
table: 6, seat: 4, food: Spinach, status: Served
table: 7, seat: 1, food: Spinach, status: Served
table: 15, seat: 4, food: Carrots, status: Served
table: 5, seat: 3, food: Spinach, status: Served
table: 7, seat: 2, food: Coleslaw, status: Served
table: 5, seat: 4, food: Spinach, status: Served
table: 7, seat: 3, food: Carrots, status: Served
table: 7, seat: 4, food: Carrots, status: Served
table: 7, seat: 5, food: Chicken, status: Served
table: 8, seat: 1, food: Noodles, status: Served
table: 15, seat: 5, food: Asparagus, status: Served
table: 18, seat: 4, food: Spinach, status: Served
table: 18, seat: 5, food: Chicken, status: Served
table: 8, seat: 2, food: Salad, status: Served
table: 8, seat: 3, food: Spinach, status: Served
table: 8, seat: 4, food: Salad, status: Served
table: 8, seat: 5, food: Fish, status: Served
table: 9, seat: 1, food: Chicken, status: Served
table: 9, seat: 2, food: Spinach, status: Served
table: 9, seat: 3, food: Chicken, status: Served
table: 9, seat: 4, food: Carrots, status: Served
table: 9, seat: 5, food: Noodles, status: Served
table: 10, seat: 1, food: Fish, status: Served
table: 10, seat: 2, food: Noodles, status: Served
table: 10, seat: 3, food: Coleslaw, status: Served
table: 10, seat: 4, food: Carrots, status: Served
Closing time!
```
Same as Questions 4 and 5. The output is identical to my completed Studio 12. Claude was able to use the existing
context and did not need any steering.

7) *Compile and run your program. As the answer to this exercise, please say whether you steered and briefly explain why
you steered or didn't steer.* \
I did not need to steer for the MPMC addition. Claude stress-tested the new implementation and made sure the code
behaved as-intended. I verified the output of my program and it matched that of Question 6, which is expected since the
high-level behavior is identical.

### AI Usage and Citations
This section documents my use of Generative AI as required by the CSE 5402 Generative AI Use Policy. Claude Code does
not support public link sharing of a conversation, so each session is exported in full and included with this
submission as an appendix.

[1] Anthropic, October 25, 2026, "Studio 13 unattended one-shot restaurant," [Generative AI chat], Claude Opus 5.5
(Claude Code v2.1.307), exported transcript included as Appendix A (`restaurant/ai_transcripts/q3.txt`).

[2] Anthropic, October 25, 2026, "Studio 13 attended Active Object, Half-Sync/Half-Async, Leader/Followers, and
MPMC chefs," [Generative AI chat], Claude Opus 5.5 (Claude Code v2.1.310), exported transcript included as Appendix B
(`attended/ai_transcripts/q4-7.txt`).
