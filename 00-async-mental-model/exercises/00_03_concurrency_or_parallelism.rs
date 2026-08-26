#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scenario {
	/// Two tasks alternate at `.await` points on one current-thread runtime.
	TasksAlternateOnOneWorker,
	/// Two CPU-heavy functions execute at the same instant on two OS threads.
	CpuWorkRunsOnTwoThreadsAtOnce,
	/// Operation B is not created until operation A has completely finished.
	SecondOperationStartsAfterFirstFinishes,
	/// One worker manages 100 socket tasks, most of which are waiting for readiness.
	OneWorkerOwnsManyWaitingSocketTasks,
	/// One task blocks the only worker before the second task gets any execution time.
	OneTaskBlocksTheOnlyWorker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionModel {
	/// Operations neither overlap in lifetime nor execute simultaneously.
	Sequential,
	/// Operations overlap in lifetime, but only one executes at any instant.
	ConcurrentNotParallel,
	/// Operations overlap in lifetime and execute simultaneously on different threads.
	ConcurrentAndParallel,
	/// Operations coexist, but blocking prevents another task from making progress.
	ConcurrentButStarved,
}

pub fn classify(_scenario: Scenario) -> ExecutionModel {
	todo!("classify the lifetime overlap, simultaneous execution, and starvation in each scenario")
}

fn main() {
	let model = classify(Scenario::TasksAlternateOnOneWorker);
	println!("Alternating tasks are: {model:?}");
}
