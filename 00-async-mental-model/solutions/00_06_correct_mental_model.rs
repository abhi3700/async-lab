#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncorrectClaim {
	/// "Calling an async function starts it in the background."
	CallingAsyncFunctionStartsInBackground,
	/// "Every task has its own thread."
	EveryTaskHasOwnThread,
	/// "After `Pending`, the executor keeps polling the future in a loop."
	PendingMeansPollContinuously,
	/// "The reactor runs the future when a socket becomes readable."
	ReactorRunsFutureDirectly,
	/// "`.await` blocks the worker until the value is ready."
	AwaitBlocksWorkerUntilReady,
	/// "A multi-thread runtime makes every task run in parallel."
	MultiThreadRuntimeMakesEveryTaskParallel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorrectModel {
	ReactorTriggersWakeUpInsteadOfPolling,
	FutureIsLazyUntilPolled,
	ParallelismDependsOnReadinessWorkersAndScheduling,
	PendingWaitsForWakeBeforeRepoll,
	AwaitYieldsWorkerWhenPending,
	TasksShareRuntimeWorkers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WakeStep {
	SchedulerSelectsTask,
	SocketBecomesReadable,
	ExecutorPollsFuture,
	WakerMarksTaskRunnable,
	ReactorObservesReadiness,
	TaskWaitsInRunnableQueue,
}

pub fn correction(claim: IncorrectClaim) -> CorrectModel {
	match claim {
		IncorrectClaim::CallingAsyncFunctionStartsInBackground =>
			CorrectModel::FutureIsLazyUntilPolled,
		IncorrectClaim::EveryTaskHasOwnThread => CorrectModel::TasksShareRuntimeWorkers,
		IncorrectClaim::PendingMeansPollContinuously =>
			CorrectModel::PendingWaitsForWakeBeforeRepoll,
		IncorrectClaim::ReactorRunsFutureDirectly =>
			CorrectModel::ReactorTriggersWakeUpInsteadOfPolling,
		IncorrectClaim::AwaitBlocksWorkerUntilReady => CorrectModel::AwaitYieldsWorkerWhenPending,
		IncorrectClaim::MultiThreadRuntimeMakesEveryTaskParallel =>
			CorrectModel::ParallelismDependsOnReadinessWorkersAndScheduling,
	}
}

pub fn readiness_to_poll_path() -> [WakeStep; 6] {
	[
		WakeStep::SocketBecomesReadable,
		WakeStep::ReactorObservesReadiness,
		WakeStep::WakerMarksTaskRunnable,
		WakeStep::TaskWaitsInRunnableQueue,
		WakeStep::SchedulerSelectsTask,
		WakeStep::ExecutorPollsFuture,
	]
}

fn main() {
	assert_eq!(
		correction(IncorrectClaim::CallingAsyncFunctionStartsInBackground),
		CorrectModel::FutureIsLazyUntilPolled
	);
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
