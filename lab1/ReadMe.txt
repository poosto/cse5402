================================================================================
SECTION 1: IDENTIFICATION AND DESIGN
================================================================================

CSE 5402 Fall 2025 Lab 1: "The play's the thing"

Team members:
    [YOUR FULL NAME] <[YOUR EMAIL]>
    [TEAMMATE NAME]  <[TEAMMATE EMAIL]>          <-- delete if working alone

--------------------------------------------------------------------------------
Summary
--------------------------------------------------------------------------------

[OVERVIEW OF THE SOLUTION APPROACH -- a draft is below; edit it into your own
words and delete anything that no longer matches the code you submit.]

The program reads a configuration file naming a play and the characters in it,
reads each character's numbered lines from that character's own part file,
merges every line into a single data structure, and then prints the assembled
script in line-number order.

Control flows through four stages:

    1. parse_args validates the command line, stores the configuration file
       name, and sets the WHINGE_MODE static atomic Boolean if the optional
       "whinge" argument was supplied.
    2. script_gen calls read_config to obtain the play's title and a PlayConfig
       vector of (character name, part file name) pairs, then calls
       process_config to read each part file and turn its lines into Play
       tuples.
    3. main sorts the Play.  Because the tuple's first element is the usize line
       number, the derived tuple ordering sorts the whole play by line number
       without any custom comparison function; ties fall back to the character
       name, which keeps the output deterministic.
    4. recite walks the sorted Play, printing a blank line and the speaker's
       name (with a trailing period) whenever the speaker changes, and then the
       text of each line.

Two error codes distinguish the two unrecoverable failures the assignment names:
ERR_BADLY_FORMED_COMMAND_LINE (1) and ERR_FAILED_TO_GENERATE_SCRIPT (2).
Everything else is recoverable: a badly formed line is skipped and the program
continues looking for well formed ones.

--------------------------------------------------------------------------------
Modules
--------------------------------------------------------------------------------

    src/main.rs                 Command line handling and output.  Declares the
                                lab1 module and defines usage, parse_args,
                                recite, and main -- that is, everything that
                                touches the user rather than the input files.

    src/lab1/mod.rs             Declares the two modules below.

    src/lab1/declarations.rs    Items used by more than one file, and therefore
                                public: the Play type, the command line argument
                                count and index constants, the two u8 error
                                codes, and the WHINGE_MODE static AtomicBool.

    src/lab1/script_gen.rs      Everything involved in turning files into a Play:
                                add_script_line, grab_trimmed_file_lines,
                                process_config, add_config, read_config, and the
                                public script_gen entry point.  The PlayConfig
                                type and the index constants live here and are
                                private, because nothing outside this file uses
                                them.

The split follows the rule the assignment gives: items used in only one file are
declared in that file, and only items shared across files are made public.  Of
the two types, Play crosses the module boundary (main declares one and passes it
to script_gen) so it is public in declarations.rs, while PlayConfig never leaves
script_gen.rs and so stays private there.

[DESIGN CHALLENGES FROM THE MODULE STRUCTURE -- describe any you hit, e.g. the
namespace nesting that requires `use super::declarations::...` inside
src/lab1/script_gen.rs versus `use lab1::declarations::...` in src/main.rs, and
how you resolved them.]

--------------------------------------------------------------------------------
Design decisions
--------------------------------------------------------------------------------

[EDIT AND KEEP THE ONES THAT MATCH YOUR FINAL CODE.  These are the judgment
calls where the assignment left room, so they are the ones worth explaining.]

1.  An unopenable part file is a badly formed configuration line, not a fatal
    error.  The assignment says the program "should test whether or not the file
    exists and can be opened correctly for reading - if not the program should
    consider the character definition line to be badly formed, and continue to
    the next line", but it also says process_config should return an error if
    grab_trimmed_file_lines fails.  We resolved the tension by having
    process_config swallow the per-character failure, skip that character, and
    count the characters whose part files did open.  Only if that count is zero
    does it return ERR_FAILED_TO_GENERATE_SCRIPT, which matches the separate
    requirement that a configuration without "at least one valid character
    definition (including the proper opening of that character's input file)"
    is an error.  grab_trimmed_file_lines itself is unchanged from its specified
    behavior: it still returns the script generation error code.

2.  Blank configuration lines are ignored silently rather than warned about.
    The configuration file paragraph says to read lines "ignoring any lines that
    only contain whitespace", while the introduction lists a line "with only
    blank space" among badly formed lines that may draw a warning.  We chose to
    ignore them: a trailing newline at the end of a file is normal, not a defect
    worth complaining about, and warning on it would produce noise on well
    formed input.  Because grab_trimmed_file_lines has already trimmed every
    line, whitespace-only lines are exactly the empty ones, so read_config drops
    them with a single retain call before it takes the title from the first
    remaining line.  This also keeps the "at least two lines" check counting
    real content.

3.  Severity is decided by the caller, not by grab_trimmed_file_lines.  That
    function has two callers with opposite needs: a configuration file that will
    not open is fatal and must always be reported, while a part file that will
    not open is handled by skipping the character and should therefore only be
    reported under whinge mode.  Since the required signature leaves no room for
    a severity parameter, grab_trimmed_file_lines emits its open and read
    failures as whinge-gated warnings that carry the underlying io::Error text,
    and read_config prints its own unconditional ERROR message when the call
    fails.  The trade-off is that the always-printed message is less specific
    than the whinge-gated one.

4.  Messages are split into two families so no two failures look alike:
    "ERROR: " for unrecoverable problems, which always print, and "WARNING: "
    for problems the program handled, which print only when whinge mode is on.
    Each message names the file, character, or token it concerns, so the three
    distinct causes behind error code 2 -- an unreadable configuration file, a
    configuration with fewer than two useful lines, and a configuration with no
    readable part files -- each explain themselves.

5.  Function signatures follow the assignment even where they are not the most
    idiomatic Rust.  usage, for instance, takes a &String rather than the &str
    that Clippy would prefer, because the assignment specifies "a shared
    (immutable) reference to a String variable".  We kept the specified
    signatures throughout.

[6. IF YOU APPLIED THE match DESTRUCTURING IN recite AND process_config, SAY SO
    HERE; IF YOU DELIBERATELY USED for-LOOP PATTERN BINDING INSTEAD, EXPLAIN
    THAT CHOICE.  As of this draft that item is still open.]

--------------------------------------------------------------------------------
Insights, observations, and questions
--------------------------------------------------------------------------------

[YOUR OWN.  Some candidates from building this, keep what is true for you:]

*   Sorting a Vec of tuples needed no comparison function at all -- the derived
    lexicographic ordering on (usize, String, String) is exactly the ordering
    the assignment asks for.
*   split_once(char::is_whitespace) splits at the first whitespace character
    only, so a part file that separates the line number from the text with two
    spaces leaves the extra space at the front of the text.  Calling trim on the
    remainder, as the assignment directs, is what removes it; without it the
    lines from King.txt print with a leading space.
*   parse::<usize> accepts more than expected ("007" and "+5" both parse) and
    less than expected ("1_0" does not), which is worth knowing before writing
    test input.
*   [A QUESTION YOU HAVE FOR THE INSTRUCTOR, e.g. whether a single-arm match is
    preferred over pattern binding in a for loop.]

================================================================================
SECTION 2: BUILDING AND RUNNING ON THE CEC LINUX LAB MACHINES
================================================================================

1.  Copy the submitted archive to a CEC Linux Lab machine and unpack it:

        unzip lab1.zip
        cd lab1

    [ADJUST THE ARCHIVE NAME AND, IF YOUR ARCHIVE UNPACKS INTO A DIFFERENT
    DIRECTORY NAME, THE cd COMMAND.  If you submit a tarball instead, the
    command is: tar xzf lab1.tar.gz]

    The unpacked directory contains Cargo.toml, Cargo.lock, the src directory,
    the configuration and part files used for testing, run_tests.sh, and this
    ReadMe.txt.

2.  Make the course Rust toolchain available:

        module add rust-2023

    This provides rustc and cargo version 1.71.0.  Confirm with:

        rustc --version

3.  Build the program:

        cargo build

    The build completes with no errors and no warnings.  The executable is
    written to target/debug/lab1.

4.  Run the program:

        cargo run -q <configuration_file_name> [whinge]

    for example:

        cargo run -q hamlet_ii_2_config.txt
        cargo run -q fuzz_config.txt whinge

    The optional third argument must be exactly the word "whinge"; it turns on
    warning messages about problems the program handled successfully.  Running
    the program with no configuration file name prints a usage message and
    returns error code 1.

    The program can equally be run directly once built:

        ./target/debug/lab1 hamlet_ii_2_config.txt

5.  To run the full regression suite described in Section 3:

        sh run_tests.sh

    It builds the program, runs all [NUMBER] test cases, prints PASS or FAIL for
    each, and exits non-zero if any case fails.

Note on the package configuration: Cargo.toml specifies edition = "2021" and
Cargo.lock is written in lockfile format version 3, both of which the 1.71.0
tools on the lab machines accept.  [KEEP THIS NOTE ONLY IF IT IS STILL TRUE OF
WHAT YOU SUBMIT.]

================================================================================
SECTION 3: TESTING
================================================================================

Testing was done by running the program over a set of configuration files and
part files written to cover both well formed input and every category of badly
formed input the assignment describes, and by checking the exit status, the
error code, and the printed script in each case.  run_tests.sh automates the
whole set; test_output.txt holds captured output from a representative run.

--------------------------------------------------------------------------------
File formats used
--------------------------------------------------------------------------------

A configuration file has the play's title on its first non-blank line, and after
that one line per character holding two whitespace delimited tokens: the
character's name and the name of the file containing that character's lines.

A part file has one line per spoken line: a usize line number, whitespace, and
then the text of the line.  Line numbers are unique across the whole play, so
the sorted result interleaves the characters' parts into a single script.

--------------------------------------------------------------------------------
Well formed input
--------------------------------------------------------------------------------

hamlet_ii_2_config.txt
    The reference fragment supplied with the assignment, with part files
    Guildenstern.txt, King.txt, Queen.txt, and Rosencrantz.txt.  Its output was
    compared line by line against hamlet_act_ii_scene_2.txt to confirm both the
    ordering and the alternation of speakers.

macbeth_ii_1_config.txt
    A second play we wrote ourselves, with part files Banquo.txt, Fleance.txt,
    Macbeth.txt, and Servant.txt.  Two properties are tested here that the
    Hamlet fragment does not exercise:

      - Banquo.txt lists its lines out of numeric order, confirming that the
        script is assembled by line number rather than by the order lines appear
        in a part file.
      - Servant is a silent character whose part file is empty.  The assignment
        calls for handling a character with no well formed numbered lines; the
        expected behavior is that the play recites normally and the Servant is
        never announced, which is what happens.

--------------------------------------------------------------------------------
Badly formed input that the program recovers from
--------------------------------------------------------------------------------

Each of these exits with status 0 and prints a complete script; under whinge
mode each also prints a warning naming what was skipped, and prints nothing
extra when whinge mode is off.

fuzz_config.txt (with Messenger.txt)
    A deliberately messy configuration exercising the recoverable configuration
    cases at once: a leading blank line, whitespace-only lines including one
    containing spaces and a tab, a tab delimited entry with trailing spaces, an
    entry with only one token, an entry with extra tokens after the required two
    (the extras are ignored and the first two used), an entry naming a part file
    that does not exist, and interior and trailing blank lines.  Messenger.txt
    adds the part file cases: a non-numeric first token, a negative number, a
    number too large for usize, a single-token line, blank and whitespace-only
    lines, extra whitespace after the line number, leading whitespace before it,
    and a line number duplicating one of King's to check that ties order
    deterministically.

whitespace_config.txt
    A well formed configuration padded with runs of blank lines before, between,
    and after its entries, confirming that blank lines are ignored, that the
    title still comes from the first non-blank line, and that no spurious
    warnings appear even under whinge mode.

numeric_config.txt (with Ghost.txt)
    Concentrates on what parse::<usize> accepts and rejects: "007" and "+5"
    parse and sort by their numeric value; the largest 64-bit usize value parses
    and sorts last; "1_0", "3.5", and "0x10" are rejected and warned about; a
    line number surrounded by whitespace with no text after it becomes a
    single-token line and is skipped; and interior tabs within a line's text are
    preserved as written.

crlf_config.txt (with Sentinel.txt)
    A configuration file and a part file written with Windows CRLF line endings,
    confirming that trimming removes the carriage returns and that no stray
    characters reach the output.

dir_part_config.txt
    Names a directory (src) rather than a file as a character's part file.
    File::open succeeds on a directory but the first read fails, so this
    exercises the read failure path rather than the open failure path; the
    character is skipped and the remaining character still recites.

--------------------------------------------------------------------------------
Unrecoverable failures
--------------------------------------------------------------------------------

Badly formed command line, error code 1: no arguments at all; four arguments;
and three arguments whose last is not "whinge".  Each prints the usage message.

Script generation failure, error code 2:

    no_such_config.txt          A configuration file name that does not exist.
    title_only_config.txt       A configuration holding a title and blank lines
                                but no character definition at all.
    no_valid_parts_config.txt   A configuration whose every character names a
                                part file that cannot be opened, plus a
                                one-token line, so that no valid character
                                definition remains.

Each prints an ERROR message identifying which of the three causes occurred,
rather than reusing one message for all of them.

--------------------------------------------------------------------------------
How to reproduce
--------------------------------------------------------------------------------

        sh run_tests.sh

[RUN IT ON THE LAB MACHINES AND RECORD THE RESULT HERE, e.g. "All [NUMBER]
cases pass under Rust 1.71.0 on the CEC Linux Lab machines."]

test_output.txt contains captured output from a run of the reference fragment,
the Macbeth fragment, the fuzzed configuration with and without whinge mode, the
numeric edge cases, two failure cases, and the full suite.
