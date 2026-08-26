# 15 — Performance

**Status:** Planned

Measure async systems before drawing conclusions about their bottlenecks.

## Learning goals

- Measure task memory, scheduling latency, and channel throughput.
- Attribute allocations, syscalls, contention, and cache effects.
- Design reproducible benchmarks across task counts and workloads.
- Reason about batching, false sharing, atomics, and NUMA boundaries.
