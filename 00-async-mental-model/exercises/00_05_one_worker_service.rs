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
pub fn transition(_state: ConnectionState, _event: ServiceEvent) -> ConnectionState {
	todo!("model waiting, wake-up, runnable queue, and scheduler transitions")
}

/// Report whether a task in this state currently occupies the single runtime worker.
pub fn occupies_runtime_worker(_state: ConnectionState) -> bool {
	todo!("only the task executing instructions should own the worker")
}

pub fn place_parsing(_work: ParsingWork) -> ParsingPlacement {
	todo!("keep long uninterrupted CPU work from monopolizing the runtime worker")
}

fn main() {
	let waiting = transition(ConnectionState::Running, ServiceEvent::ReadReturnsPending);
	println!("Connection state after Pending: {waiting:?}");
}
