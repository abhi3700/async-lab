#[cfg(not(reference_solution))]
#[allow(dead_code)]
#[path = "../exercises/00_04_debug_frozen_ticker.rs"]
mod exercise;
#[cfg(reference_solution)]
#[allow(dead_code)]
#[path = "../solutions/00_04_debug_frozen_ticker.rs"]
mod exercise;

use exercise::{
	BlockedResource, FrozenTickerDiagnosis, Remedy, SlowWork, TaskName, choose_remedy,
	diagnose_frozen_ticker,
};

#[test]
fn identifies_why_the_ticker_freezes() {
	assert_eq!(
		diagnose_frozen_ticker(),
		FrozenTickerDiagnosis {
			culprit: TaskName::SlowTask,
			blocked_resource: BlockedResource::RuntimeWorkerThread,
			ticker_await_causes_stall: false,
			more_runtime_workers_fix_design: false,
		}
	);
}

#[test]
fn chooses_a_boundary_for_each_kind_of_slow_work() {
	let cases = [
		(SlowWork::WaitingForTime, Remedy::AwaitNonBlockingRuntimeOperation),
		(SlowWork::UnavoidableBlockingOperation, Remedy::RunOnDedicatedBlockingPool),
		(SlowWork::CpuHeavyComputation, Remedy::RunOnBoundedCpuPool),
	];

	for (work, expected) in cases {
		assert_eq!(choose_remedy(work), expected, "wrong remedy for {work:?}");
	}
}
