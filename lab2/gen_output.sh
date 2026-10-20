#!/bin/sh
#
# gen_output.sh - regenerate test_output.txt for CSE 5402 Lab 2.
#
# Builds the program, then runs it over a representative set of inputs and
# captures everything into test_output.txt: the provided Hamlet script (with
# whinge mode's stderr captured on its own, to show it never reaches stdout),
# characters exiting and re-entering across fragments, a second play, the
# fuzzed inputs with and without whinge mode, one failure per kind of error
# with its exit status, the full regression suite, and a fuzzer run.  Each
# section is labeled with the command that produced it, and a non-zero exit
# status is recorded beneath the output it came from.
#
# Run this on a CEC Linux Lab machine (after "module add rust-2023") so the
# captured output reflects the 1.71.0 toolchain the solution is graded with.
#
# Usage: sh gen_output.sh      (from the lab2 package directory)

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

# in_inputs <command...> - run a command from inside test_inputs/, where the
# hand-made scripts name their configuration and part files
in_inputs() {
    (cd test_inputs && "$@")
}

# stderr_only <command...> - show only what a command writes to standard error
stderr_only() {
    "$@" 2>&1 > /dev/null
}

cargo build -q || exit 1

{
    printf '================================================================\n'
    printf 'CSE 5402 Fall 2025 Lab 2 - captured output from a test run\n'
    printf '================================================================\n'

    emit 'cargo run -q partial_hamlet_act_ii_script.txt' \
         cargo run -q partial_hamlet_act_ii_script.txt

    emit 'cargo run -q partial_hamlet_act_ii_script.txt whinge 2>&1 > /dev/null   (stderr only)' \
         stderr_only cargo run -q partial_hamlet_act_ii_script.txt whinge

    emit 'cd test_inputs && cargo run -q recurring_script.txt' \
         in_inputs cargo run -q recurring_script.txt

    emit 'cd test_inputs && cargo run -q macbeth_script.txt' \
         in_inputs cargo run -q macbeth_script.txt

    emit 'cd test_inputs && cargo run -q fuzz_script.txt whinge' \
         in_inputs cargo run -q fuzz_script.txt whinge

    emit 'cd test_inputs && cargo run -q fuzz_script.txt        (same input, no whinge)' \
         in_inputs cargo run -q fuzz_script.txt

    emit 'cd test_inputs && cargo run -q bare_scene_script.txt whinge' \
         in_inputs cargo run -q bare_scene_script.txt whinge

    emit 'cd test_inputs && cargo run -q gaps_script.txt whinge' \
         in_inputs cargo run -q gaps_script.txt whinge

    emit 'cd test_inputs && cargo run -q numeric_script.txt whinge' \
         in_inputs cargo run -q numeric_script.txt whinge

    emit 'cargo run -q' \
         cargo run -q

    emit 'cargo run -q no_such_script.txt' \
         cargo run -q no_such_script.txt

    emit 'cd test_inputs && cargo run -q untitled_script.txt' \
         in_inputs cargo run -q untitled_script.txt

    emit 'cd test_inputs && cargo run -q missing_part_script.txt whinge' \
         in_inputs cargo run -q missing_part_script.txt whinge

    emit 'cd test_inputs && cargo run -q junk_config_script.txt whinge' \
         in_inputs cargo run -q junk_config_script.txt whinge

    emit 'sh run_tests.sh' \
         sh run_tests.sh

    emit 'python3 fuzz_test.py target/debug/lab2 2000' \
         python3 fuzz_test.py target/debug/lab2 2000
} > "$OUTPUT"

printf 'wrote %s (%d lines)\n' "$OUTPUT" "$(wc -l < "$OUTPUT")"
