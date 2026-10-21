#!/usr/bin/env python3
"""Randomized fuzzer for CSE 5402 Lab 2.

Generates random script files, scene fragment configuration files, and part
files (including malformed, adversarial, and pathological content), runs the
program on them, and compares the program against an independent Python model
of the behavior the lab spec requires.  The model predicts the exact standard
output, the exit status, and how many whinge warnings should appear.  Checked
invariants:

  I1  the program never panics or crashes (exit status must be 0 or 2)
  I2  the program always terminates promptly (no hangs or infinite loops)
  I3  a successful run's stdout matches the model exactly: scene titles only
      on a scene's first fragment, entrances in first-speaking order, lines in
      line-number order, speaker changes announced, exits in reverse order
  I4  a failed run reports "Error: 2", prints an ERROR message, and recites
      nothing on stdout
  I5  whinge output goes only to stderr, never stdout; without whinge, stderr
      is empty on success
  I6  with whinge, the number of WARNING lines matches the model
  I7  the program is deterministic for identical input

Before the random cases, the model is also checked against the provided Hamlet
script and the hand-made inputs in test_inputs/, so a model bug shows up there.

Usage: python3 fuzz_test.py <path to lab2 binary> [iterations] [seed]
"""

import os, random, re, shutil, subprocess, sys, tempfile

LAB2 = os.path.abspath(sys.argv[1])
ITERATIONS = int(sys.argv[2]) if len(sys.argv) > 2 else 2000
SEED = int(sys.argv[3]) if len(sys.argv) > 3 else 5402

ROOT = os.path.dirname(os.path.abspath(__file__))
USIZE_MAX = 2**64 - 1
SCENE_DIRECTIVE = "[scene]"
PROGRESS_EVERY = 200   # random cases between progress lines

# Rust's trim() and split_whitespace() use the Unicode White_Space property,
# which differs slightly from Python's str.isspace(), so spell it out.
RUST_WS = ("\t\n\x0b\x0c\r \x85\xa0 "
           + "".join(chr(c) for c in range(0x2000, 0x200B))
           + "    　")
WS_RUN = re.compile("[" + re.escape(RUST_WS) + "]+")


# ---------------------------------------------------------------------------
# Model of the program
# ---------------------------------------------------------------------------

class Failed(Exception):
    """Script generation fails (the program should exit with error code 2)."""


def trim(s):
    return s.strip(RUST_WS)


def split_ws(s):
    return [t for t in WS_RUN.split(s) if t]


def read_lines(path):
    """grab_trimmed_file_lines: every line trimmed, or Failed if unreadable."""
    try:
        with open(path, "rb") as f:
            data = f.read()
        text = data.decode("utf-8")
    except (OSError, UnicodeDecodeError):
        raise Failed(path)
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()   # read_line reports EOF rather than a final empty line
    return [trim(line) for line in lines]


def parse_usize(token):
    """str::parse::<usize>: an optional '+' then ASCII digits, within range."""
    digits = token[1:] if token.startswith("+") else token
    if not digits or any(c not in "0123456789" for c in digits):
        return None
    value = int(digits)
    return value if value <= USIZE_MAX else None


class Model:
    def __init__(self, workdir):
        self.workdir = workdir
        self.out = []
        self.warnings = 0

    def path(self, name):
        return os.path.join(self.workdir, name)

    def load_player(self, name, part_file):
        lines = []
        for line in read_lines(self.path(part_file)):
            if not line:
                continue
            match = re.search("[" + re.escape(RUST_WS) + "]", line)
            if not match:
                self.warnings += 1   # only one token
                continue
            number = parse_usize(line[:match.start()])
            if number is None:
                self.warnings += 1   # first token is not a usize
                continue
            lines.append((number, trim(line[match.end():])))
        # sort by line number only, and Python's sort is stable like Rust's
        # sort_by_key, so lines that share a number keep their file order
        lines.sort(key=lambda entry: entry[0])
        return {"name": name, "lines": lines, "index": 0}

    def load_fragment(self, config_file, title):
        lines = [line for line in read_lines(self.path(config_file)) if line]
        if not lines:
            raise Failed(config_file)
        characters = []
        for line in lines:
            words = split_ws(line)
            if len(words) != 2:
                self.warnings += 1
            if len(words) >= 2:
                characters.append((words[0], words[1]))
        # a config whose lines are all malformed names no characters, so it fails
        # like an empty one, before any part file is read
        if not characters:
            raise Failed(config_file)
        players = [self.load_player(name, part) for name, part in characters]
        # silent players first, then by first line number; Python's sort is
        # stable, like Rust's Vec::sort
        players.sort(key=lambda p: (1, p["lines"][0][0]) if p["lines"] else (0,))
        return {"title": title, "players": players}

    def prepare(self, script_file):
        lines = [line for line in read_lines(self.path(script_file)) if line]
        if not lines:
            raise Failed(script_file)
        config = []
        for line in lines:
            words = split_ws(line)
            if words == [SCENE_DIRECTIVE]:
                self.warnings += 1
            elif words[0] == SCENE_DIRECTIVE:
                config.append((True, " ".join(words[1:])))
            else:
                if len(words) > 1:
                    self.warnings += 1
                config.append((False, words[0]))
        fragments, title = [], ""
        for is_new_scene, text in config:
            if is_new_scene:
                title = text
            else:
                fragments.append(self.load_fragment(text, title))
                title = ""
        if not fragments or fragments[0]["title"] == "":
            raise Failed(script_file)
        return fragments

    def recite_fragment(self, fragment):
        players = fragment["players"]
        expected, recent = 0, None
        while True:
            # Ties on a line number go to the player sorted earlier in the
            # fragment, matching recite's min() over (line, index) tuples.
            candidates = [(p["lines"][p["index"]][0], i)
                          for i, p in enumerate(players) if p["index"] < len(p["lines"])]
            if not candidates:
                return
            line, i = min(candidates)
            if line != expected:
                self.warnings += 1   # missing range or duplicated line
            player = players[i]
            if player["name"] != recent:
                self.out += ["", player["name"] + "."]
                recent = player["name"]
            self.out.append(player["lines"][player["index"]][1])
            player["index"] += 1
            expected = min(line + 1, USIZE_MAX)

    def recite(self, fragments):
        for i, fragment in enumerate(fragments):
            previous = {p["name"] for p in fragments[i - 1]["players"]} if i > 0 else set()
            entering = [p for p in fragment["players"] if p["name"] not in previous]
            if trim(fragment["title"]):
                if i > 0:
                    self.out.append("")
                self.out.append(fragment["title"])
                if entering:
                    self.out.append("")
            self.out += ["[Enter %s.]" % p["name"] for p in entering]

            self.recite_fragment(fragment)

            following = ({p["name"] for p in fragments[i + 1]["players"]}
                         if i + 1 < len(fragments) else set())
            exiting = [p for p in reversed(fragment["players"]) if p["name"] not in following]
            if exiting:
                self.out.append("")
            self.out += ["[Exit %s.]" % p["name"] for p in exiting]


def predict(workdir, script_file):
    """Return (expected stdout or None on failure, expected warning count)."""
    model = Model(workdir)
    try:
        fragments = model.prepare(script_file)
    except Failed:
        return None, None
    model.recite(fragments)
    return "".join(line + "\n" for line in model.out), model.warnings


# ---------------------------------------------------------------------------
# Running and checking the program
# ---------------------------------------------------------------------------

def run(workdir, script_file, whinge):
    argv = [LAB2, script_file] + (["whinge"] if whinge else [])
    # stdout/stderr=PIPE rather than capture_output=True, which needs Python 3.7
    # (some CEC machines have 3.6)
    return subprocess.run(argv, cwd=workdir, stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE, timeout=10)


def check(workdir, script_file, whinge):
    """Return None if the program agrees with the model, else a description."""
    expected_out, expected_warnings = predict(workdir, script_file)
    try:
        p = run(workdir, script_file, whinge)
    except subprocess.TimeoutExpired:
        return "TIMEOUT (I2)"
    out = p.stdout.decode("utf-8", "replace")
    err = p.stderr.decode("utf-8", "replace")
    err_lines = err.splitlines()
    warnings = [l for l in err_lines if l.startswith("WARNING: ")]

    if p.returncode not in (0, 2):
        return "bad exit status %d (I1)" % p.returncode
    if "WARNING" in out:
        return "WARNING printed on stdout (I5)"
    if not whinge and warnings:
        return "WARNING printed without whinge mode (I5)"

    if expected_out is None:
        if p.returncode != 2 or "Error: 2" not in err_lines:
            return "expected script generation to fail with Error: 2, got status %d (I4)" % p.returncode
        if not any(l.startswith("ERROR: ") for l in err_lines):
            return "failed without an ERROR message (I4)"
        if out:
            return "failed but still printed to stdout (I4)"
    else:
        if p.returncode != 0:
            return "expected success, got status %d (I1)" % p.returncode
        if out != expected_out:
            return "stdout differs from the model (I3)\n--- expected:\n%s\n--- actual:\n%s" % (
                expected_out[:1500], out[:1500])
        if not whinge and err:
            return "stderr not empty on a quiet successful run (I5)"
        if whinge and len(warnings) != expected_warnings:
            return "expected %d WARNING lines, saw %d (I6)" % (expected_warnings, len(warnings))
        if len(warnings) != len(err_lines):
            return "unexpected non-WARNING stderr on success: %r" % err[:300]

    try:
        q = run(workdir, script_file, whinge)
    except subprocess.TimeoutExpired:
        return "TIMEOUT on the repeat run (I2)"
    if q.stdout != p.stdout or q.stderr != p.stderr or q.returncode != p.returncode:
        return "nondeterministic output (I7)"
    return None


# ---------------------------------------------------------------------------
# Random input generation
# ---------------------------------------------------------------------------

NAMES = ["King", "Queen", "Hamlet", "Ghost", "A", "Ros_Guil", "Ünïcødé",
         "x" * 60, "123", "-", "..", "a.b"]
TEXTS = ["To be or not to be", "Ay, my lord.", "What ho!", "   spaced   out   ",
         "a", "x" * 300, "Ünïcødé — ✓", "[scene] not a directive here"]
TITLE_WORDS = ["Hamlet", "ACT", "II", "Scene", "I", "A", "room", "Ünïcødé", "✓",
               "[scene]", "x" * 80]


def gen_part_line(rng):
    kind = rng.choice(["good"] * 6 + [
        "blank", "ws_only", "single_token", "bad_number", "negative", "huge",
        "float", "plus", "lone_plus", "leading_ws", "tabs", "zero", "usize_max",
        "overflow", "number_only_ws", "unicode", "fullwidth_digit"])
    n = rng.randint(0, 40)
    if kind == "good":
        sep = rng.choice([" ", "\t", "   ", " \t "])
        return f"{n}{sep}{rng.choice(TEXTS)}"
    return {
        "blank": "",
        "ws_only": rng.choice(["   ", "\t", " \t  "]),
        "single_token": rng.choice(["42", "hello", "x" * 100]),
        "bad_number": "abc rest of line",
        "negative": "-5 negative line number",
        "huge": "9" * 40 + " too big for usize",
        "float": "3.14 not an integer",
        "plus": f"+{n} leading plus sign",
        "lone_plus": "+ just a plus sign",
        "leading_ws": f"    {n}   indented line   ",
        "tabs": f"{n}\t\ttab separated",
        "zero": "0 zeroth line",
        "usize_max": f"{USIZE_MAX} last line ever",
        "overflow": f"{USIZE_MAX + 1} overflows usize",
        "number_only_ws": f"{n}    ",
        "unicode": f"{n} Ünïcødé ✓ — ok",
        "fullwidth_digit": "７ not an ASCII digit",
    }[kind]


def write_lines(rng, path, lines):
    newline = rng.choice(["\n", "\n", "\r\n"])
    trailing = rng.choice(["", newline])
    with open(path, "w", encoding="utf-8", newline="") as f:
        f.write(newline.join(lines) + trailing)


def gen_part_file(rng, workdir, index):
    """Create a part file (usually readable) and return its name."""
    flavor = rng.choices(["ok", "empty", "missing", "dir", "binary"],
                         weights=[90, 4, 2, 2, 2])[0]
    name = f"part{index}.txt"
    if flavor == "missing":
        return f"no_such_part{index}.txt"
    if flavor == "dir":
        os.mkdir(os.path.join(workdir, f"dir{index}"))
        return f"dir{index}"
    if flavor == "binary":
        with open(os.path.join(workdir, name), "wb") as f:
            f.write(b"1 fine\n\xff\xfe\x00\x01 invalid utf-8\n")
        return name
    count = 0 if flavor == "empty" else rng.randint(0, 6)
    write_lines(rng, os.path.join(workdir, name), [gen_part_line(rng) for _ in range(count)])
    return name


def gen_config_file(rng, workdir, index, part_files):
    """Create a scene fragment configuration file and return its name."""
    name = f"config{index}.txt"
    if rng.random() < 0.03:
        return f"no_such_config{index}.txt"
    lines = []
    for _ in range(rng.randint(0, 4)):
        flavor = rng.choices(["ok", "one_token", "extra_tokens", "blank"],
                             weights=[85, 5, 5, 5])[0]
        character = rng.choice(NAMES)
        part = rng.choice(part_files)
        sep = rng.choice([" ", "\t", "   "])
        lines.append({
            "ok": f"{character}{sep}{part}",
            "one_token": character,
            "extra_tokens": f"{character} {part} extra ignored tokens",
            "blank": rng.choice(["", "  ", "\t"]),
        }[flavor])
    write_lines(rng, os.path.join(workdir, name), lines)
    return name


def gen_title(rng):
    words = [rng.choice(TITLE_WORDS) for _ in range(rng.randint(1, 5))]
    return "".join(rng.choice([" ", "  ", "\t"]) + w for w in words)


def gen_case(rng, workdir):
    """Build one random script (with its files) and return the script name."""
    part_files = [gen_part_file(rng, workdir, i) for i in range(rng.randint(1, 6))]
    config_files = [gen_config_file(rng, workdir, i, part_files)
                    for i in range(rng.randint(1, 4))]
    lines = []
    if rng.random() < 0.9:
        lines.append(SCENE_DIRECTIVE + gen_title(rng))   # usually starts with a title
    for _ in range(rng.randint(0, 7)):
        flavor = rng.choices(["config", "scene", "bare_scene", "blank", "extra_tokens"],
                             weights=[55, 20, 5, 10, 10])[0]
        lines.append({
            "config": lambda: rng.choice(config_files),
            "scene": lambda: rng.choice(["", "  "]) + SCENE_DIRECTIVE + gen_title(rng),
            "bare_scene": lambda: SCENE_DIRECTIVE + rng.choice(["", "   "]),
            "blank": lambda: rng.choice(["", "   ", "\t"]),
            "extra_tokens": lambda: rng.choice(config_files) + " extra tokens",
        }[flavor]())
    script = "script.txt"
    if rng.random() < 0.02:
        return "no_such_script.txt"
    write_lines(rng, os.path.join(workdir, script), lines)
    return script


# ---------------------------------------------------------------------------

def main():
    failures = []

    # The model must agree with the program on the hand-checked inputs first.
    fixed = [(ROOT, "partial_hamlet_act_ii_script.txt")]
    inputs = os.path.join(ROOT, "test_inputs")
    fixed += [(inputs, f) for f in sorted(os.listdir(inputs)) if f.endswith("_script.txt")]
    for workdir, script in fixed:
        for whinge in (False, True):
            problem = check(workdir, script, whinge)
            if problem:
                failures.append((os.path.join(workdir, script), problem))

    # Progress goes to stderr, and only on a terminal, so a long run does not
    # look frozen but gen_output.sh's captured output stays the same.
    show_progress = sys.stderr.isatty()

    rng = random.Random(SEED)
    for done in range(1, ITERATIONS + 1):
        workdir = tempfile.mkdtemp(prefix="lab2fuzz")
        case_rng = random.Random(rng.randrange(2**63))
        script = gen_case(case_rng, workdir)
        problem = check(workdir, script, case_rng.random() < 0.5)
        if problem:
            failures.append((workdir, problem))   # keep the directory to reproduce
        else:
            shutil.rmtree(workdir, ignore_errors=True)
        if show_progress and (done % PROGRESS_EVERY == 0 or done == ITERATIONS):
            print(f"  {done}/{ITERATIONS} random cases run, {len(failures)} failed",
                  file=sys.stderr, flush=True)

    total = 2 * len(fixed) + ITERATIONS
    print(f"{total - len(failures)}/{total} cases passed "
          f"({2 * len(fixed)} fixed, {ITERATIONS} random, seed {SEED})")
    for where, why in failures[:10]:
        print(f"\nFAIL: {why}\n  repro: {where}")
    return 1 if failures else 0


sys.exit(main())
