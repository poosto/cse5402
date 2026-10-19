#!/usr/bin/env python3
# TODO: adapt for Lab 2 (script files with [scene] directives, entrances/exits, whinge output on stderr).
"""Randomized fuzzer for CSE 5402 Lab 1.

Generates random configuration files and part files (including malformed,
adversarial, and pathological content), runs the program on them, and checks
invariants the lab spec requires:

  I1  the program never panics / never crashes (exit status must be 0, 1, or 2)
  I2  the program always terminates promptly (no hangs / infinite loops)
  I3  a successful run prints the title, then well-formed speaker blocks
  I4  recited lines appear in nondecreasing line-number order
  I5  a speaker is announced exactly when the speaker changes
  I6  every well-formed numbered line from a readable part file is recited
  I7  running without "whinge" prints no WARNING lines
  I8  the program is deterministic for identical input
"""

import os, random, re, shutil, subprocess, sys, tempfile

LAB1 = os.path.abspath(sys.argv[1])
ITERATIONS = int(sys.argv[2]) if len(sys.argv) > 2 else 2000
SEED = int(sys.argv[3]) if len(sys.argv) > 3 else 5402

NAMES = ["King", "Queen", "Hamlet", "Ghost", "A", "Ros_Guil", "Ünïcødé", "x" * 200, "123", "-", "..", "a.b"]

# Snippets of line text, each paired with the line number it should parse as,
# or None when the line is expected to be skipped as badly formed.
def gen_part_line(rng, used_numbers):
    kind = rng.choice([
        "good", "good", "good", "good", "good",
        "blank", "ws_only", "single_token", "bad_number", "negative",
        "huge", "float", "plus", "leading_ws", "tabs", "crlf",
        "zero", "usize_max", "overflow", "number_only_ws", "unicode",
    ])
    if kind == "good":
        n = rng.randint(0, 10_000)
        text = rng.choice(["To be or not to be", "Ay, my lord.", "What ho!",
                           "   spaced   out   ", "a", "x" * 300, "Ünïcødé — ✓"])
        sep = rng.choice([" ", "\t", "   ", " \t "])
        return (f"{n}{sep}{text}", n, text.strip())
    if kind == "blank":
        return ("", None, None)
    if kind == "ws_only":
        return (rng.choice(["   ", "\t", " \t  "]), None, None)
    if kind == "single_token":
        return (rng.choice(["42", "hello", "x" * 100]), None, None)
    if kind == "bad_number":
        return ("abc rest of line", None, None)
    if kind == "negative":
        return ("-5 negative line number", None, None)
    if kind == "huge":
        return ("9" * 40 + " too big for usize", None, None)
    if kind == "float":
        return ("3.14 not an integer", None, None)
    if kind == "plus":
        return ("+7 leading plus sign", 7, "leading plus sign")
    if kind == "leading_ws":
        # grab_trimmed_file_lines trims, so this is a well formed line
        n = rng.randint(0, 999)
        return (f"    {n}   indented line   ", n, "indented line")
    if kind == "tabs":
        n = rng.randint(0, 999)
        return (f"{n}\t\ttab separated", n, "tab separated")
    if kind == "crlf":
        n = rng.randint(0, 999)
        return (f"{n} carriage return line\r", n, "carriage return line")
    if kind == "zero":
        return ("0 zeroth line", 0, "zeroth line")
    if kind == "usize_max":
        n = 2**64 - 1
        return (f"{n} last line ever", n, "last line ever")
    if kind == "overflow":
        return (f"{2**64} overflows usize", None, None)
    if kind == "number_only_ws":
        # "5   " trims to "5" -> single token -> skipped
        return ("5    ", None, None)
    if kind == "unicode":
        n = rng.randint(0, 999)
        return (f"{n} Ünïcødé ✓ — ok", n, "Ünïcødé ✓ — ok")
    raise AssertionError(kind)


def gen_case(rng, workdir):
    """Build one random test case; return (config_name, expected_lines or None).

    expected_lines is the sorted list of (number, speaker, text) the program
    should recite, or None when we expect script generation to fail.
    """
    n_chars = rng.randint(0, 5)
    config_lines = []
    expected = []
    readable_parts = 0
    title = rng.choice(["Hamlet, ACT II, Scene II", "A Play", "   Title With   Spaces   ",
                        "Ünïcødé Title ✓", "x" * 300, "one"])
    config_lines.append(title)

    for i in range(n_chars):
        name = rng.choice(NAMES)
        flavor = rng.choice(["ok", "ok", "ok", "ok", "missing", "extra_tokens",
                             "one_token", "blank", "ws", "dir", "empty_file", "binary"])
        fname = f"part{i}.txt"

        if flavor == "blank":
            config_lines.append("")
            continue
        if flavor == "ws":
            config_lines.append(rng.choice(["   ", "\t\t"]))
            continue
        if flavor == "one_token":
            config_lines.append(name)
            continue
        if flavor == "missing":
            config_lines.append(f"{name} no_such_file_{i}.txt")
            continue
        if flavor == "dir":
            os.mkdir(os.path.join(workdir, f"dir{i}"))
            config_lines.append(f"{name} dir{i}")
            continue
        if flavor == "binary":
            with open(os.path.join(workdir, fname), "wb") as f:
                f.write(b"\xff\xfe\x00\x01 invalid utf-8\n")
            config_lines.append(f"{name} {fname}")
            continue

        # ok / extra_tokens / empty_file all have a readable part file
        body = []
        if flavor != "empty_file":
            for _ in range(rng.randint(0, 8)):
                raw, num, text = gen_part_line(rng, None)
                body.append(raw)
                if num is not None:
                    expected.append((num, name, text))

        newline = rng.choice(["\n", "\r\n"])
        trailing = rng.choice(["", newline])
        with open(os.path.join(workdir, fname), "w", encoding="utf-8") as f:
            f.write(newline.join(body) + trailing)
        readable_parts += 1

        if flavor == "extra_tokens":
            config_lines.append(f"{name} {fname} extra ignored tokens")
        else:
            sep = rng.choice([" ", "\t", "   "])
            config_lines.append(f"{name}{sep}{fname}")

    # sprinkle blank lines through the config
    for _ in range(rng.randint(0, 3)):
        config_lines.insert(rng.randint(1, len(config_lines)), rng.choice(["", "  ", "\t"]))

    config_name = "config.txt"
    newline = rng.choice(["\n", "\r\n"])
    with open(os.path.join(workdir, config_name), "w", encoding="utf-8") as f:
        f.write(newline.join(config_lines) + rng.choice(["", newline]))

    nonblank = [l for l in config_lines if l.strip()]
    if len(nonblank) < 2 or readable_parts == 0:
        expected = None   # script generation must fail
    else:
        expected.sort()
    return config_name, expected


def check_output(out, expected, whinge):
    """Verify invariants I3-I7 against the recited script."""
    lines = out.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    # Drop WARNING lines, which may be interleaved before the script is printed.
    warnings = [l for l in lines if l.startswith("WARNING: ")]
    if warnings and not whinge:
        return "printed WARNING lines without whinge mode"    # I7
    body = [l for l in lines if not l.startswith("WARNING: ")]

    if not body:
        return "printed no output at all"
    body = body[1:]   # I3: first line is the title

    recited = []
    speaker = None
    i = 0
    while i < len(body):
        if body[i] == "" and i + 1 < len(body) and body[i + 1].endswith("."):
            candidate = body[i + 1][:-1]
            # A speaker announcement is a blank line plus "Name." where Name is
            # one of the speakers we still expect to hear from.
            if expected is not None and any(c == candidate for _, c, _ in expected):
                if candidate == speaker:
                    return f"re-announced speaker {candidate!r} without a change"   # I5
                speaker = candidate
                i += 2
                continue
        recited.append((speaker, body[i]))
        i += 1

    if expected is None:
        return None
    exp = [(c, t) for _, c, t in expected]
    if recited != exp:
        return f"recited {recited!r} but expected {exp!r}"   # I4/I6
    return None


def main():
    rng = random.Random(SEED)
    failures = []
    for it in range(ITERATIONS):
        workdir = tempfile.mkdtemp(prefix="lab1fuzz")
        try:
            case_rng = random.Random(rng.randrange(2**63))
            config, expected = gen_case(case_rng, workdir)
            whinge = case_rng.random() < 0.5
            argv = [LAB1, config] + (["whinge"] if whinge else [])
            try:
                p = subprocess.run(argv, cwd=workdir, capture_output=True,
                                   text=True, timeout=10)
            except subprocess.TimeoutExpired:
                failures.append((workdir, "TIMEOUT (I2)", ""))   # I2
                workdir = None
                continue

            if p.returncode not in (0, 1):
                failures.append((workdir, f"bad exit status {p.returncode} (I1)",
                                 p.stdout + p.stderr))
                workdir = None
                continue

            # main returns Result<(), u8>, so the lab's error code shows up in
            # the "Error: N" line Rust prints on stderr, not in the exit status.
            reported = re.findall(r"^Error: (\d+)$", p.stderr, re.M)
            want_ok = expected is not None
            if want_ok and (p.returncode != 0 or reported):
                failures.append((workdir, f"expected success, got status {p.returncode} {reported}",
                                 p.stdout + p.stderr))
                workdir = None
                continue
            if not want_ok and reported != ["2"]:
                failures.append((workdir, f"expected script-gen error code 2, got {reported!r} (status {p.returncode})",
                                 p.stdout + p.stderr))
                workdir = None
                continue

            if want_ok:
                problem = check_output(p.stdout, expected, whinge)
                if problem:
                    failures.append((workdir, problem, p.stdout))
                    workdir = None
                    continue

            # I8: determinism
            q = subprocess.run(argv, cwd=workdir, capture_output=True, text=True, timeout=10)
            if q.stdout != p.stdout or q.returncode != p.returncode:
                failures.append((workdir, "nondeterministic output (I8)", p.stdout))
                workdir = None
                continue
        finally:
            if workdir:
                shutil.rmtree(workdir, ignore_errors=True)

    print(f"{ITERATIONS - len(failures)}/{ITERATIONS} cases passed")
    for wd, why, out in failures[:10]:
        print(f"\nFAIL: {why}\n  repro dir: {wd}\n  output:\n{out[:1500]}")
    return 1 if failures else 0


sys.exit(main())
