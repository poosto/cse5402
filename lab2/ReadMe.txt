================================================================================
SECTION 1: IDENTIFICATION AND DESIGN
================================================================================

CSE 5402 Fall 2025 Lab 2: "They have their exits and their entrances"

Team members:
- Preston Meek: meek@wustl.edu

--------------------------------------------------------------------------------
Summary
--------------------------------------------------------------------------------

The approach to this lab was a bit different from Lab 1. Rather than starting
from a fresh codebase, this lab was about moving existing code to a
struct-based design instead of many free functions. I followed the spec in
order and ran the regression tests after finishing major chunks, like a full
struct implementation. This made sure the behavior matched Lab 1 (except where
the Lab 2 spec said otherwise) and kept the migration smooth. The Lab 1 code
was already quite modular, so much of the migration was removing function
input and output parameters and using some form of self instead.

The program reads a script file. The script file names one or more scene
fragment configuration files, and its [scene] lines give scene titles. Each
configuration file names the characters in that fragment and the part file that
holds each character's numbered lines. The program reads every part file,
recites each fragment's lines in line number order, and announces which
characters enter and exit between fragments.

Control flows through these stages:

    1. main calls run. run calls parse_args, which checks the command line,
       stores the script file name, and sets the WHINGE_MODE static atomic
       Boolean if the optional "whinge" argument was given.
    2. Play::prepare calls Play::read_config to read the script file into a
       ScriptConfig vector of (bool, String) tuples. A true tuple holds a scene
       title from a [scene] line. A false tuple holds a configuration file name.
    3. Play::process_config makes one SceneFragment for each configuration file.
       A scene title goes to the next fragment only. Later fragments in the same
       scene get an empty title, so the title is printed once per scene.
    4. SceneFragment::prepare reads its configuration file into a PlayConfig
       vector of (character name, part file name) tuples. It makes one Player
       for each character. Player::prepare reads the part file and sorts the
       player's lines by line number. The fragment then sorts its players.
    5. Play::recite walks the fragments in order. For each one it prints the
       title (if any) and the entrances, recites the lines, and prints the
       exits. Only players who are not in both neighboring fragments enter or
       exit.
    6. SceneFragment::recite merges the players' lines. It keeps asking every
       player for its next_line, picks the smallest line number, and calls
       speak on that player. speak prints the player's name first whenever the
       speaker changes.
    7. run returns a Result<(), u8>. main wraps it in a ReturnWrapper. When main
       returns, ReturnWrapper::report prints "Error: N" to stderr if N is not 0,
       and returns N as the exit code.

The program has three exit codes:

    0   success
    1   ERR_BADLY_FORMED_COMMAND_LINE: the command line is wrong. The usage
        message is printed.
    2   ERR_FAILED_TO_GENERATE_SCRIPT: the command line is fine, but the play
        could not be built. Each cause prints its own ERROR message.

Problems that the program can handle are skipped, and the program keeps going.
In whinge mode it also prints a WARNING about each one. All ERROR and WARNING
messages go to stderr. Only the recited play goes to stdout.

--------------------------------------------------------------------------------
Modules
--------------------------------------------------------------------------------

    src/main.rs                 Command line handling and the program's entry
                                point. Declares the lab2 module and defines
                                usage, parse_args, run, and main. The WHINGE_ARG
                                and FALLBACK_PROGRAM_NAME constants are private
                                to this file, since only usage and parse_args
                                need them.

    src/lab2/mod.rs             Declares the five modules below.

    src/lab2/declarations.rs    Items used by more than one file: the command
                                line count and index constants, the two u8
                                error codes, the WHINGE_MODE static AtomicBool,
                                BYTES_READ_AT_EOF, and grab_trimmed_file_lines.
                                The spec moved grab_trimmed_file_lines here
                                because Play, SceneFragment, and Player all read
                                files with it.

    src/lab2/play.rs            The Play struct. It reads the script file and
                                holds a vector of SceneFragment structs. The
                                ScriptConfig and Fragments types are private
                                here.

    src/lab2/scene_fragment.rs  The SceneFragment struct. It reads one
                                configuration file, holds a vector of Player
                                structs, recites their lines, and prints
                                entrances and exits. The PlayConfig type and the
                                configuration line index constants are private
                                here.

    src/lab2/player.rs          The Player struct. It reads one part file, keeps
                                its lines sorted, and speaks them one at a time.
                                It also implements PartialEq, Eq, PartialOrd,
                                and Ord so a vector of players can be sorted.

    src/lab2/return_wrapper.rs  The ReturnWrapper struct and its Termination
                                impl, which turn the program's result into the
                                exit code.

The split follows the same rule as Lab 1. Items used in only one file are
declared in that file. Only items shared across files are made public. Fields
that another module needs to read (Player::name and SceneFragment::title) are
pub(crate) rather than pub, so code outside the crate cannot see them. Every
file inside lab2 imports from its siblings with super::, which keeps the files
from depending on where lab2 is mounted in the crate.

--------------------------------------------------------------------------------
Design decisions
--------------------------------------------------------------------------------

1.  An unreadable part file is now fatal. In Lab 1, a part file that could not
    be opened or read was treated as a badly formed configuration line. That
    character was skipped and the program went on. The Lab 2 spec says
    SceneFragment::process_config should return an error if a player's prepare
    fails, so now the program prints an ERROR naming the part file and the
    character, and exits with code 2. For example, ../lab1/dir_part_config.txt
    recited its other character in Lab 1, but fails in Lab 2. Also,
    process_config calls prepare on the player before pushing it into the
    vector, not after as the spec words it. This way a player that fails to
    prepare is never added.

2.  Blank lines are ignored silently rather than warned about. This holds for
    script files, configuration files, and part files. A trailing newline at
    the end of a file is normal, not a defect, and warning on it would make
    noise on well formed input. Since grab_trimmed_file_lines has already
    trimmed every line, whitespace-only lines are exactly the empty ones, so
    each read_config drops them with a single retain call.

3.  Severity is decided by the caller, not by grab_trimmed_file_lines. Its
    signature has no room for a severity parameter. So it prints its open and
    read failures as whinge-gated warnings that carry the io::Error text. Each
    caller then prints its own ERROR message, which always appears. The trade
    off is that the message that always prints is less specific than the
    whinge-gated one.

4.  Messages come in two families so no two failures look alike. "ERROR: " is
    for problems that stop the program and always prints. "WARNING: " is for
    problems the program handled and only prints in whinge mode. Each message
    names the file, character, line, or token it is about. This matters
    because error code 2 now has many causes: a script file that cannot be
    read, is empty, names no configuration files, or gives no [scene] title
    before its first configuration file, and a configuration file that cannot
    be read, is empty, has no valid character lines, or names a part file that
    cannot be read.

5.  Two diversions from the assignment's instructions, kept from Lab 1. Instead
    of using String references, &str slices were used since this is much more
    idiomatic Rust and does not force heap-allocated String structs to be used.
    Additionally, instead of a match statement with a single branch,
    destructuring for loops were used instead since they are more syntactically
    concise. In particular, the for-loop case was a warning from Clippy.

6.  A configuration file that holds only malformed lines is an error.
    This matches the Lab 1 rule that a configuration needs at least one
    valid character definition (the Lab 2 spec does not restate this rule).
    The Lab 2 spec explicitly states that an empty script file is an error,
    and Lab 1 made an empty configuration file an error. From the end user's
    perspective, an empty configuration file is identical to one with only
    malformed lines, since no character will enter or speak in either case.

7.  Play derives Default as well as defining new.
    Clippy suggested implementing the Default trait alongside fn new(),
    since new() takes zero parameters. However, the spec specifically asks
    for new(), so I kept that method and derived Default, which works because
    every field of Play also implements Default. I would have removed the
    explicit new() implementation, but I wanted to follow the spec's
    instructions.

--------------------------------------------------------------------------------
Structs
--------------------------------------------------------------------------------

TODO: summarize how you refactored the Lab 1 code into the Player and Play
structs and their implementations, and the design challenges you hit and how
you solved them. Points you might cover:
  - Which Lab 1 functions became methods of which struct, and which stayed free
    functions (grab_trimmed_file_lines moved to declarations.rs).
  - Why add_script_line takes &mut self rather than mut self.
  - The String vs &mut str comparison error you hit while writing speak, and
    why speak takes most_recent_speaker as &mut String.
  - Why name is pub(crate) while lines and line_index stay private.
  - Changing the whinge println! calls to eprintln! so whinge output goes to
    stderr, as the spec now requires.
  - The usize::MAX overflow and hang that came back during the refactor (see
    Testing below).


--------------------------------------------------------------------------------
Return Wrapper
--------------------------------------------------------------------------------

  - ReturnWrapper has one u8 field. ReturnWrapper::new turns Ok into 0 and
    keeps an Err's code. report prints "Error: N" to stderr when N is not 0,
    then returns ExitCode::from(N) in both cases. ExitCode::from(0) is
    effectively a success code.
  - When adding the ReturnWrapper, I noticed a lot of boilerplate from bubbling
    up the Err values and unwrapping the Ok values. This is exactly what the
    question mark operator (?) solves, but it only works for core
    types like Result<T, E> and Option<T>. There is no way to implement it
    [in stable Rust] for my ReturnWrapper.
    To get around this, all of main's core logic lives in the run() function,
    which returns a regular Result<(), u8>. This means the logic of the
    main program can use ? to bubble up error codes to the return value of
    run(), which can be caught *once* by main() and wrapped in a ReturnWrapper.
    This differs slightly from the spec's wording, but I think this is the
    more ergonomic approach.
  - I realized that the Termination impl for Result<T, E>
    always returns ExitCode::FAILURE (1 on Linux) for an Err
    since it is meant to work with any error type E, not just a u8,
    so it can't reliably return E. It uses debug printing ({:?}) to print
    out the error in the "Error: {:?}" message.
  - Exit codes were verified using echo $?. See the Testing section for
    more details.

--------------------------------------------------------------------------------
Scene Fragments
--------------------------------------------------------------------------------

TODO: summarize how you extended the solution to handle several scene fragments
in a row, and the design challenges. Points you might cover:
  - The Lab 1 read_config still took the first line of a configuration file as
    the title. Lab 2 configuration files have no title line, so the first
    character was dropped. Titles now come from [scene] lines in the script.
  - Why Play::recite uses an index loop rather than an iterator (it needs to
    borrow the previous and next fragments while calling recite on the current
    one).
  - How players_not_in matches players by name with a HashSet, and why it
    cannot use Player's == (all silent players compare equal).
  - Why exits are printed in reverse order (rev() on a DoubleEndedIterator).
  - The blank line rules around titles, entrances, and exits.

A core component of Lab 2 was extending Lab 1's functionality to handle
multiple scene fragments rather than a single configuration file. The core
of this functionality is in the SceneFragment struct, which was based on
the original Play struct (which itself was based on Lab 1's implementation).
Core challenges and design points are as follows:
 - In Lab 1's code, the recite function was part of main.rs and took in
   an already sorted list of play lines. Now, recite was a method in
   scene_fragment.rs that only takes &mut self. A SceneFragment contains
   multiple Players, each of which have their own sorted list of lines
   (but these lines don't have a total sequential ordering). Thus, the
   recite method must effectively perform a merge by grabbing the
   global minimum line number across each Player every iteration.
- In Lab 1, read_config took the first line of a configuration file as
  the title. In Lab 2, the title is part of a script config, and individual
  config files don't contain a title anymore.
- In Play::recite, I originally tried using iterators in a declarative
  style to handle calling each SceneFragment's recite method. However,
  Play::recite needs to borrow the previous and next fragments, and
  mutably borrow the current fragment. Thus, I would need to use
  iter_mut(), which means I can't immutably borrow any other SceneFragments
  during that iterator's lifetime. Thus, indices were the easier approach.
- I made a SceneFragment::players_not_in helper method to efficiently
  return the Players in &self but not in &other. This collects the other
  Player list into a HashSet for O(1) comparisons against &self. Also,
  note that this is based on the *names* of Players matching rather than
  their line numbers, so the == operator cannot be used.
- The players_not_in method returns a DoubleEndedIterator since exits
  happen in reverse order, which requires rev() to be implemented.
- The enter_with and exit_with methods carefully handle newlines for
  different states for visual purposes.
- The recite method delivers lines by repeatedly scanning every Player's
next_line for the smallest line number and calling speak on that player, so
reciting n lines across c characters takes O(n * c) time. Since each player's
lines are already sorted, this is a c-way merge, and keeping the players' next
line numbers in a BinaryHeap (pushing back only the player that just spoke)
would bring it down to O(n log c). With only a handful of characters in a scene,
the simpler scan was kept: it is just as fast in practice, easier to read, and
follows the spec's description of delivering lines through next_line and speak.
I didn't run actual benchmarks (because I don't think micro-optimizations are
the point of this lab...), but I'm curious about the performance difference
since both are contiguous in memory and should have good cache locality,
especially for small c.

Through this implementation, I learned about marker traits, namely Eq. PartialEq
always returns a boolean, but does not guarantee reflexivity. For example, with
floats, NaN != NaN. The Eq trait has no actual methods to implement, but it
means that your PartialEq implementation guarantees reflexivity. This is very
useful for generic code that works off of trait requirements. Ord is *not* a
marker trait, unlike Eq. Two things that cannot be compared for equality can
simply be called unequal, meaning PartialEq and Eq both only have two states
(true/false). When comparing two things for ordering, there are four possible
states: less than, greater than, equal to, or incomparable. The Ord trait
removes the incomparable state, therefore it has a different return type and
possibly its own logic, so it needs its own method. If you have a struct that
implements Ord, your PartialOrd implementation can simply be
Some(Ord::cmp(self, other)). For this project, a Player can always be compared,
and Player equivalence guarantees reflexivity, so it implements Eq and Ord.

TODO: explain how tied line numbers are handled. Across players, the player
sorted earlier speaks first, so players tied on their first line keep their
configuration file order (Vec::sort is stable). Within one player, lines with
the same number keep their part file order, because Player::prepare uses
sort_by_key on the line number only, not sort on the whole (line number, text)
tuple. Use crlf_config.txt as the example: Lab 1 printed King's line first (by
name), and Lab 2 prints Sentinel's line first (configuration order).

The script file rules the program enforces:
  - A [scene] line gives its title to the next configuration file only. If two
    [scene] lines come in a row, the second title replaces the first.
  - A [scene] line with no title is skipped, with a warning in whinge mode.
  - Tokens after a configuration file name are ignored, with a warning in
    whinge mode.
  - The directive must be exactly "[scene]". A line like "[Scene] Title" is
    read as the configuration file name "[Scene]".
  - A script must name at least one configuration file, and must give a
    [scene] title before its first one. Otherwise it is error code 2.

--------------------------------------------------------------------------------
Insights, observations, and questions
--------------------------------------------------------------------------------

-   Rust guarantees that passing &title to SceneFragment::new and calling
    title.clear() is safe. If the struct stored that reference internally,
    lifetime annotations would be required and the compiler would not allow
    a call to title.clear(). This is *not* true in C++, where clearing the
    title would be completely allowed by the compiler, but would cause
    undesired behavior at runtime if the class stored the reference. There is
    no way to know if the class stores the reference itself or [properly]
    copies it to its own std::string from the call site alone.
-   Marker traits are a neat way to describe generic enforcements across
    types. Something like the Eq marker trait does not translate well to C++,
    since a type satisfies a concept just by having the right operations
    (structural), rather than by explicitly opting in like a Rust impl block
    (nominal). This means anything that is Eq rather than just PartialEq
    would need an opt-in constexpr bool (like enable_borrowed_range in
    std::ranges) or something similar, which works but is a bit messier.
-   Why have a new() associated function with no parameters rather than
    implement the Default trait? From what I understand, Clippy
    suggests Default so that it can work with generic code.
-   Why not use an enum for error codes rather than consts? Enums seem more
    flexible since they can carry different data depending on the error, such
    as a &str or a u8.
-   The ReturnWrapper type is a neat design pattern for getting custom
    behavior when exiting the program! Having an 'inner main' function allows
    the rest of my code to use regular Result<T, E> types and then have a
    single mapping point before exiting the program.

================================================================================
SECTION 2: BUILDING AND RUNNING ON THE CEC LINUX LAB MACHINES
================================================================================

1.  Log in to the CEC shell server with your WUSTL Key username, then move to
    a dedicated Linux Lab machine with qlogin:

        ssh <username>@shell.cec.wustl.edu
        qlogin

    The course asks that programs not be compiled on shell.cec.wustl.edu
    itself, since compiles can be computationally intensive. qlogin gives one
    core by default. "qlogin -c 2" asks for two.

2.  Copy the submitted archive to a CEC Linux Lab machine and unpack it:

        unzip lab2.zip
        cd lab2

    The unpacked directory contains Cargo.toml, Cargo.lock, the src directory,
    the provided Hamlet script, configuration, and part files, the test_inputs
    directory of hand-made test files, run_tests.sh, gen_output.sh,
    fuzz_test.py, test_output.txt, the ai_transcripts directory holding the
    exported Generative AI conversations cited in Section 4, and this
    ReadMe.txt.

3.  Make the course Rust toolchain available:

        module add rust-2023

    This provides rustc and cargo version 1.71.0. Confirm with:

        rustc --version

4.  Build the program:

        cargo build

    The build completes with no errors and no warnings. The executable is
    written to target/debug/lab2.

5.  Run the program:

        cargo run -q <script_file_name> [whinge]

    for example:

        cargo run -q partial_hamlet_act_ii_script.txt
        cargo run -q partial_hamlet_act_ii_script.txt whinge

    The optional second argument must be exactly the word "whinge". It turns on
    warning messages, printed to stderr, about problems the program handled.
    Running the program with no script file name prints a usage message and
    returns error code 1.

    File names inside a script or configuration file are opened relative to
    the directory the program is run from. So the hand-made tests must be run
    from inside test_inputs:

        cd test_inputs
        cargo run -q macbeth_script.txt
        cargo run -q fuzz_script.txt whinge
        cd ..

    The program can also be run directly once built:

        ./target/debug/lab2 partial_hamlet_act_ii_script.txt

    To see the exit code of the last run:

        echo $?

6.  To run the unit tests for Player's ordering traits and for the entrance and
    exit logic:

        cargo test

7.  To run the full regression suite described in Section 3:

        sh run_tests.sh

    It fails if the build prints any warning, runs the unit tests, runs the
    other test cases, prints PASS or FAIL for each, and exits non-zero if any
    case fails.

Note on the package configuration: Cargo.toml specifies edition = "2021" and
Cargo.lock is written in lockfile format version 3, both of which the 1.71.0
tools on the lab machines accept.

================================================================================
SECTION 3: DEVELOPMENT AND TESTING
================================================================================

--------------------------------------------------------------------------------
Development stages
--------------------------------------------------------------------------------

TODO: describe each stage of how you refactored and extended your Lab 1
solution (the spec asks for this). The stages followed the spec's order:
  1. Structs: Player and Play replace the Lab 1 tuples and free functions.
     Whinge output moves to stderr. The program is checked against the Lab 1
     input files in ../lab1.
  2. Return Wrapper: main returns a ReturnWrapper, and each exit code is
     checked with echo $?.
  3. Scene Fragments: Player gets its ordering traits, the old Play becomes
     SceneFragment, and a new Play reads a script file of [scene] lines and
     configuration files, and prints entrances and exits.
  4. Cleanup: header and function comments, the tie order
     fix, and the junk-only configuration check.
Say what you did, and what you checked, at the end of each stage.

--------------------------------------------------------------------------------
File formats used
--------------------------------------------------------------------------------

A script file has one entry per non-blank line. A line that starts with the
token [scene] gives a scene title (the rest of the line). Any other line names
a scene fragment configuration file (its first token).

A configuration file has one line per character, holding two whitespace
delimited tokens: the character's name and the name of that character's part
file. Unlike Lab 1, it has no title line.

A part file has one line per spoken line: a usize line number, whitespace, and
then the text of the line.

--------------------------------------------------------------------------------
Testing
--------------------------------------------------------------------------------

Testing was done in four layers:

    1. Running the provided Hamlet files and checking the output by eye and by
       diff against hamlet_act_ii_scene_1.txt.
    2. Running the Lab 1 input files in ../lab1 after the Structs and Return
       Wrapper stages, to confirm the program still ran as it did in Lab 1.
    3. run_tests.sh, a regression suite of 46 cases over hand-made inputs in
       test_inputs. Each case checks the exit code, the "Error: N" line, and
       stdout and stderr separately, so whinge output must reach stderr and
       never stdout. 16 of the checks are unit tests run by cargo test.
    4. fuzz_test.py, a randomized tester described below.

Problems that testing found, and how they were fixed:

  - The usize::MAX overflow came back during the Structs refactor. A line
    numbered 18446744073709551615 made expected_line = line + 1 panic, and in
    whinge mode the new missing line loop would have printed about 1.8 x 10^19
    warnings. Lab 1 had fixed this, and the fix was lost when recite was
    rewritten. recite now uses saturating_add and reports each gap as one
    range ("Lines 13 to 18446744073709551614 are missing").
  - SceneFragment::read_config still took the first line of each
    configuration file as the title, as in Lab 1. Lab 2 configuration files
    have no title line, so the first character of every fragment was dropped.
    The title now comes from the script file's [scene] lines instead.
  - Lab 1 inputs that named an unreadable part file (dir_part_config.txt,
    fuzz_config.txt) now stop with error code 2. This is expected, see design
    decision 1.
  - crlf_config.txt now alternates Sentinel and King lines in configuration
    order, where Lab 1 broke ties by name. This led to the tied line number
    test below.
  - Changing Player::prepare from sort to sort_by_key changed the order of
    lines that share a number. The fuzzer's model was updated to match, and
    tie_script.txt was added to pin down the order.

Exit codes were checked with echo $? and are also checked by every
run_tests.sh case: 0 on success, 1 for a badly formed command line, and 2 for
each cause of a script generation failure.

--------------------------------------------------------------------------------
Well formed input
--------------------------------------------------------------------------------

partial_hamlet_act_ii_script.txt
    The provided script, with hamlet_ii_1a_config.txt, hamlet_ii_1b_config.txt,
    hamlet_ii_2a_config.txt, and their eight part files. Its Scene I output is
    compared against hamlet_act_ii_scene_1.txt (ignoring the trailing space the
    sample puts after each speaker name), and the full output against
    test_inputs/expected_hamlet.txt. Whinge mode reports only "Line 0 is
    missing", and only on stderr.

recurring_script.txt (cast_a, cast_b, and cast_c configurations)
    Three fragments over two scenes. Beta stays on stage across the first two
    fragments, Alpha exits and comes back, and the second scene's title is
    printed only before its first fragment. Compared against
    expected_recurring.txt.

repeat_script.txt
    The same configuration twice in a row. No one exits or enters between the
    two fragments. Compared against expected_repeat.txt.

tie_script.txt (tie_config.txt, tie_amy.txt, tie_bob.txt, tie_cara.txt)
    Amy and Cara both start at line 0, and all three players have a line 7.
    Amy has two lines numbered 7. The output must keep configuration order
    between tied players and part file order within Amy's lines. Compared
    against expected_ties.txt. Whinge mode reports "Line 7 is duplicated".

macbeth_script.txt (macbeth_ii_1_config.txt)
    A second play. Banquo.txt lists its lines out of order, which checks that
    lines are sorted by number. Servant has an empty part file, so Servant
    enters and exits but is never announced as a speaker.

--------------------------------------------------------------------------------
Badly formed input that the program recovers from
--------------------------------------------------------------------------------

Each of these exits with code 0 and recites the play. In whinge mode each also
prints a warning on stderr about what was skipped. Without whinge, stderr is
empty.

whitespace_script.txt     Blank and whitespace-only lines, a tab and extra
                          spaces inside the [scene] title, and a padded
                          configuration file name.
crlf_script.txt           A script, configuration, and part file with Windows
                          CRLF line endings. No carriage return reaches the
                          output.
numeric_script.txt        What parse::<usize> accepts and rejects in Ghost.txt:
                          "007" and "+5" parse, usize::MAX parses and sorts
                          last, "1_0", "3.5", and "0x10" are skipped, a number
                          with no text is skipped, and interior tabs in a line
                          are kept.
bare_scene_script.txt     [scene] lines with no title are skipped.
extra_tokens_script.txt   Extra tokens after a configuration file name are
                          ignored.
double_title_script.txt   Two [scene] lines in a row. Only the second title is
                          printed.
fuzz_script.txt           A messy configuration (blank lines, a tab delimited
                          entry, a one-token line, extra tokens) with a messy
                          part file, Messenger.txt (a non-numeric token, a
                          negative number, a number too large for usize, a
                          single-token line, extra and leading whitespace).
gaps_script.txt           A duplicated line 1 and missing lines 2 to 4, which
                          whinge reports as one range. Starting at line 0 is not
                          a gap.

--------------------------------------------------------------------------------
Unrecoverable failures
--------------------------------------------------------------------------------

Badly formed command line, error code 1: no arguments at all, too many
arguments, and a second argument that is not "whinge". Each prints the usage
message.

Script generation failure, error code 2. Each prints its own ERROR message and
recites nothing on stdout:

    no_such_script.txt          The script file does not exist.
    empty_script.txt            The script file is empty.
    blank_script.txt            The script file holds only blank lines.
    titles_only_script.txt      [scene] lines but no configuration files.
    untitled_script.txt         A configuration file before any [scene] title.
    missing_config_script.txt   A configuration file that does not exist.
    wrong_case_script.txt       "[Scene]" is not the directive, so it is read
                                as a configuration file name that does not
                                exist.
    empty_config_script.txt     An empty configuration file.
    junk_config_script.txt      A configuration file of only one-token and
                                blank lines. Whinge mode also warns about each
                                bad line.
    missing_part_script.txt     A part file that does not exist.
    dir_part_script.txt         A directory in place of a part file. Opening it
                                works but reading fails, so this tests the read
                                failure path.
    binary_part_script.txt      A part file that is not valid UTF-8.

--------------------------------------------------------------------------------
Randomized testing
--------------------------------------------------------------------------------

fuzz_test.py builds random script, configuration, and part files, runs the
program on them, and compares the result against an independent Python model
of what the spec requires. The model predicts the exact stdout, the exit code,
and the number of whinge warnings.

Script files mix [scene] lines, bare [scene] lines, blank lines, configuration
file names with and without extra tokens, and sometimes no title at all.
Configuration files mix good lines, one-token lines, lines with extra tokens,
blank lines, and sometimes do not exist. Part files are usually readable but
are sometimes empty, missing, a directory, or invalid UTF-8. Part file lines
include well formed lines and every malformed shape worth trying: blank and
whitespace-only lines, a lone token, a non-numeric first token, a negative
number, a decimal, a forty-digit number, exactly 2^64, 0 and usize::MAX, a
leading plus sign, tab and multi-space delimiters, CRLF endings, missing
trailing newlines, non-ASCII text, and a full-width digit.

Each case is checked against seven invariants:

    1. the program never panics or crashes
    2. it always terminates promptly, never hanging
    3. a successful run's stdout matches the model exactly
    4. a failed run prints "Error: 2" and an ERROR message, and recites nothing
    5. whinge output goes only to stderr, and stderr is empty on success
       without whinge
    6. with whinge, the number of WARNING lines matches the model
    7. two runs on identical input produce identical output

Before the random cases, the model is checked against the provided Hamlet
script and every hand-made script in test_inputs, so a bug in the model shows
up there first.

The run is seeded, so it reproduces exactly:

        cargo build
        python3 fuzz_test.py ./target/debug/lab2 2000 5402

The generator file created by Claude, fuzz_test.py, is included for reference,
but does not need to be run unless interested. run_tests.sh and the commands in
Section 2 show the program working.

--------------------------------------------------------------------------------
How to reproduce
--------------------------------------------------------------------------------

[meek@iht32-1501.sif lab2]$ module add rust-2023
[meek@iht32-1501.sif lab2]$ sh run_tests.sh
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
PASS  build has no warnings
PASS  unit tests pass

PASS  no arguments prints usage
PASS  too many arguments prints usage
PASS  unrecognized optional argument
PASS  Hamlet Scene I matches hamlet_act_ii_scene_1.txt
PASS  full Hamlet script matches expected output
PASS  Hamlet whinges about line 0 on stderr
PASS  Hamlet is silent on stderr by default
PASS  characters exit, re-enter, and titles start scenes
PASS  same cast twice in a row stays on stage
PASS  tied line numbers keep config and file order
PASS  tied line numbers whinge as duplicates
PASS  Macbeth: lines out of order are sorted
PASS  Macbeth: silent character enters
PASS  Macbeth: silent character never speaks
PASS  blank lines, tabs and extra spaces
PASS  CRLF script and configuration
PASS  carriage returns are trimmed
PASS  numeric edge cases are recited
PASS  numeric edge cases whinge
PASS  bare [scene] line is skipped
PASS  bare [scene] line whinges
PASS  extra tokens after a config file
PASS  extra tokens whinge
PASS  second [scene] title replaces the first
PASS  replaced [scene] title is never printed
PASS  fuzzed configuration still recites
PASS  fuzzed configuration whinges
PASS  duplicate line whinges
PASS  missing lines whinge as a range
PASS  starting at line 0 is not a gap
PASS  missing script file
PASS  empty script file
PASS  script of blank lines
PASS  script with no configuration files
PASS  configuration before any [scene] title
PASS  missing configuration file
PASS  [Scene] is not the [scene] directive
PASS  empty configuration file
PASS  configuration of only malformed lines
PASS  malformed-only configuration whinges
PASS  malformed-only configuration is quiet
PASS  missing part file
PASS  part file is a directory
PASS  part file is not UTF-8

46 passed, 0 failed
[meek@iht32-1501.sif lab2]$

test_output.txt contains captured output from gen_output.sh: the provided
Hamlet script, its whinge output on stderr alone, the recurring cast script,
the Macbeth script, the fuzzed script with and without whinge mode, several
whinge cases, one failure of each kind with its exit code, the full regression
suite, and a fuzzer run.

--------------------------------------------------------------------------------
Known limitations
--------------------------------------------------------------------------------

Because the expected line number saturates at usize::MAX rather than moving
past it, a second line also numbered usize::MAX is not reported as a duplicate
in whinge mode. Both lines are still recited, one after the other and in the
correct order. Only the complaint is missing. Catching it would mean tracking
the last line spoken separately (for example as an Option<usize>), which did
not seem worth the extra state for a case that only arises if two lines are
given the largest possible line number.

================================================================================
SECTION 4: AI USAGE AND CITATIONS
================================================================================

This section documents collaboration with Generative AI as required by the
CSE 5402 Generative AI Use Policy. Claude Code does not support public link
sharing of a conversation, so under the policy's Access Link or Appendix
requirement each interaction is exported in full and included with this
submission, in the ai_transcripts directory.

--------------------------------------------------------------------------------
Citations
--------------------------------------------------------------------------------

[1] Anthropic, October 19, 2026, "Lab 2 structs refactor," [Generative AI
    chat], Claude Opus 5.5 (Claude Code v2.1.302), exported transcript included
    as Appendix A (ai_transcripts/2026-10-19-201738-lab2-structs-refactor.txt).

[2] Anthropic, October 19, 2026, "Lab 2 return wrapper and scene fragment,"
    [Generative AI chat], Claude Opus 5.5 (Claude Code v2.1.302), exported
    transcript included as Appendix B
    (ai_transcripts/2026-10-19-232912-lab2-scene-fragment.txt).

[3] Anthropic, October 20, 2026, "Lab 2 finishing the spec," [Generative AI
    chat], Claude Opus 5.5 (Claude Code v2.1.302), exported transcript included
    as Appendix C (ai_transcripts/2026-10-20-113647-lab2-finish-spec.txt).

[4] Anthropic, October 20, 2026, "Lab 2 comments and cleanup," [Generative
    AI chat], Claude Opus 5.5 (Claude Code v2.1.302), exported transcript
    included as Appendix D
    (ai_transcripts/2026-10-20-145251-lab2-resolve-comment-todos.txt).

[5] Anthropic, October 20, 2026, "Lab 2 ReadMe writeup," [Generative AI
    chat], Claude Opus 5.5 (Claude Code v2.1.302), exported transcript
    included as Appendix E
    (ai_transcripts/2026-10-20-212956-lab2-work-on-writeup.txt).

Code carried over from Lab 1 that Claude helped with (the empty argument
vector guard in parse_args and the first version of fuzz_test.py and
gen_output.sh) is cited in the Lab 1 ReadMe, with its transcripts.

--------------------------------------------------------------------------------
Division of work
--------------------------------------------------------------------------------

I wrote the program: the Player, Play, SceneFragment, and ReturnWrapper structs
and their methods, the trait implementations, the module layout, and the
design decisions recorded in Section 1.

Work Claude contributed, all of it reviewed and accepted by me:

-   Changing the whinge println! calls to eprintln! [1].
-   The one-range-per-gap missing line warning in recite, after testing found
    the usize::MAX hang [1].
-   The empty trait impl scaffold for PartialEq, Eq, PartialOrd, and Ord in
    player.rs, which I then filled in [2].
-   The warning messages in Play::add_config [3].
-   The rewritten run_tests.sh, fuzz_test.py, and gen_output.sh for Lab 2, and
    the hand-made input files in test_inputs [1], [2], [3], [4].
-   The unit tests in player.rs and scene_fragment.rs [2], [4].
-   The header comments and function summary comments in each file,
    based on my initial header and function comment style [4].
-   Drafting and editing passes on this ReadMe [5].

--------------------------------------------------------------------------------
Key prompts and how the interactions fit the workflow
--------------------------------------------------------------------------------

 -  All of the core implementation for this lab assignment was written by me.
    I used Claude to guide me on what to implement next and to answer
    syntax or behavior questions I had along the way, such as how
    marker traits work and if it's possible to implement your own ?
    operator.
 -  The primary use of Claude was for testing my program and catching any
    edge cases or bugs. Claude is particularly great at scanning the details
    of an assignment (i.e., the Lab 2 instructions), finding inconsistencies,
    developing test cases based on the requirements, discovering strange
    edge cases, etc. I found AI to be extremely useful for this, and the
    vast majority of my key prompts centered around this philosophy.
 -  I had more questions about syntax in Lab 2 than I did for Lab 1.
    For example, I forgot exactly how to make struct fields visible to
    just the current crate. Furthermore, there were more complex design
    patterns in this lab like trait implementations and ownership concerns
    that Claude was great at guiding me through.
 -  In that same vein, I engineered my prompts such that Claude was
    able to enhance my understanding of Rust without just giving me
    the answer. For example, I wasn't sure about the most idiomatic
    way to look at the first element of a sequence, and Claude introduced
    me to peekable iterators. Another great example is the slice pattern
    matching in play.rs. I knew I wanted to match on different line
    structures based on the number of words provided, the specific words
    in the stream, etc. I learned that the best approach is to collect
    the line into a Vec of word slices and pattern match on that.
 -  Lastly, Claude was very helpful in creating the write-up: text
    formatting, generating a ReadMe.txt template, filling in the obvious parts,
    leaving blank sections for my own design decisions that I needed to write,
    fixing grammar, making sure I completed all required sections of the
    write-up, etc. This made the assignment far more enjoyable as I was
    able to focus on the code and have Claude help with documentation and
    testing.
