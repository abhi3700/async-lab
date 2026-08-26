# Lesson 0 solutions — Async mental model

Read this only after attempting [`exercises.md`](exercises.md). Equivalent wording is fine; compare the
responsibility and causal chain, not just the vocabulary.

## 0.1 — Classify the actors

1. **Future** — a value representing a computation that may eventually produce an output.
2. **Task** — a spawned, schedulable unit that owns or contains a future.
3. **OS thread** — the kernel-managed resource on which instructions actually execute.
4. **Executor** — the component that polls runnable futures.
5. **Scheduler** — the policy and machinery choosing which runnable task runs, when, and where.
6. **Reactor** — the component observing readiness from external event sources.
7. **Runtime** — the environment combining execution, scheduling, and drivers such as I/O and time.

Calling an `async fn` creates a **future**. Spawning it creates a **task** around that future. An **OS
thread** can execute parts of many different tasks over its lifetime.

Implementations sometimes combine executor and scheduler machinery, but the conceptual
responsibilities remain different: polling runnable work versus choosing the runnable work to poll.

## 0.2 — Trace one socket read

The order is:

```text
B → E → G → D → A → F → C
```

The causal trace is:

1. **Application:** calls the async function, producing a lazy future (`B`).
2. **Spawner/runtime:** wraps that future in a task and submits it (`E`).
3. **Scheduler then executor:** selects the runnable task; a worker polls its future (`G`).
4. **Future:** attempts the read, registers the current task's waker with the I/O machinery, and
   returns `Pending` (`D`).
5. **Reactor:** observes socket readability (`A`).
6. **Waker:** tells the runtime that the associated task can make progress, making it runnable (`F`).
7. **Scheduler then executor:** selects it again and polls; the read now completes (`C`).

Between `D` and `F`, the task is waiting and does not need an OS thread. Its previous worker is free to
poll other runnable tasks.

## 0.3 — Concurrency or parallelism?

1. **Concurrent: yes. Parallel: no.** Both tasks are in progress during overlapping lifetimes, but
   only one executes instructions at any instant on the single worker.
2. **Concurrent: yes. Parallel: yes.** Their lifetimes overlap and two threads execute them at the
   same instant.
3. **Concurrent: no. Parallel: no.** The operations neither overlap in lifetime nor execute
   simultaneously.
4. **Concurrent: yes. Parallel: no.** The worker interleaves progress among many tasks; most tasks
   consume no thread while waiting.
5. **Concurrent in lifetime, but not making fair progress; parallel: no.** Both tasks exist, but the
   first prevents the only worker from running the second. Concurrency does not guarantee fairness.

One thread can advance many concurrent tasks because each task relinquishes the worker when it cannot
make progress. The thread is reused for whichever task is runnable next.

## 0.4 — Debug the frozen ticker

1. `slow_task` causes the stall by calling a blocking function while being polled.
2. The runtime's only **OS worker thread** is blocked inside `std::thread::sleep`.
3. Polling is ordinary synchronous execution. The executor cannot regain control and poll another
   task until `slow_task` returns from `sleep` and yields from its poll.
4. No. The ticker's `.await` lets it yield while its timer is pending. That is the behavior that
   should allow other tasks to run.
5. More workers may let the ticker run elsewhere, hiding this particular symptom. The blocking call
   still occupies an async worker and can exhaust a larger pool under load, so the design is still
   wrong.
6. If the duration represents **waiting**, use the runtime's non-blocking async timer and await it. If
   it represents an unavoidable blocking API or CPU-heavy work, move it to a dedicated blocking or
   CPU-oriented pool and await only the result.

The key distinction is that `.await` can return control to the executor; `std::thread::sleep` cannot.

## 0.5 — Extend a one-worker service

A valid architecture is:

```text
Runtime
├── Reactor
│   ├── watches timer readiness
│   └── watches socket readiness
├── Scheduler / runnable queue
└── Executor on one OS worker thread
    └── polls each runnable Task
        └── owns a connection Future
```

1. Most connection futures are `Pending`, so they do not occupy the worker. The worker is reused to
   poll whichever tasks are runnable.
2. Socket readiness reaches the registered **waker**, which marks or enqueues the associated task as
   runnable.
3. It waits in the scheduler's runnable/ready queue, not in a busy polling loop.
4. The uninterrupted parse monopolizes the only worker for 500 ms. Timer delivery and every other
   connection task can be delayed by roughly that time or longer if such work accumulates.
5. Send parsing work to a bounded CPU-oriented worker pool. Await its result from the connection task,
   and bound submissions so offloading does not merely create an unbounded queue elsewhere.

## 0.6 — Correct the mental model

1. Calling an async function returns a lazy future. Its body begins executing only when something
   polls that future.
2. A task is a schedulable unit, not a thread. Many tasks may be interleaved on one thread, and a
   movable task may be polled by different runtime workers at different times.
3. `Pending` means the future cannot currently progress. It must arrange for the task's waker to be
   notified before another useful poll; continuous polling would be a wasteful busy loop.
4. The reactor observes socket readiness and triggers the registered wake-up path. The scheduler and
   executor later arrange and perform the future's next poll.
5. `.await` polls the awaited future and, when it is pending, suspends the surrounding future so the
   worker can run other tasks. Synchronous work performed between await points can still block a
   worker.
6. A multi-thread runtime permits parallel execution of runnable tasks, but does not guarantee that
   every task runs simultaneously. Readiness, task behavior, worker count, and scheduling determine
   actual execution.

The completed path is:

```text
socket becomes readable
        ↓
reactor observes readiness
        ↓
waker marks/enqueues the task
        ↓
runnable queue
        ↓
scheduler selects the task and executor dispatches it on a worker
        ↓
poll(future)
```

