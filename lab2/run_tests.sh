#!/bin/sh
#
# run_tests.sh - regression tests for CSE 5402 Lab 2.
#
# Builds the program (failing if the build prints any warning), runs the unit
# tests, then runs the program over the provided Hamlet script and the hand-made
# inputs in test_inputs/.  Each case checks the exit status, the "Error: N" line
# the return wrapper prints, and what appears on standard output and standard
# error separately, so whinge output must reach stderr and never stdout.  Some
# cases also compare standard output against an expected output file.  Prints
# PASS or FAIL for each case and exits non-zero if any case failed.
#
# Usage: sh run_tests.sh      (from the lab2 package directory)

ROOT=$(pwd)
BIN=$ROOT/target/debug/lab2
SAMPLE=$ROOT/hamlet_act_ii_scene_1.txt
PASSES=0
FAILURES=0

# directory the program runs in, since scripts name files relative to it
DIR=$ROOT

TMP=$(mktemp -d) || exit 1
trap 'rm -rf "$TMP"' EXIT
CR=$(printf '\r')

# record <description> <failure reason, or empty for a pass>
record() {
    if [ -z "$2" ]; then
        PASSES=$((PASSES + 1))
        printf 'PASS  %s\n' "$1"
    else
        FAILURES=$((FAILURES + 1))
        printf 'FAIL  %s (%s)\n' "$1" "$2"
    fi
}

# run <arguments...> - run the program in $DIR, capturing stdout, stderr and status
run() {
    (cd "$DIR" && "$BIN" "$@") > "$TMP/out" 2> "$TMP/err"
    STATUS=$?
}

# matches <file> <pattern> - succeeds if the pattern is satisfied by the file:
# "-" always matches, "!pattern" requires the pattern to be absent, and any
# other pattern must appear on at least one line
matches() {
    case $2 in
        -) return 0 ;;
        !*) ! grep -q -- "${2#!}" "$1" ;;
        *) grep -q -- "$2" "$1" ;;
    esac
}

# check <description> <expected status> <stdout pattern> <stderr pattern> <arguments...>
#
# A non-zero status must be reported by the return wrapper as "Error: N" on
# stderr, and a zero status must not report any error.
check() {
    description=$1
    expected_status=$2
    out_pattern=$3
    err_pattern=$4
    shift 4

    run "$@"
    reason=""

    if [ "$STATUS" -ne "$expected_status" ]; then
        reason="expected exit status $expected_status but saw $STATUS"
    elif [ "$STATUS" -ne 0 ] && ! grep -q "^Error: $STATUS\$" "$TMP/err"; then
        reason="expected \"Error: $STATUS\" on stderr"
    elif [ "$STATUS" -eq 0 ] && grep -q "^Error: " "$TMP/err"; then
        reason="reported an error but was expected to succeed"
    elif ! matches "$TMP/out" "$out_pattern"; then
        reason="stdout did not satisfy \"$out_pattern\""
    elif ! matches "$TMP/err" "$err_pattern"; then
        reason="stderr did not satisfy \"$err_pattern\""
    fi

    record "$description" "$reason"
}

# check_output <description> <expected output file> <arguments...> - the run
# must succeed and its stdout must match the expected file (a path relative to
# the package directory) exactly
check_output() {
    description=$1
    expected=$2
    shift 2

    run "$@"
    reason=""

    if [ "$STATUS" -ne 0 ]; then
        reason="expected exit status 0 but saw $STATUS"
    elif ! diff "$ROOT/$expected" "$TMP/out" > "$TMP/diff"; then
        reason="stdout differs from $expected: $(head -4 "$TMP/diff" | tr '\n' ' ')"
    fi

    record "$description" "$reason"
}

# Build and unit tests: the spec requires a build with no warnings.
cargo build 2> "$TMP/build"
build_status=$?
cat "$TMP/build"

if [ "$build_status" -ne 0 ]; then
    exit 1
fi

if grep -q "^warning" "$TMP/build"; then
    record "build has no warnings" "cargo build printed a warning"
else
    record "build has no warnings" ""
fi

if cargo test -q > "$TMP/unit" 2>&1; then
    record "unit tests pass" ""
else
    cat "$TMP/unit"
    record "unit tests pass" "cargo test failed"
fi

printf '\n'

# Command line handling: a badly formed command line is error code 1.
check "no arguments prints usage"            1 "^usage: .* <script_file_name> \[whinge\]$" -
check "too many arguments prints usage"      1 "^usage: " - partial_hamlet_act_ii_script.txt whinge extra
check "unrecognized optional argument"       1 "^usage: " - partial_hamlet_act_ii_script.txt complain

# The provided Hamlet script.  Its expected output covers Scene I only and puts
# a trailing space after each speaker name, so compare that many lines of our
# output against it while ignoring differences in whitespace.
run partial_hamlet_act_ii_script.txt
head -n "$(wc -l < "$SAMPLE")" "$TMP/out" > "$TMP/scene1"

if [ "$STATUS" -ne 0 ]; then
    record "Hamlet Scene I matches hamlet_act_ii_scene_1.txt" "exit status $STATUS"
elif ! diff -b "$SAMPLE" "$TMP/scene1" > "$TMP/diff"; then
    record "Hamlet Scene I matches hamlet_act_ii_scene_1.txt" "$(head -4 "$TMP/diff" | tr '\n' ' ')"
else
    record "Hamlet Scene I matches hamlet_act_ii_scene_1.txt" ""
fi

check_output "full Hamlet script matches expected output" test_inputs/expected_hamlet.txt partial_hamlet_act_ii_script.txt
check "Hamlet whinges about line 0 on stderr"  0 "!WARNING" "^WARNING: Line 0 is missing$" partial_hamlet_act_ii_script.txt whinge
check "Hamlet is silent on stderr by default"  0 "^\[Exit King\.\]$" "!." partial_hamlet_act_ii_script.txt

# Hand-made inputs, run from inside test_inputs/.
DIR=$ROOT/test_inputs

# Entrances and exits across fragments and scenes.
check_output "characters exit, re-enter, and titles start scenes" test_inputs/expected_recurring.txt recurring_script.txt
check_output "same cast twice in a row stays on stage" test_inputs/expected_repeat.txt repeat_script.txt

# Tied line numbers: across players the earlier sorted player speaks first
# (config order among players tied on their first line), and within one player
# lines that share a number keep their part file order rather than sorting by text.
check_output "tied line numbers keep config and file order" test_inputs/expected_ties.txt tie_script.txt
check "tied line numbers whinge as duplicates"   0 "!WARNING" "^WARNING: Line 7 is duplicated$" tie_script.txt whinge

# Well formed input from a second play.
check "Macbeth: lines out of order are sorted"  0 "^A friend\.$"         "!." macbeth_script.txt
check "Macbeth: silent character enters"        0 "^\[Enter Servant\.\]$" -   macbeth_script.txt
check "Macbeth: silent character never speaks"  0 "!^Servant\.$"         -   macbeth_script.txt

# Formatting tolerance.
check "blank lines, tabs and extra spaces"      0 "^Spaced Out Title$"   "!." whitespace_script.txt
check "CRLF script and configuration"           0 "^Sentinel\.$"         "!." crlf_script.txt
check "carriage returns are trimmed"            0 "!$CR"                 -    crlf_script.txt
check "numeric edge cases are recited"          0 "^The largest usize value" "!." numeric_script.txt
check "numeric edge cases whinge"               0 "!WARNING" "^WARNING: Lines 13 to 18446744073709551614 are missing$" numeric_script.txt whinge

# Recoverable problems: skipped (with a complaint under whinge), still recited.
check "bare [scene] line is skipped"            0 "^A Titled Scene$"     "!." bare_scene_script.txt
check "bare [scene] line whinges"               0 "!WARNING" "no scene title" bare_scene_script.txt whinge
check "extra tokens after a config file"        0 "^Extra Tokens$"       "!." extra_tokens_script.txt
check "extra tokens whinge"                     0 "!WARNING" "^WARNING: Ignoring extra tokens" extra_tokens_script.txt whinge
check "second [scene] title replaces the first" 0 "^Surviving Title$"    -    double_title_script.txt
check "replaced [scene] title is never printed" 0 "!Overridden Title"    -    double_title_script.txt
check "fuzzed configuration still recites"      0 "^Messenger\.$"        "!." fuzz_script.txt
check "fuzzed configuration whinges"            0 "!WARNING" "^WARNING: Configuration line \"Lonely\"" fuzz_script.txt whinge
check "duplicate line whinges"                  0 "!WARNING" "^WARNING: Line 1 is duplicated$" gaps_script.txt whinge
check "missing lines whinge as a range"         0 "!WARNING" "^WARNING: Lines 2 to 4 are missing$" gaps_script.txt whinge
check "starting at line 0 is not a gap"         0 -          "!Line 0" gaps_script.txt whinge

# Unrecoverable problems: script generation failure is error code 2, and
# nothing is recited (stdout stays empty).
check "missing script file"                     2 "!." "^ERROR: Failed to read script file"   no_such_script.txt
check "empty script file"                       2 "!." "^ERROR: Script file .* is empty"     empty_script.txt
check "script of blank lines"                   2 "!." "^ERROR: Script file .* is empty"     blank_script.txt
check "script with no configuration files"     2 "!." "does not name any configuration"     titles_only_script.txt
check "configuration before any [scene] title"  2 "!." "must give a \[scene\] title"         untitled_script.txt
check "missing configuration file"              2 "!." "^ERROR: Failed to read configuration file" missing_config_script.txt
check "[Scene] is not the [scene] directive"    2 "!." "^ERROR: Failed to read configuration file \[Scene\]" wrong_case_script.txt
check "empty configuration file"                2 "!." "^ERROR: Configuration file .* cannot be empty" empty_config_script.txt
check "configuration of only malformed lines"  2 "!." "^ERROR: Configuration file junk_config.txt must have at least one valid line" junk_config_script.txt
check "malformed-only configuration whinges"   2 "!." "^WARNING: Configuration line \"Alone\" has 1 tokens" junk_config_script.txt whinge
check "malformed-only configuration is quiet"  2 "!." "!WARNING" junk_config_script.txt
check "missing part file"                       2 "!." "^ERROR: Failed to read part file no_such_part.txt" missing_part_script.txt
check "part file is a directory"                2 "!." "^ERROR: Failed to read part file a_directory" dir_part_script.txt
check "part file is not UTF-8"                  2 "!." "^ERROR: Failed to read part file binary.txt" binary_part_script.txt

printf '\n%d passed, %d failed\n' "$PASSES" "$FAILURES"
[ "$FAILURES" -eq 0 ]
