# Async Lab repository instructions

These instructions apply to every agent and every file in this repository unless a deeper
`AGENTS.md` explicitly narrows them.

## Mission and curriculum

- Async Lab teaches the machinery beneath async Rust, not only Tokio APIs.
- Preserve the 17-chapter order from `00-async-mental-model` through `16-capstone`.
- Preserve the conceptual chain: `Future → State machine → Poll → Context → Waker → Task → Ready
  queue → Executor → Scheduler → Reactor → Runtime → Tokio`.
- Chapters 00–05 should expose language/runtime mechanics without using Tokio to hide the mechanism.
  Tokio is introduced in Chapter 06. A narrow comparison may be added earlier only when explicitly
  identified as such.
- Never claim a planned lesson, exercise, runtime behavior, or visual result is complete when it has
  not been implemented and verified.

## Lesson contract

Every completed lesson should contain, where applicable:

1. Mental model
2. Architecture
3. Core theory
4. Minimal implementation
5. Tokio implementation
6. Internal execution trace
7. Common mistakes
8. Experiment
9. Exercises
10. Documentation update

Exercise sets should progress through Observe, Implement, Break it, Debug it, Extend it, and
Architecture reasoning. Not every concept needs to be machine-graded; preserve written reasoning
when explanation is the learning objective.

## Exercise and solution separation

- Preserve each chapter's written `exercises.md` and `solution.md` as separate files.
- Never reveal, paste, or copy a reference solution into a learner starter.
- Treat `NN-*/exercises/*.rs` as learner-owned files. If they contain an attempt, do not overwrite or
  reset it unless the user explicitly requests that exact action.
- Auto-checked exercises use matching basenames:

```text
NN-chapter/
├── exercises/<name>.rs
├── checks/<name>.rs
├── hints/<name>.md       # optional
└── solutions/<name>.rs
```

- Lexical chapter path and filename order defines CLI progression. Use names such as
  `01_02_descriptive_slug.rs`.
- A Rust starter, checker, and reference solution are required for every discovered auto-checked
  exercise. The CLI deliberately fails discovery when either the checker or solution is missing.
- Register every learner starter as an explicit `[[bin]]` target in its chapter's `Cargo.toml`, with
  `test = false` and `bench = false`. This keeps rust-analyzer attached to the file while the custom
  CLI remains responsible for grading it.
- Check semantic behavior, invariants, compiler properties, or observable output. Never require the
  learner's source text to equal the reference source.
- Keep hints directional. Explain the next useful question or contract without giving away the full
  implementation.
- A reference solution must compile and satisfy the same checker before the exercise is considered
  complete.

## Exercise runner contract

- `cargo run` must start the watcher for the next incomplete Rust exercise.
- Keep `cargo run -- list`, `cargo run -- check [NAME]`, and `cargo run -- hint [NAME]` working.
- The runner lives in `tools/async-lab-cli` and uses the `spinners` crate for terminal animation. Keep
  the remaining dependency surface small unless another dependency solves a demonstrated problem.
- The watcher must check once on startup, react promptly to saved content changes, print available
  hints after failures, persist passing progress, and advance automatically.
- Preserve terminal-aware colors and the `spinners`-backed compilation/test loader. Redirected output
  must remain free of animation, and the standard `NO_COLOR` environment variable must disable ANSI
  colors.
- Keep hints visually distinct with an uppercase `💡 HINT` heading. Render supported Markdown
  emphasis as terminal styling instead of printing raw markers.
- Progress belongs in ignored `.async-lab/`; never commit, delete, or reset a learner's progress as a
  side effect of unrelated work.
- A completion fingerprint must include the learner source and checker so edited solutions or updated
  requirements are rechecked.

## Code and documentation synchronization

- Numbered chapter crates contain code, experiments, checks, and reference solutions.
- `docs/` mirrors the curriculum for Mintlify. Update the relevant MDX page and `docs/docs.json`
  navigation whenever a meaningful lesson or exercise surface is added.
- Mark incomplete content as `Planned` or otherwise state its exact status.
- A lesson is not finished until its code and documentation agree.
- Keep early chapters dependency-light. Add dependencies to the chapter that needs them instead of
  making every crate inherit a large shared set.

## Validation

Run checks proportional to the change, and use the full set before handing off structural or runner
changes:

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run -- list
```

Also format standalone exercise, checker, and solution files with `rustfmt --edition 2024`. For each
new auto-checked exercise, verify that:

- the starter fails for the intended reason and prints the intended hint;
- the reference solution compiles and demonstrates the expected behavior;
- the dedicated checker accepts the reference behavior;
- unrelated exercises remain discoverable in the correct order.

Lesson 0 checkers support `rustc --cfg reference_solution` for this maintenance validation. Preserve
an equivalent non-learner path for future checks so the reference can be tested without overwriting
the starter.

Run `mint validate` from `docs/` after documentation or navigation changes. Mintlify validation is
not a substitute for browser-based visual verification; report those separately.
