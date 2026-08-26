#[cfg(not(reference_solution))]
#[allow(dead_code)]
#[path = "../exercises/00_02_socket_read_trace.rs"]
mod exercise;
#[cfg(reference_solution)]
#[allow(dead_code)]
#[path = "../solutions/00_02_socket_read_trace.rs"]
mod exercise;

use exercise::{Event, causal_order};

#[test]
fn orders_the_socket_read_lifecycle() {
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

#[test]
fn waiting_interval_is_bounded_by_pending_and_wake_up() {
	let events = causal_order();
	let pending = events
		.iter()
		.position(|event| *event == Event::ReadReturnsPending)
		.expect("the read must become pending");
	let runnable = events
		.iter()
		.position(|event| *event == Event::WakerMakesTaskRunnable)
		.expect("the waker must make the task runnable");

	assert!(pending < runnable, "the task can only be woken after it has become pending");
}
