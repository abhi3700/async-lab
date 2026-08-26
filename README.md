# Async Lab

Async Lab is a code-first course for learning how asynchronous Rust works beneath the API
surface. The path starts with futures and polling, builds an executor, adds the reactor/runtime
model, and only then studies Tokio and production async systems.

The goal is to move from **using async Rust** to **reasoning about async runtimes**.

## Mental model

```text
Future
  ↓
State machine
  ↓
Poll + Context
  ↓
Waker
  ↓
Task + ready queue
  ↓
Executor + scheduler
  ↓
Reactor
  ↓
Runtime
  ↓
Tokio
```

The course keeps these distinctions explicit:

- `Future ≠ Task ≠ Thread`
- `Executor ≠ Scheduler ≠ Reactor ≠ Runtime`
- `Concurrency ≠ Parallelism`
- `.await ≠ blocking`

## Curriculum

| Chapter | Subject | Main outcome |
| --- | --- | --- |
| `00` | Async mental model | Name the moving parts and their boundaries |
| `01` | Futures | Poll a lazy computation without Tokio |
| `02` | State machines and `Pin` | Explain state held across `.await` points |
| `03` | Wakers | Connect external progress to another poll |
| `04` | Mini executor | Build tasks, a spawner, and a ready queue |
| `05` | Runtime and reactor | Separate execution from I/O readiness |
| `06` | Tokio runtime | Configure and reason about Tokio runtimes |
| `07` | Tasks and concurrency | Compose, spawn, join, and localize tasks |
| `08` | Channels | Design message-passing and actor boundaries |
| `09` | Synchronization | Choose locks, semaphores, notifications, or messages |
| `10` | Async I/O | Handle readiness, buffers, framing, and partial I/O |
| `11` | Cancellation and backpressure | Bound work and shut systems down safely |
| `12` | Scheduling | Reason about fairness, starvation, and work stealing |
| `13` | Observability and testing | Trace, measure, and deterministically test async code |
| `14` | Runtime internals | Read real Tokio runtime implementation paths |
| `15` | Performance | Benchmark scheduling, allocation, and contention |
| `16` | Capstone | Build an observable, backpressured async event runtime |

Each numbered directory is an independent workspace crate. Its `README.md` defines the chapter
boundary; code, examples, experiments, and tests live beside it as the chapter is developed.

## Lesson contract

Every lesson follows the same sequence:

1. Mental model
2. Architecture
3. Core theory
4. Minimal implementation
5. Tokio implementation, when applicable
6. Trace what happens internally
7. Common mistakes
8. Experiment
9. Exercises
10. Documentation update

Exercises progress through **Observe → Implement → Break it → Debug it → Extend it → Architecture
reasoning**.

> A lesson is not finished until its code and documentation agree.

See the [lesson template](docs/contributing/lesson-format.mdx) and
[exercise template](docs/exercises/index.mdx) before adding lesson content.

## Exercise runner

Async Lab includes a rustlings-like CLI powered by the `spinners` crate. Run it from the repository
root:

```bash
cargo run
```

It finds the next incomplete Rust exercise, checks it immediately, and watches that source file every
200 ms. After a saved change it compiles and runs the exercise's dedicated behavioral tests. A failed
check prints its compiler/test output and an optional hint; a passing check is recorded locally and
the watcher advances to the next exercise. Interactive terminals show colorful status labels and an
`Aesthetic` spinner while compilation and tests run. Hints use a prominent `💡 HINT` heading and
render Markdown emphasis as terminal styling. Redirected output stays plain, and the standard
`NO_COLOR` environment variable disables colors explicitly.

```bash
cargo run -- list
cargo run -- check 00_01_classify_actors
cargo run -- hint 00_01_classify_actors
```

Progress is stored in the ignored `.async-lab/progress.tsv` file. The fingerprint includes both the
starter source and its checker, so editing a completed exercise or changing its tests makes it
incomplete again.

The runner accepts any implementation that passes the behavioral checker. It does not compare source
text with the reference answer. Written worksheets and solutions remain separate, and auto-checked
Rust exercises follow the same rule:

```text
NN-chapter/
├── exercises/<name>.rs   # learner-owned starter
├── checks/<name>.rs      # behavioral tests
├── hints/<name>.md       # optional, shown after failure
└── solutions/<name>.rs   # reference implementation
```

## Repository layout

```text
async-lab/
├── 00-async-mental-model/
├── 01-futures/
├── 02-state-machines-pin/
├── 03-wakers/
├── 04-mini-executor/
├── 05-runtime-reactor/
├── 06-tokio-runtime/
├── 07-tasks-concurrency/
├── 08-channels/
├── 09-synchronization/
├── 10-async-io/
├── 11-cancellation-backpressure/
├── 12-scheduling/
├── 13-observability-testing/
├── 14-runtime-internals/
├── 15-performance/
├── 16-capstone/
└── docs/
```

## Working with the scaffold

The repository currently contains the curriculum scaffold, not completed lessons. Add dependencies
to individual chapter manifests only when the lesson needs them; early chapters should remain free of
Tokio so the language-level mechanics stay visible.

```bash
cargo test --workspace
cargo clippy --workspace --all-targets
cargo fmt --all --check
```

The documentation site is configured in [`docs/docs.json`](docs/docs.json). From `docs/`, run
`mint dev` when the Mintlify CLI is available.
