# Lesson 0 exercises — Async mental model

Do these in order and write down your reasoning. A one-word label is not enough: for each answer,
state **who owns the work**, **what causes progress**, and **whether an OS thread is occupied**.

Do not open [`solution.md`](solution.md) until you have attempted all six exercises.

## 0.1 — Classify the actors

Use each term exactly once:

```text
Future   Task   OS thread   Executor   Scheduler   Reactor   Runtime
```

Match the terms to these descriptions:

1. A value representing a computation that may eventually produce an output.
2. A spawned, schedulable unit that owns or contains a future.
3. A kernel-managed execution resource on which machine instructions run.
4. The component that calls `poll` on runnable futures.
5. The policy and machinery that decide which runnable task executes, when, and on which worker.
6. The component that watches timers, sockets, and other external event sources for readiness.
7. The complete environment bundling task execution, scheduling, and I/O/time drivers.

Then answer:

- Which of these is created by calling an `async fn`?
- Which of these is created when that future is spawned?
- Which of these can execute many tasks over its lifetime?

## 0.2 — Trace one socket read

An application calls an `async fn`, spawns the returned value, and eventually reads from a socket.
Put these events in causal order:

```text
A. The reactor reports that the socket is readable.
B. Calling the async function returns a future; its body has not run yet.
C. A worker polls the task's future again, and the read completes.
D. The read is not ready, so it registers a waker and returns Pending.
E. Spawning wraps the future in a task and places it on a runnable queue.
F. The task's waker marks or enqueues the task as runnable.
G. A worker takes the task and polls its future for the first time.
```

For each transition, name the responsible actor: application, spawner/runtime, scheduler, executor,
future, reactor, or waker.

Finally, identify the interval during which the task is waiting but its worker thread is free to run
other tasks.

## 0.3 — Concurrency or parallelism?

For each scenario, answer **concurrent?**, **parallel?**, and explain why.

1. Two tasks alternate at `.await` points on one current-thread runtime.
2. Two CPU-heavy functions execute at the same instant on two OS threads.
3. Operation B is not created until operation A has completely finished.
4. One worker manages 100 socket tasks, most of which are waiting for readiness.
5. Two tasks exist on one worker, but the first task blocks that worker for five seconds before the
   second task gets any execution time.

Then explain why “100 concurrent tasks” does not imply “100 threads.”

## 0.4 — Debug the frozen ticker

Assume this pseudocode runs on a current-thread async runtime:

```rust
async fn slow_task() {
	println!("slow: start");
	std::thread::sleep(Duration::from_secs(3));
	println!("slow: done");
}

async fn ticker() {
	loop {
		runtime_sleep(Duration::from_millis(100)).await;
		println!("tick");
	}
}

join(slow_task(), ticker()).await;
```

Observed output:

```text
slow: start
... no output for three seconds ...
slow: done
tick
tick
```

Answer:

1. Which task is responsible for the stall?
2. Which resource is actually blocked?
3. Why can the runtime not poll the ticker during those three seconds?
4. Does the `.await` inside `ticker` cause the problem?
5. Would using a multi-thread runtime make `slow_task` correct? Explain the distinction between
   hiding the symptom and fixing the design.
6. Give one appropriate fix when the three seconds represent waiting, and another when they represent
   unavoidable blocking or CPU-heavy work.

## 0.5 — Extend a one-worker service

Design a service with:

- one runtime worker thread;
- a timer that fires every second;
- 1,000 mostly-idle TCP connections;
- one task per connection;
- a reactor watching timers and sockets.

Draw or describe the ownership and control flow using all seven terms from Exercise 0.1.

Then answer:

1. How can one worker support 1,000 connections?
2. What moves a connection task from waiting back to runnable?
3. Where should a runnable task wait before it is polled?
4. After a socket read, one task performs 500 ms of uninterrupted CPU parsing. What happens to timer
   latency and the other connections?
5. What architectural boundary would you introduce for that parsing work?

## 0.6 — Correct the mental model

Each statement is wrong or dangerously incomplete. Rewrite it accurately in one or two sentences.

1. “Calling an async function starts it in the background.”
2. “Every task has its own thread.”
3. “When a future returns `Pending`, the executor keeps polling it in a loop.”
4. “The reactor runs the future when a socket becomes readable.”
5. “`.await` blocks until the value is ready.”
6. “A multi-thread runtime makes every task run in parallel.”

Finish by drawing this path from memory without looking back:

```text
socket becomes readable → ? → ? → runnable queue → ? → poll(future)
```

## Completion check

You are ready for Lesson 1 when you can explain the socket-read trace without using “async magic” as
a step and without assigning the same responsibility to the future, task, thread, and runtime.

