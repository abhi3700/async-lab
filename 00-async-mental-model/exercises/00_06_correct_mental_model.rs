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

pub fn correction(_claim: IncorrectClaim) -> CorrectModel {
	todo!("replace each incorrect claim with the corresponding accurate mental model")
}

pub fn readiness_to_poll_path() -> [WakeStep; 6] {
	todo!("order the path from socket readiness to the future's next poll")
}

fn main() {
	println!("Wake-up path: {:?}", readiness_to_poll_path());
}
