TODO: update this ReadMe for Lab 2 (title, overview, build/run steps, testing, AI citations) and add the Structs, Return Wrapper, Scene Fragments, and Testing subsections the spec asks for.

================================================================================
SECTION 1: IDENTIFICATION AND DESIGN
================================================================================

CSE 5402 Fall 2025 Lab 1: "The play's the thing"

Team members:
* Preston Meek: meek@wustl.edu

--------------------------------------------------------------------------------
Summary
--------------------------------------------------------------------------------

When approaching this lab, I focused on having a modular design such that
individual functions and components were isolated. This decoupling allowed me to
seamlessly follow each numbered step in the lab overview and implement functions
one-by-one. Once all of the individual parsing and logic functions were written,
putting them together in the main entry point of the program was very
straightforward. This approach led to very little hurdles, even with minimal
(practically zero, besides a few cleanups which were outlined explicitly) use of
AI for writing code.

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
    3. main sorts the Play. Because the tuple's first element is the usize line
       number, the derived tuple ordering sorts the whole play by line number
       without any custom comparison function; ties (which should never happens
       in the well-formed case) fall back to the character
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

    src/main.rs                 Command line handling and output. Declares the
                                lab1 module and defines usage, parse_args,
                                recite, and main -- that is, everything that
                                touches the user rather than the input files.
                                The WHINGE_ARG and FALLBACK_PROGRAM_NAME
                                constants are private to this file, since only
                                parse_args and usage ever need them.

    src/lab1/mod.rs             Declares the two modules below.

    src/lab1/declarations.rs    The program-wide vocabulary, all public: the
                                Play type, the command line argument count and
                                index constants, the two u8 error codes, and the
                                WHINGE_MODE static AtomicBool.

    src/lab1/script_gen.rs      Everything involved in turning files into a
                                Play: add_script_line, grab_trimmed_file_lines,
                                process_config, add_config, read_config, and the
                                public script_gen entry point. The PlayConfig
                                type and the index constants live here and are
                                private, because nothing outside this file uses
                                them.

The split follows the rule the assignment gives: items used in only one file are
declared in that file, and only items shared across files are made public. Of
the two types, Play crosses the module boundary (main declares one and passes it
to script_gen) so it is public in declarations.rs, while PlayConfig never leaves
script_gen.rs and so stays private there. The same rule keeps WHINGE_ARG and
FALLBACK_PROGRAM_NAME private in main.rs and the configuration line index
constants private in script_gen.rs.

Two items in declarations.rs are there by instruction rather than by that rule.
The assignment names declarations.rs as the place for the command line count
and index constants and for the two u8 error codes, so that is where they live,
but in practice each is used by only one file: the argument constants and
ERR_BADLY_FORMED_COMMAND_LINE only by main.rs, and
ERR_FAILED_TO_GENERATE_SCRIPT only by script_gen.rs. Of everything in
declarations.rs, only Play and WHINGE_MODE genuinely have readers in both
files. I followed the assignment here rather than relocating them, since
grouping the program-wide vocabulary in one file is also what makes the error
codes easy to compare against each other.

In terms of the module structure, the biggest challenge was getting used to the
different ways to declare namespaces, i.e., super vs. crate vs lab1. The same
module can be named three ways depending on where the naming happens:
main.rs is the crate root and so reaches the module it declares as lab1::...,
while script_gen.rs sits inside lab1 and can say either super::declarations
(relative, one level up to its parent) or crate::lab1::declarations (absolute,
down from the root). super is not available in main.rs at all, since the crate
root has no parent; trying it is a compile error.

That ambiguity showed up concretely in script_gen.rs, which ended up importing
from declarations.rs twice with two different spellings: an original
use super::declarations::{Play, WHINGE_MODE} written by hand, and a
use crate::lab1::declarations::ERR_FAILED_TO_GENERATE_SCRIPT added later when
the editor auto-imported the error code for me. Both resolve to the same
module, so the compiler was perfectly happy and nothing flagged it. I caught it
during review and merged the two into a single use super::declarations::{...},
choosing the relative form because it matches the example in the assignment and
keeps the file from depending on where lab1 is mounted in the crate.

--------------------------------------------------------------------------------
Design decisions
--------------------------------------------------------------------------------

1.  An unopenable part file is a badly formed configuration line, not a fatal
    error. The assignment says the program "should test whether or not the file
    exists and can be opened correctly for reading - if not the program should
    consider the character definition line to be badly formed, and continue to
    the next line", but it also says process_config should return an error if
    grab_trimmed_file_lines fails. I resolved the tension by having
    process_config swallow the per-character failure, skip that character, and
    count the characters whose part files did open. Only if that count is zero
    does it return ERR_FAILED_TO_GENERATE_SCRIPT, which matches the separate
    requirement that a configuration without "at least one valid character
    definition (including the proper opening of that character's input file)"
    is an error. grab_trimmed_file_lines itself is unchanged from its specified
    behavior: it still returns the script generation error code.

2.  Blank configuration lines are ignored silently rather than warned about.
    The configuration file paragraph says to read lines "ignoring any lines that
    only contain whitespace", while the introduction lists a line "with only
    blank space" among badly formed lines that may draw a warning. I chose to
    ignore them: a trailing newline at the end of a file is normal, not a defect
    worth complaining about, and warning on it would produce noise on well
    formed input. Because grab_trimmed_file_lines has already trimmed every
    line, whitespace-only lines are exactly the empty ones, so read_config drops
    them with a single retain call before it takes the title from the first
    remaining line. This also keeps the "at least two lines" check counting
    real content.

3.  Severity is decided by the caller, not by grab_trimmed_file_lines. That
    function has two callers with opposite needs: a configuration file that will
    not open is fatal and must always be reported, while a part file that will
    not open is handled by skipping the character and should therefore only be
    reported under whinge mode. Since the required signature leaves no room for
    a severity parameter, grab_trimmed_file_lines emits its open and read
    failures as whinge-gated warnings that carry the underlying io::Error text,
    and read_config prints its own unconditional ERROR message when the call
    fails. The trade-off is that the always-printed message is less specific
    than the whinge-gated one.

4.  Messages are split into two families so no two failures look alike:
    "ERROR: " for unrecoverable problems, which always print, and "WARNING: "
    for problems the program handled, which print only when whinge mode is on.
    Each message names the file, character, or token it concerns, so the three
    distinct causes behind error code 2 -- an unreadable configuration file, a
    configuration with fewer than two useful lines, and a configuration with no
    readable part files -- each explain themselves.

5.  Two primary diversions from the assignment's instructions. Instead of using
    String references &str slices were used since this is much more idiomatic
    Rust and does not force heap-allocated String structs to be used.
    Additionally, instead of a match statement with a single branch,
    destructuring for loops were used instead since they are more syntactically
    concise. In particular, the for-loop case was a warning from Clippy.

--------------------------------------------------------------------------------
Insights, observations, and questions
--------------------------------------------------------------------------------

*   Through testing and securing this program from all possible edge cases, I
    learned that the 0th argument in argv, the program name, is not required! It
    is always passed in by [well-behaving] shells, but a bare execve syscall
    does not require any arguments at all. Therefore, a default program name was
    added for this [albeit very obscure] edge case.
*   Why was a single-arm match statement preferred over a destructuring for-loop
    in both studios and this lab? The compiler warns in these cases and says the
    syntax can be reduced, and I personally agree that a destructuring for loop
    looks much cleaner. From what I understand, they should compile down to the
    same assembly regardless.
*   The error-handling syntax of Rust is quite enjoyable to work with,
    especially the ? operator. While I think exceptions have their use cases, I
    generally prefer to work without them, and Rust obviously agrees with this
    :). On top of that, being forced to handle every enumeration case with
    compile-time enforcement makes me feel much better about my code actually
    working at runtime.
*   Why were String references preferred over &str? From what I understand, &str
    is strictly better in these cases as it takes in a generic character slice
    that does not require string heap allocation.
*   Sorting a Vec of tuples needed no comparison function at all as the derived
    lexicographic ordering on (usize, String, String) is exactly the ordering
    the assignment asks for.

================================================================================
SECTION 2: BUILDING AND RUNNING ON THE CEC LINUX LAB MACHINES
================================================================================

1.  Copy the submitted archive to a CEC Linux Lab machine and unpack it:

        unzip lab1.zip
        cd lab1

    The unpacked directory contains Cargo.toml, Cargo.lock, the src directory,
    the configuration and part files used for testing, run_tests.sh,
    gen_output.sh, fuzz_test.py, test_output.txt, the ai_transcripts directory
    holding the exported Generative AI conversations cited in Section 4, and
    this ReadMe.txt.

2.  Make the course Rust toolchain available:

        module add rust-2023

    This provides rustc and cargo version 1.71.0. Confirm with:

        rustc --version

3.  Build the program:

        cargo build

    The build completes with no errors and no warnings. The executable is
    written to target/debug/lab1.

4.  Run the program:

        cargo run -q <configuration_file_name> [whinge]

    for example:

        cargo run -q hamlet_ii_2_config.txt
        cargo run -q fuzz_config.txt whinge

    The optional third argument must be exactly the word "whinge"; it turns on
    warning messages about problems the program handled successfully. Running
    the program with no configuration file name prints a usage message and
    returns error code 1.

    The program can equally be run directly once built:

        ./target/debug/lab1 hamlet_ii_2_config.txt

5.  To run the full regression suite described in Section 3:

        ./run_tests.sh

    It builds the program, runs all 16 test cases, prints PASS or FAIL for
    each, and exits non-zero if any case fails.

Note on the package configuration: Cargo.toml specifies edition = "2021" and
Cargo.lock is written in lockfile format version 3, both of which the 1.71.0
tools on the lab machines accept.

================================================================================
SECTION 3: TESTING
================================================================================

Testing was done by running the program over a set of configuration files and
part files written to cover both well formed input and every category of badly
formed input the assignment describes, and by checking the exit status, the
error code, and the printed script in each case. run_tests.sh automates the
whole set; test_output.txt holds captured output from a representative run.

--------------------------------------------------------------------------------
File formats used
--------------------------------------------------------------------------------

A configuration file has the play's title on its first non-blank line, and after
that one line per character holding two whitespace delimited tokens: the
character's name and the name of the file containing that character's lines.

A part file has one line per spoken line: a usize line number, whitespace, and
then the text of the line. Line numbers are unique across the whole play, so
the sorted result interleaves the characters' parts into a single script.

--------------------------------------------------------------------------------
Well formed input
--------------------------------------------------------------------------------

hamlet_ii_2_config.txt
    The reference fragment supplied with the assignment, with part files
    Guildenstern.txt, King.txt, Queen.txt, and Rosencrantz.txt. Its output was
    compared line by line against hamlet_act_ii_scene_2.txt to confirm both the
    ordering and the alternation of speakers.

macbeth_ii_1_config.txt
    A second play I wrote myself, with part files Banquo.txt, Fleance.txt,
    Macbeth.txt, and Servant.txt. Two properties are tested here that the
    Hamlet fragment does not exercise:

      - Banquo.txt lists its lines out of numeric order, confirming that the
        script is assembled by line number rather than by the order lines appear
        in a part file.
      - Servant is a silent character whose part file is empty. The assignment
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
    that does not exist, and interior and trailing blank lines. Messenger.txt
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
and three arguments whose last is not "whinge". Each prints the usage message.

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
Randomized testing
--------------------------------------------------------------------------------

Beyond the fixed cases above, the program was exercised by a generator that
builds random configuration and part files and checks what the program prints
against an independent model of what the assignment requires. Each generated
case picks zero to five characters and gives each of them one of: a well formed
part file, a part file named but absent, a directory in place of a file, a file
of invalid UTF-8, an empty file, a configuration line with extra tokens, a
one-token line, or a blank line. Part file lines are drawn from well formed
numbered lines and from every malformed shape worth trying: blank and
whitespace-only lines, a lone token, a non-numeric first token, a negative
number, a decimal, a forty-digit number, exactly 2^64, 0 and usize::MAX, tab
and multi-space delimiters, CRLF endings, missing trailing newlines,
300-character lines, and non-ASCII text in both character names and spoken
lines.

Each case is checked against nine invariants:

    1. the program never panics
    2. it always terminates promptly, never hanging
    3. it reports error code 2 exactly when the configuration yields no
       usable character, and succeeds otherwise
    4. on success it prints the title followed by well formed speaker blocks
    5. recited lines appear in nondecreasing line-number order
    6. a speaker is announced exactly when the speaker changes, and is never
       re-announced while still speaking
    7. every well formed line from a readable part file is recited verbatim
    8. no WARNING appears unless whinge mode is on
    9. two runs on identical input produce identical output

Twelve thousand cases across six seeds pass. One defect was found and fixed:
the empty argument vector described in the insights above, where indexing
position 0 of the argument vector panicked instead of printing the usage
message. Every category of malformed file content the generator produced was
already handled correctly.

The generator varies file content rather than command lines, and always passes
a valid Unicode configuration file name, so it does not reach the argument
handling itself; the argv cases were tested by hand instead.

The generator file created by Claude, fuzz_test.py, is included for reference,
but does not need to be run unless interested. There are simpler shell scripts
included in this repository that show the program working.

The campaign is seeded, so it reproduces exactly:

        cargo build
        for s in 1 7 99 2718 31415 8675309; do
            python3 fuzz_test.py ./target/debug/lab1 2000 $s
        done

Each seed runs 2000 cases and prints how many passed.

The miscellaneous .txt files in this repository are from trying things manually
and prompting Claude to create files that catch specific edge cases. It is
also nice to have these text files to verify the program works with manual
cargo commands.

--------------------------------------------------------------------------------
How to reproduce
--------------------------------------------------------------------------------

[meek@ssh-shell-1 lab1]$ ./run_tests.sh
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s

PASS  no arguments prints usage
PASS  too many arguments prints usage
PASS  unrecognized optional argument
PASS  reference Hamlet fragment
PASS  Macbeth fragment, lines out of order
PASS  silent character is never announced
PASS  blank lines and extra whitespace
PASS  fuzzed configuration and part files
PASS  fuzzed input warns under whinge
PASS  fuzzed input stays quiet without whinge
PASS  badly formed line numbers are skipped
PASS  carriage returns are trimmed
PASS  unreadable part file is skipped
PASS  missing configuration file
PASS  configuration with no character lines
PASS  configuration with no readable parts

16 passed, 0 failed

test_output.txt contains captured output from a run of the reference fragment,
the Macbeth fragment, the fuzzed configuration with and without whinge mode, the
numeric edge cases, two failure cases, and the full suite.

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

[1] Anthropic, September 5, 2026, "Lab 1 design review and ReadMe drafting,"
    [Generative AI chat], Claude Opus 5 (Claude Code v2.1.245), exported
    transcript included as Appendix A
    (ai_transcripts/2026-09-05-lab1-design-and-readme.txt).

[2] Anthropic, September 8, 2026, "Lab 1 code review," [Generative AI chat],
    Claude Opus 5 (Claude Code v2.1.245), exported transcript included as
    Appendix B (ai_transcripts/2026-09-08-lab1-review.txt).

[3] Anthropic, September 20, 2026, "Lab 1 fuzz testing, &str conversion, and
    ReadMe completion," [Generative AI chat], Claude Opus 5
    (Claude Code v2.1.245), exported transcript included as Appendix C
    (ai_transcripts/2026-09-20-lab1-fuzzing-and-readme.txt).

--------------------------------------------------------------------------------
Division of work
--------------------------------------------------------------------------------

I wrote the program: the module structure, all ten functions, the type and
constant declarations, the two error codes and the whinge-gated warning scheme,
and the configuration and part files used for testing, along with run_tests.sh
and its sixteen cases. The design decisions recorded in Section 1 are mine.

Work Claude contributed, all of it reviewed and accepted by me:

*   The empty argument vector guard in parse_args -- the FALLBACK_PROGRAM_NAME
    constant and the args.get(...).unwrap_or(...) that replaced indexing
    position 0 -- after randomized testing found that the original code
    panicked in that case [3].
*   The mechanical &String-to-&str conversion across the function signatures.
    The decision to diverge from the assignment's stated signatures was mine;
    Claude applied it and verified it [3].
*   fuzz_test.py, the randomized test generator described in Section 3, and
    the twelve thousand case campaign it ran [3].
*   gen_output.sh, which regenerates test_output.txt [3].
*   Drafting and editing passes on this ReadMe, including the Modules
    subsection and the Randomized testing subsection [1], [3].

--------------------------------------------------------------------------------
Key prompts and how the interactions fit the workflow
--------------------------------------------------------------------------------

 -  All of the initial code for the program was written by me. Before I used
    AI at all, I had a ~95% complete Lab 1 program.
 -  The primary use of Claude was for testing my program and catching any
    edge cases or bugs. Claude is particularly great at scanning the details
    of an assignment (i.e., the Lab 1 instructions), finding inconsistencies,
    developing test cases based on the requirements, discovering strange
    edge cases, etc. I found AI to be extremely useful for this, and the
    vast majority of my key prompts centered around this philosophy.
 -  Claude is also great at determining what is 'idiomatic Rust'. Since it has a
    ton of training data (textbooks, professional open-source code bases), etc.
    it is quickly able to deduce if a pattern is well-written or not. This was
    extremely helpful since I am still learning the best approaches for writing
    idiomatic Rust programs coming from a primarily C++ background.
 -  Lastly, Claude was *extremely* helpful in creating the write-up: text
    formatting, generating a ReadMe.txt template, filling in the obvious parts,
    leaving blank sections for my own design decisions that I needed to write,
    fixing grammar, making sure I completed all required sections of the
    write-up, etc. This made the Lab 1 assignment far more enjoyable as I was
    able to focus on the code and have Claude help with documentation and
    testing.
