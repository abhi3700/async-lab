`Pending` releases the worker and leaves the connection task waiting. The reactor only **observes**
readiness; follow the registered waker before placing the task in the runnable queue. A queued task
does not occupy the worker until selected. Keep a long CPU-heavy parse behind a **bounded** CPU-work
boundary.
