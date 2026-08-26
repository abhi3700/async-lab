#[cfg(not(reference_solution))]
#[allow(dead_code)]
#[path = "../exercises/00_01_classify_actors.rs"]
mod exercise;
#[cfg(reference_solution)]
#[allow(dead_code)]
#[path = "../solutions/00_01_classify_actors.rs"]
mod exercise;

use exercise::{AsyncActor, Responsibility, classify};

#[test]
fn classifies_every_responsibility() {
	let cases = [
		(Responsibility::RepresentsDeferredComputation, AsyncActor::Future),
		(Responsibility::OwnsFutureAsSpawnedUnit, AsyncActor::Task),
		(Responsibility::ExecutesMachineInstructions, AsyncActor::OsThread),
		(Responsibility::PollsRunnableFutures, AsyncActor::Executor),
		(Responsibility::SelectsRunnableTaskForExecution, AsyncActor::Scheduler),
		(Responsibility::ObservesExternalReadiness, AsyncActor::Reactor),
		(Responsibility::BundlesAsyncInfrastructure, AsyncActor::Runtime),
	];

	for (responsibility, expected) in cases {
		assert_eq!(classify(responsibility), expected, "wrong actor for {responsibility:?}");
	}
}
