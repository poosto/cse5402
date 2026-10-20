#!/bin/sh
# TODO: adapt for Lab 2 (binary name, script files instead of config files, new inputs). Lab 1 inputs now live in ../lab1.
#
# run_tests.sh - regression tests for CSE 5402 Lab 1.
#
# Builds the program, then runs it over every configuration file in this
# directory and checks the process exit status, the error code the program
# reports, and a distinguishing piece of its output.  Prints PASS or FAIL for
# each case and exits non-zero if any case failed.
#
# Usage: sh run_tests.sh

LAB1=./target/debug/lab2
PASSES=0
FAILURES=0

# check <description> <expected exit status> <expected error tag> <expected
# output pattern> <arguments...>
#
# The expected error tag is the "Error: N" line that Rust's Termination
# implementation prints on standard error when main returns Err(N), or "-" when
# the program is expected to succeed.  The expected output pattern is a grep
# pattern that must appear somewhere in the program's combined output, or "-"
# when no particular text is required; prefixing the pattern with "!" instead
# requires that the pattern does not appear anywhere in the output.
check() {
    description=$1
    expected_status=$2
    expected_error=$3
    expected_pattern=$4
    shift 4

    output=$("$LAB1" "$@" 2>&1)
    status=$?
    reason=""

    if [ "$status" -ne "$expected_status" ]; then
        reason="expected exit status $expected_status but saw $status"
    elif [ "$expected_error" != "-" ] && ! printf '%s\n' "$output" | grep -q "^$expected_error\$"; then
        reason="expected the program to report \"$expected_error\""
    elif [ "$expected_error" = "-" ] && printf '%s\n' "$output" | grep -q "^Error: "; then
        reason="the program reported an error but was expected to succeed"
    elif [ "$expected_pattern" != "-" ]; then
        case $expected_pattern in
            !*)
                unwanted_pattern=${expected_pattern#!}

                if printf '%s\n' "$output" | grep -q "$unwanted_pattern"; then
                    reason="expected the output not to contain \"$unwanted_pattern\""
                fi
                ;;
            *)
                if ! printf '%s\n' "$output" | grep -q "$expected_pattern"; then
                    reason="expected the output to contain \"$expected_pattern\""
                fi
                ;;
        esac
    fi

    if [ -z "$reason" ]; then
        PASSES=$((PASSES + 1))
        printf 'PASS  %s\n' "$description"
    else
        FAILURES=$((FAILURES + 1))
        printf 'FAIL  %s (%s)\n' "$description" "$reason"
    fi
}

cargo build || exit 1
printf '\n'

# Command line handling: a badly formed command line is error code 1.
check "no arguments prints usage"                1 "Error: 1" "^usage: "
check "too many arguments prints usage"          1 "Error: 1" "^usage: " hamlet_ii_2_config.txt whinge extra
check "unrecognized optional argument"           1 "Error: 1" "^usage: " hamlet_ii_2_config.txt complain

# Well formed input: the reference play fragment and a second play.
check "reference Hamlet fragment"                0 "-" "^King\.$"            hamlet_ii_2_config.txt
check "Macbeth fragment, lines out of order"     0 "-" "^A friend\.$"        macbeth_ii_1_config.txt
check "silent character is never announced"      0 "-" "!^Servant\."         macbeth_ii_1_config.txt
check "blank lines and extra whitespace"         0 "-" "^Guildenstern\.$"    whitespace_config.txt

# Recoverable problems: the program skips what it cannot use and still recites.
check "fuzzed configuration and part files"      0 "-" "^Messenger\.$"       fuzz_config.txt
check "fuzzed input warns under whinge"          0 "-" "^WARNING: "          fuzz_config.txt whinge
check "fuzzed input stays quiet without whinge"  0 "-" "-"                   fuzz_config.txt
check "badly formed line numbers are skipped"    0 "-" "^Ghost\.$"           numeric_config.txt
check "carriage returns are trimmed"             0 "-" "^Sentinel\.$"        crlf_config.txt
check "unreadable part file is skipped"          0 "-" "^King\.$"            dir_part_config.txt

# Unrecoverable problems: script generation failure is error code 2.
check "missing configuration file"               1 "Error: 2" "^ERROR: "     no_such_config.txt
check "configuration with no character lines"    1 "Error: 2" "^ERROR: "     title_only_config.txt
check "configuration with no readable parts"     1 "Error: 2" "^ERROR: "     no_valid_parts_config.txt

printf '\n%d passed, %d failed\n' "$PASSES" "$FAILURES"
[ "$FAILURES" -eq 0 ]
