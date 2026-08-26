Focus on the verbs. A future **represents** deferred work, a task is what the runtime can **schedule**,
and an OS thread **executes instructions**. Then separate choosing runnable work from polling it, and
separate observing external readiness from the runtime that bundles all these components.
