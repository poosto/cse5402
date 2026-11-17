# CSE 5402 Lab 3

- **Source of truth:** https://www.cse.wustl.edu/~cdgill/courses/cse5402/lab3/index.html. If anything here conflicts with it, the spec wins. Course coding guidelines: https://www.cse.wustl.edu/~cdgill/courses/cse5402/Guidelines.html
- **AI use policy:** `../CSE_5402_Generative_AI_Use_Policy.pdf`. Any AI help must be cited in the ReadMe (APA style: company, date, chat title, [Generative AI chat], model, link or exported transcript in `ai_transcripts/` as an appendix).
- Must build with **no warnings** on Rust 1.71.0 (CEC Linux Lab, `module add rust-2023`). Don't use newer language or std features.
- **Layout:** `lab3client/` (the Lab 2 program, moved into `src/lab3/`), plus `lab3server/` and `lab3testclient/` later. Lab 2 is in `../lab2` for checking that the client still runs as it did at the end of Lab 2.
- Every `.rs` file needs a header comment explaining its purpose, and every function needs a summary comment (Lab 1 lost points for missing these).
- **No hard coded numbers** in code, not even 0 and 1. Use descriptively named consts (Lab 2 lost 3 points for 0 and 1 in play.rs, scene_fragment.rs, player.rs).
- **Working style:** I write the majority of the code myself. Don't implement lab features unless I ask. Use AI for: answering questions, testing, cleanup/refactors I request, documentation (ReadMe, comments), and verifying that every spec requirement is met.
- **Testing should be exhaustive**, in the style of Lab 1 (`run_tests.sh` regression suite, `fuzz_test.py` randomized fuzzer, `gen_output.sh` captured output, hand-made malformed/edge-case input files). Check every spec requirement, including whinge output going to stderr and distinct exit codes (`echo $?`).
- **Writing style (comments, ReadMe, any text I submit):** Keep the English very simple and easy to understand. No em dashes. Avoid semicolons. Use short, plain sentences.
- **All `TODO`s must be resolved before submission.** Check with `grep -rn TODO --exclude-dir=target .`
