# 12 — Scheduling

**Status:** Planned

Reason about how runnable tasks share workers and where fairness can fail.

## Learning goals

- Explain cooperative scheduling, budgets, and starvation.
- Trace local queues, the global queue, LIFO slots, and work stealing.
- Separate asynchronous waiting from CPU-bound work.
- Choose among yielding, `spawn_blocking`, and a CPU-oriented pool.
