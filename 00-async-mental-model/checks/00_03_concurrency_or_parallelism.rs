#[cfg(not(reference_solution))]
#[allow(dead_code)]
#[path = "../exercises/00_03_concurrency_or_parallelism.rs"]
mod exercise;
#[cfg(reference_solution)]
#[allow(dead_code)]
#[path = "../solutions/00_03_concurrency_or_parallelism.rs"]
mod exercise;

use exercise::{ExecutionModel, Scenario, classify};

#[test]
fn classifies_execution_models() {
	let cases = [
		(Scenario::TasksAlternateOnOneWorker, ExecutionModel::ConcurrentNotParallel),
		(Scenario::CpuWorkRunsOnTwoThreadsAtOnce, ExecutionModel::ConcurrentAndParallel),
		(Scenario::SecondOperationStartsAfterFirstFinishes, ExecutionModel::Sequential),
		(Scenario::OneWorkerOwnsManyWaitingSocketTasks, ExecutionModel::ConcurrentNotParallel),
		(Scenario::OneTaskBlocksTheOnlyWorker, ExecutionModel::ConcurrentButStarved),
	];

	for (scenario, expected) in cases {
		assert_eq!(classify(scenario), expected, "wrong execution model for {scenario:?}");
	}
}
