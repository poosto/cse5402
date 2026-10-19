#!/bin/sh
#
# gen_output.sh - regenerate test_output.txt for CSE 5402 Lab 1.
#
# Builds the program, then runs it over a representative set of inputs and
# captures everything into test_output.txt: the two well formed plays, the
# fuzzed configuration with and without whinge mode, the numeric edge cases,
# two of the unrecoverable failures, and the full regression suite.  Each
# section is labeled with the command that produced it, and a non-zero exit
# status is recorded beneath the output it came from.
#
# Run this on a CEC Linux Lab machine (after "module add rust-2023") so the
# captured output reflects the 1.71.0 toolchain the solution is graded with.
#
# Usage: sh gen_output.sh

OUTPUT=test_output.txt

# emit <label> <command...> - run one command under a labeled banner.
emit() {
    label=$1
    shift

    printf '\n$ %s\n' "$label"
    printf -- '----------------------------------------------------------------\n'

    "$@" 2>&1
    status=$?

    if [ "$status" -ne 0 ]; then
        printf '(exit status %d)\n' "$status"
    fi

    return 0
}

cargo build -q || exit 1

{
    printf '================================================================\n'
    printf 'CSE 5402 Fall 2025 Lab 1 - captured output from a test run\n'
    printf '================================================================\n'

    emit 'cargo run -q hamlet_ii_2_config.txt' \
         cargo run -q hamlet_ii_2_config.txt

    emit 'cargo run -q macbeth_ii_1_config.txt' \
         cargo run -q macbeth_ii_1_config.txt

    emit 'cargo run -q fuzz_config.txt whinge' \
         cargo run -q fuzz_config.txt whinge

    emit 'cargo run -q fuzz_config.txt        (same input, no whinge)' \
         cargo run -q fuzz_config.txt

    emit 'cargo run -q numeric_config.txt whinge' \
         cargo run -q numeric_config.txt whinge

    emit 'cargo run -q no_valid_parts_config.txt whinge' \
         cargo run -q no_valid_parts_config.txt whinge

    emit 'cargo run -q' \
         cargo run -q

    emit 'sh run_tests.sh' \
         sh run_tests.sh
} > "$OUTPUT"

printf 'wrote %s (%d lines)\n' "$OUTPUT" "$(wc -l < "$OUTPUT")"
