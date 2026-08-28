Separate creation from execution: an async function returns a lazy future. Separate tasks from
threads, and `Pending` from busy polling. For the final path, the reactor **observes**, the waker
**marks runnable**, the scheduler **selects**, and only the executor performs the next poll.
