#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
	/// Calling the async function returns a future; its body has not run yet.
	AsyncFunctionCalled,
	/// Spawning wraps the future in a task and places it on a runnable queue.
	FutureSpawnedAsTask,
	/// A worker takes the task and polls its future for the first time.
	FirstPoll,
	/// The read is not ready, so it registers a waker and returns `Pending`.
	ReadReturnsPending,
	/// The reactor reports that the socket is readable.
	ReactorObservesReadiness,
	/// The task's waker marks or enqueues the task as runnable.
	WakerMakesTaskRunnable,
	/// A worker polls the task's future again, and the read completes.
	SecondPollCompletesRead,
}

pub fn causal_order() -> [Event; 7] {
	[
		Event::AsyncFunctionCalled,
		Event::FutureSpawnedAsTask,
		Event::FirstPoll,
		Event::ReadReturnsPending,
		Event::ReactorObservesReadiness,
		Event::WakerMakesTaskRunnable,
		Event::SecondPollCompletesRead,
	]
}

fn main() {
	assert_eq!(
		causal_order(),
		[
			Event::AsyncFunctionCalled,
			Event::FutureSpawnedAsTask,
			Event::FirstPoll,
			Event::ReadReturnsPending,
			Event::ReactorObservesReadiness,
			Event::WakerMakesTaskRunnable,
			Event::SecondPollCompletesRead,
		]
	);
}
