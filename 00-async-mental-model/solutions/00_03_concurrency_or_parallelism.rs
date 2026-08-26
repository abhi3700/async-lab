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

pub fn classify(scenario: Scenario) -> ExecutionModel {
	match scenario {
		Scenario::TasksAlternateOnOneWorker => ExecutionModel::ConcurrentNotParallel,
		Scenario::CpuWorkRunsOnTwoThreadsAtOnce => ExecutionModel::ConcurrentAndParallel,
		Scenario::SecondOperationStartsAfterFirstFinishes => ExecutionModel::Sequential,
		Scenario::OneWorkerOwnsManyWaitingSocketTasks => ExecutionModel::ConcurrentNotParallel,
		Scenario::OneTaskBlocksTheOnlyWorker => ExecutionModel::ConcurrentButStarved,
	}
}

fn main() {
	assert_eq!(
		classify(Scenario::TasksAlternateOnOneWorker),
		ExecutionModel::ConcurrentNotParallel
	);
	assert_eq!(
		classify(Scenario::CpuWorkRunsOnTwoThreadsAtOnce),
		ExecutionModel::ConcurrentAndParallel
	);
	assert_eq!(
		classify(Scenario::SecondOperationStartsAfterFirstFinishes),
		ExecutionModel::Sequential
	);
	assert_eq!(
		classify(Scenario::OneWorkerOwnsManyWaitingSocketTasks),
		ExecutionModel::ConcurrentNotParallel
	);
	assert_eq!(
		classify(Scenario::OneTaskBlocksTheOnlyWorker),
		ExecutionModel::ConcurrentButStarved
	);
}
