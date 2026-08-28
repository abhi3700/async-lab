#[cfg(not(reference_solution))]
#[allow(dead_code)]
#[path = "../exercises/00_05_one_worker_service.rs"]
mod exercise;
#[cfg(reference_solution)]
#[allow(dead_code)]
#[path = "../solutions/00_05_one_worker_service.rs"]
mod exercise;

use exercise::{
	ConnectionState, ParsingPlacement, ParsingWork, ServiceEvent, occupies_runtime_worker,
	place_parsing, transition,
};

#[test]
fn moves_a_connection_through_waiting_and_wake_up() {
	let waiting = transition(ConnectionState::Running, ServiceEvent::ReadReturnsPending);
	assert_eq!(waiting, ConnectionState::WaitingForReadiness);

	let still_waiting = transition(waiting, ServiceEvent::ReactorObservesSocketReadiness);
	assert_eq!(
		still_waiting,
		ConnectionState::WaitingForReadiness,
		"the reactor observes readiness; the waker makes the task runnable"
	);

	let runnable = transition(still_waiting, ServiceEvent::WakerMarksTaskRunnable);
	assert_eq!(runnable, ConnectionState::Runnable);

	let running = transition(runnable, ServiceEvent::SchedulerSelectsTask);
	assert_eq!(running, ConnectionState::Running);
}

#[test]
fn unrelated_events_do_not_skip_required_actors() {
	assert_eq!(
		transition(ConnectionState::WaitingForReadiness, ServiceEvent::SchedulerSelectsTask),
		ConnectionState::WaitingForReadiness
	);
	assert_eq!(
		transition(ConnectionState::Runnable, ServiceEvent::ReactorObservesSocketReadiness),
		ConnectionState::Runnable
	);
}

#[test]
fn only_running_work_occupies_the_worker() {
	assert!(occupies_runtime_worker(ConnectionState::Running));
	assert!(!occupies_runtime_worker(ConnectionState::WaitingForReadiness));
	assert!(!occupies_runtime_worker(ConnectionState::Runnable));
}

#[test]
fn long_cpu_work_crosses_a_bounded_boundary() {
	assert_eq!(place_parsing(ParsingWork::ShortCooperativeChunk), ParsingPlacement::RuntimeWorker);
	assert_eq!(
		place_parsing(ParsingWork::UninterruptedCpuHeavyParse),
		ParsingPlacement::BoundedCpuPool
	);
}
