================================================================================
SECTION 1: IDENTIFICATION AND DESIGN
================================================================================

CSE 5402 Fall 2026 Lab 3

Team members:
- Preston Meek: meek@wustl.edu

--------------------------------------------------------------------------------
Summary
--------------------------------------------------------------------------------

TODO: Overview of the three packages (lab3client, lab3server, lab3testclient)
      and how they fit together.

--------------------------------------------------------------------------------
Design decisions
--------------------------------------------------------------------------------

TODO: Step 1. The Lab 2 code moved into lab3client/src/lab3. Hard coded 0 and 1
      values from Lab 2 are now named consts (FIRST_FRAGMENT_IDX,
      NEIGHBOR_FRAGMENT_OFFSET, FIRST_LINE_IDX, LINE_IDX_STEP,
      NEIGHBOR_LINE_OFFSET), based on the Lab 2 grading feedback.

TODO: Step 2. The write_error! and write_message! macros.
      - Both wrap writeln! on a locked stream, as step 2 asks. write_error!
        writes to stderr and write_message! writes to stdout.
      - Each one turns a failed write into the new error code 3,
        ERR_FAILED_TO_WRITE_OUTPUT. The ? stays at each call site so the early
        return is easy to see.
      - Why a macro: it keeps the write and the error handling in one place
        instead of repeating them at every call. A function would not work as
        well because format strings need a macro.
      - Each macro has two forms. One takes a lock the caller already holds, so
        several lines can be kept together while other threads wait. The other
        takes the lock itself for just one line. The lock form checks the lock's
        type, so a stdout lock cannot be passed to write_error! by mistake.
      - Explain the choice that a failed write returns 3 even if another error
        (code 2) was already being reported. Also explain the same choice in
        ReturnWrapper::report.
      - Explain where the lock is held across more than one line (for example
        Player::speak printing the name and then the line) and why.

TODO: Step 3. Play holds Arc<Mutex<SceneFragment>>.

TODO: Step 4. SceneFragment holds Arc<Mutex<Player>>, and the custom compare
      function used with sort_by.

TODO: Steps 5 and 6. Running SceneFragment::prepare and Player::prepare on
      threads, joining them, and turning errors into panics.

TODO: Steps 7 to 12. The lab3server and lab3testclient design.

TODO: Steps 13 and 14. get_buffered_reader and reading files from net: addresses.

TODO: Exit codes. List all of them, now including 3.

================================================================================
SECTION 2: BUILDING AND RUNNING ON THE CEC LINUX LAB MACHINES
================================================================================

TODO: How to unpack, build, and run each package on the Linux Lab machines,
      using module add rust-2023.

================================================================================
SECTION 3: DEVELOPMENT AND TESTING
================================================================================

--------------------------------------------------------------------------------
Development stages
--------------------------------------------------------------------------------

TODO: Each refactoring stage and how it was checked against Lab 2's output.

--------------------------------------------------------------------------------
Testing
--------------------------------------------------------------------------------

TODO: Well formed input, badly formed input, fuzzed input, mixed local and
      remote files, and several clients at once (steps 15 to 19).

TODO: Write failure tests. Running with stderr sent to a closed pipe gives exit
      code 3.

================================================================================
SECTION 4: AI USAGE AND CITATIONS
================================================================================

TODO: APA citations for each AI chat, with transcripts in ai_transcripts/.
