# CSE 5402 Lab 2

- **Source of truth:** https://www.cse.wustl.edu/~cdgill/courses/cse5402/lab2/index.html. If anything here conflicts with it, the spec wins. Course coding guidelines: https://www.cse.wustl.edu/~cdgill/courses/cse5402/Guidelines.html
- **AI use policy:** `../CSE_5402_Generative_AI_Use_Policy.pdf`. Any AI help must be cited in the ReadMe (APA style: company, date, chat title, [Generative AI chat], model, link or exported transcript in `ai_transcripts/` as an appendix).
- Must build with **no warnings** on Rust 1.71.0 (CEC Linux Lab, `module add rust-2023`). Don't use newer language or std features.
- Lab 1 input files are in `../lab1` for checking that the program still runs as it did in Lab 1 while refactoring.
- Every `.rs` file needs a header comment explaining its purpose, and every function needs a summary comment (Lab 1 lost points for missing these).
- **Working style:** I write the majority of the code myself. Don't implement lab features unless I ask. Use AI for: answering questions, testing, cleanup/refactors I request, documentation (ReadMe, comments), and verifying that every spec requirement is met.
- **Testing should be exhaustive**, in the style of Lab 1 (`run_tests.sh` regression suite, `fuzz_test.py` randomized fuzzer, `gen_output.sh` captured output, hand-made malformed/edge-case input files). Check every spec requirement, including whinge output going to stderr and distinct exit codes (`echo $?`).
- **All `TODO`s must be resolved before submission.** Check with `grep -rn TODO --exclude-dir=target .`
