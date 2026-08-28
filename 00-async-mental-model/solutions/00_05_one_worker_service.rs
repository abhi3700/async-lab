#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
	Runnable,
	WaitingForReadiness,
	Running,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceEvent {
	/// The running connection future cannot read yet, registers its waker, and returns `Pending`.
	ReadReturnsPending,
	/// The reactor detects that the connection's socket is now readable.
	ReactorObservesSocketReadiness,
	/// The registered waker marks or enqueues the connection task as runnable.
	WakerMarksTaskRunnable,
	/// The scheduler chooses the runnable connection task for execution.
	SchedulerSelectsTask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsingWork {
	/// A small cooperative parsing step that returns control promptly.
	ShortCooperativeChunk,
	/// An uninterrupted 500 ms CPU-heavy parse after a socket read.
	UninterruptedCpuHeavyParse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsingPlacement {
	BoundedCpuPool,
	RuntimeWorker,
}

/// Apply one event. An event that is not valid for the current state must leave it unchanged.
pub fn transition(state: ConnectionState, event: ServiceEvent) -> ConnectionState {
	match (state, event) {
		(ConnectionState::Running, ServiceEvent::ReadReturnsPending) =>
			ConnectionState::WaitingForReadiness,
		(ConnectionState::WaitingForReadiness, ServiceEvent::WakerMarksTaskRunnable) =>
			ConnectionState::Runnable,
		(ConnectionState::Runnable, ServiceEvent::SchedulerSelectsTask) => ConnectionState::Running,
		_ => state,
	}
}

/// Report whether a task in this state currently occupies the single runtime worker.
pub fn occupies_runtime_worker(state: ConnectionState) -> bool {
	matches!(state, ConnectionState::Running)
}

pub fn place_parsing(work: ParsingWork) -> ParsingPlacement {
	match work {
		ParsingWork::ShortCooperativeChunk => ParsingPlacement::RuntimeWorker,
		ParsingWork::UninterruptedCpuHeavyParse => ParsingPlacement::BoundedCpuPool,
	}
}

fn main() {
	assert_eq!(
		transition(ConnectionState::Running, ServiceEvent::ReadReturnsPending),
		ConnectionState::WaitingForReadiness
	);
	assert_eq!(
		transition(
			ConnectionState::WaitingForReadiness,
			ServiceEvent::ReactorObservesSocketReadiness
		),
		ConnectionState::WaitingForReadiness
	);
	assert_eq!(
		transition(ConnectionState::WaitingForReadiness, ServiceEvent::WakerMarksTaskRunnable),
		ConnectionState::Runnable
	);
	assert_eq!(
		transition(ConnectionState::Runnable, ServiceEvent::SchedulerSelectsTask),
		ConnectionState::Running
	);
}
