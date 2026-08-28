#[cfg(not(reference_solution))]
#[allow(dead_code)]
#[path = "../exercises/00_06_correct_mental_model.rs"]
mod exercise;
#[cfg(reference_solution)]
#[allow(dead_code)]
#[path = "../solutions/00_06_correct_mental_model.rs"]
mod exercise;

use exercise::{CorrectModel, IncorrectClaim, WakeStep, correction, readiness_to_poll_path};

#[test]
fn replaces_every_incorrect_claim() {
	let cases = [
		(
			IncorrectClaim::CallingAsyncFunctionStartsInBackground,
			CorrectModel::FutureIsLazyUntilPolled,
		),
		(IncorrectClaim::EveryTaskHasOwnThread, CorrectModel::TasksShareRuntimeWorkers),
		(
			IncorrectClaim::PendingMeansPollContinuously,
			CorrectModel::PendingWaitsForWakeBeforeRepoll,
		),
		(
			IncorrectClaim::ReactorRunsFutureDirectly,
			CorrectModel::ReactorTriggersWakeUpInsteadOfPolling,
		),
		(IncorrectClaim::AwaitBlocksWorkerUntilReady, CorrectModel::AwaitYieldsWorkerWhenPending),
		(
			IncorrectClaim::MultiThreadRuntimeMakesEveryTaskParallel,
			CorrectModel::ParallelismDependsOnReadinessWorkersAndScheduling,
		),
	];

	for (claim, expected) in cases {
		assert_eq!(correction(claim), expected, "wrong correction for {claim:?}");
	}
}

#[test]
fn traces_readiness_to_the_next_poll() {
	assert_eq!(
		readiness_to_poll_path(),
		[
			WakeStep::SocketBecomesReadable,
			WakeStep::ReactorObservesReadiness,
			WakeStep::WakerMarksTaskRunnable,
			WakeStep::TaskWaitsInRunnableQueue,
			WakeStep::SchedulerSelectsTask,
			WakeStep::ExecutorPollsFuture,
		]
	);
}
