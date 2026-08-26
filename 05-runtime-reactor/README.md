# 05 — Runtime and reactor

**Status:** Planned

Bridge language-level futures and a full async runtime.

## Learning goals

- Separate an executor from an I/O reactor.
- Trace timer and socket readiness back to a waker.
- Identify the scheduler, I/O driver, and timer driver inside a runtime.
- Relate platform event systems to portable async APIs.
