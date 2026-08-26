#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncActor {
	Scheduler,
	Future,
	Runtime,
	OsThread,
	Reactor,
	Task,
	Executor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Responsibility {
	/// A value representing a computation that may eventually produce an output.
	RepresentsDeferredComputation,
	/// A spawned, schedulable unit that owns or contains a future.
	OwnsFutureAsSpawnedUnit,
	/// A kernel-managed execution resource on which machine instructions run.
	ExecutesMachineInstructions,
	/// The component that calls `poll` on runnable futures.
	PollsRunnableFutures,
	/// The policy and machinery that decide which runnable task executes, when, and on which
	/// worker.
	SelectsRunnableTaskForExecution,
	/// The component that watches timers, sockets, and other external event sources for readiness.
	ObservesExternalReadiness,
	/// The complete environment bundling task execution, scheduling, and I/O/time drivers.
	BundlesAsyncInfrastructure,
}

pub fn classify(_responsibility: Responsibility) -> AsyncActor {
	todo!("map each responsibility to the async actor that owns it")
}

fn main() {
	let actor = classify(Responsibility::RepresentsDeferredComputation);
	println!("Deferred computation is owned by: {actor:?}");
}
