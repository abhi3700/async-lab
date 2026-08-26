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

pub fn classify(responsibility: Responsibility) -> AsyncActor {
	match responsibility {
		Responsibility::RepresentsDeferredComputation => AsyncActor::Future,
		Responsibility::OwnsFutureAsSpawnedUnit => AsyncActor::Task,
		Responsibility::ExecutesMachineInstructions => AsyncActor::OsThread,
		Responsibility::PollsRunnableFutures => AsyncActor::Executor,
		Responsibility::SelectsRunnableTaskForExecution => AsyncActor::Scheduler,
		Responsibility::ObservesExternalReadiness => AsyncActor::Reactor,
		Responsibility::BundlesAsyncInfrastructure => AsyncActor::Runtime,
	}
}

fn main() {
	assert_eq!(classify(Responsibility::RepresentsDeferredComputation), AsyncActor::Future);
	assert_eq!(classify(Responsibility::OwnsFutureAsSpawnedUnit), AsyncActor::Task);
	assert_eq!(classify(Responsibility::ExecutesMachineInstructions), AsyncActor::OsThread);
	assert_eq!(classify(Responsibility::PollsRunnableFutures), AsyncActor::Executor);
	assert_eq!(classify(Responsibility::SelectsRunnableTaskForExecution), AsyncActor::Scheduler);
	assert_eq!(classify(Responsibility::ObservesExternalReadiness), AsyncActor::Reactor);
	assert_eq!(classify(Responsibility::BundlesAsyncInfrastructure), AsyncActor::Runtime);
}
