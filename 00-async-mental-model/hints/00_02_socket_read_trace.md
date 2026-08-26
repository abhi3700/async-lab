Start with laziness: calling the async function only creates a future. It must become a task before a
worker can poll it. The unsuccessful read and waker registration happen before the reactor can report
useful readiness; readiness then triggers the wake-up path before the next poll.
