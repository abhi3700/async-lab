#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskName {
	Ticker,
	SlowTask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockedResource {
	Reactor,
	RuntimeWorkerThread,
	TimerFuture,
	TickerTask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrozenTickerDiagnosis {
	pub culprit: TaskName,
	pub blocked_resource: BlockedResource,
	pub ticker_await_causes_stall: bool,
	pub more_runtime_workers_fix_design: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlowWork {
	/// The delay represents time passing while no machine instructions need to run.
	WaitingForTime,
	/// The operation calls an API that cannot run asynchronously and blocks its caller.
	UnavoidableBlockingOperation,
	/// The operation spends a long uninterrupted period executing machine instructions.
	CpuHeavyComputation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Remedy {
	RunOnBoundedCpuPool,
	AwaitNonBlockingRuntimeOperation,
	RunOnDedicatedBlockingPool,
}

pub fn diagnose_frozen_ticker() -> FrozenTickerDiagnosis {
	FrozenTickerDiagnosis {
		culprit: TaskName::SlowTask,
		blocked_resource: BlockedResource::RuntimeWorkerThread,
		ticker_await_causes_stall: false,
		more_runtime_workers_fix_design: false,
	}
}

pub fn choose_remedy(work: SlowWork) -> Remedy {
	match work {
		SlowWork::WaitingForTime => Remedy::AwaitNonBlockingRuntimeOperation,
		SlowWork::UnavoidableBlockingOperation => Remedy::RunOnDedicatedBlockingPool,
		SlowWork::CpuHeavyComputation => Remedy::RunOnBoundedCpuPool,
	}
}

fn main() {
	assert_eq!(
		diagnose_frozen_ticker(),
		FrozenTickerDiagnosis {
			culprit: TaskName::SlowTask,
			blocked_resource: BlockedResource::RuntimeWorkerThread,
			ticker_await_causes_stall: false,
			more_runtime_workers_fix_design: false,
		}
	);
	assert_eq!(choose_remedy(SlowWork::WaitingForTime), Remedy::AwaitNonBlockingRuntimeOperation);
	assert_eq!(
		choose_remedy(SlowWork::UnavoidableBlockingOperation),
		Remedy::RunOnDedicatedBlockingPool
	);
	assert_eq!(choose_remedy(SlowWork::CpuHeavyComputation), Remedy::RunOnBoundedCpuPool);
}
