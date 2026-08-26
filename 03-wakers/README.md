# 03 — Wakers

**Status:** Planned

Connect a pending future to the event that lets it make progress.

## Learning goals

- Explain how a `Waker` schedules another poll.
- Build a timer future and count its polls.
- Handle a waker that changes between polls.
- Diagnose a future that hangs because it never wakes its task.
