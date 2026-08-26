# 09 — Synchronization

**Status:** Planned

Choose synchronization primitives based on what is protected and how long it is held.

## Learning goals

- Compare standard and async mutexes.
- Use read/write locks, semaphores, notifications, and barriers.
- Avoid holding inappropriate guards across `.await` points.
- Recognize when message passing is the clearer ownership model.
