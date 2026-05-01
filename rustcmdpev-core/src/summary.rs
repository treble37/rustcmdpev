//! Aggregate metrics computed once over the analyzed plan and consumed by the
//! renderer's summary block.
//!
//! Keeping this in its own module lets rendering pull the data without
//! re-walking the plan tree, and gives tests a stable surface to assert
//! against.

use crate::structure::data::buffers::PlanBuffers;
use crate::structure::data::explain::Explain;
use crate::structure::data::plan::Plan;

/// Aggregated buffer counters across every node in the plan.
#[derive(Debug, Clone, Copy, Default)]
pub struct BufferTotals {
    pub shared_hit_blocks: u64,
    pub shared_read_blocks: u64,
    pub shared_written_blocks: u64,
    pub shared_dirtied_blocks: u64,
    pub local_hit_blocks: u64,
    pub local_read_blocks: u64,
    pub local_written_blocks: u64,
    pub local_dirtied_blocks: u64,
    pub temp_read_blocks: u64,
    pub temp_written_blocks: u64,
}

impl BufferTotals {
    pub fn total(&self) -> u64 {
        self.shared_hit_blocks
            + self.shared_read_blocks
            + self.shared_written_blocks
            + self.shared_dirtied_blocks
            + self.local_hit_blocks
            + self.local_read_blocks
            + self.local_written_blocks
            + self.local_dirtied_blocks
            + self.temp_read_blocks
            + self.temp_written_blocks
    }

    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }

    fn add(&mut self, b: &PlanBuffers) {
        self.shared_hit_blocks += b.shared_hit_blocks;
        self.shared_read_blocks += b.shared_read_blocks;
        self.shared_written_blocks += b.shared_written_blocks;
        self.shared_dirtied_blocks += b.shared_dirtied_blocks;
        self.local_hit_blocks += b.local_hit_blocks;
        self.local_read_blocks += b.local_read_blocks;
        self.local_written_blocks += b.local_written_blocks;
        self.local_dirtied_blocks += b.local_dirtied_blocks;
        self.temp_read_blocks += b.temp_read_blocks;
        self.temp_written_blocks += b.temp_written_blocks;
    }
}

/// One-shot rollup used to render the header summary block.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlanSummary {
    pub total_cost: f64,
    pub planning_time: f64,
    pub execution_time: f64,
    pub total_loops: u64,
    pub total_io_read_time: f64,
    pub total_io_write_time: f64,
    pub buffers: BufferTotals,
    pub node_count: u64,
}

impl PlanSummary {
    pub fn from_explain(explain: &Explain) -> Self {
        let mut summary = Self {
            total_cost: explain.total_cost,
            planning_time: explain.planning_time,
            execution_time: explain.execution_time,
            ..Self::default()
        };
        summary.accumulate(&explain.plan);
        summary
    }

    fn accumulate(&mut self, plan: &Plan) {
        self.node_count += 1;
        self.total_loops = self.total_loops.saturating_add(plan.actuals.actual_loops);
        self.total_io_read_time += plan.io_timing.io_read_time;
        self.total_io_write_time += plan.io_timing.io_write_time;
        self.buffers.add(&plan.buffers);
        for child in &plan.plans {
            self.accumulate(child);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structure::data::actuals::PlanActuals;

    fn leaf(loops: u64, hit: u64, read: u64, io_read: f64) -> Plan {
        let mut plan = Plan::default();
        plan.identity.node_type = "Seq Scan".into();
        plan.actuals = PlanActuals {
            actual_loops: loops,
            ..PlanActuals::default()
        };
        plan.buffers.shared_hit_blocks = hit;
        plan.buffers.shared_read_blocks = read;
        plan.io_timing.io_read_time = io_read;
        plan
    }

    #[test]
    fn summary_aggregates_loops_and_buffers_recursively() {
        let mut root = leaf(3, 5, 0, 0.5);
        root.identity.node_type = "Hash Join".into();
        root.plans.push(leaf(2, 7, 1, 0.25));
        root.plans.push(leaf(4, 0, 2, 0.0));
        let explain = Explain {
            total_cost: 12.0,
            planning_time: 1.5,
            execution_time: 9.0,
            plan: root,
            ..Explain::default()
        };

        let summary = PlanSummary::from_explain(&explain);

        assert_eq!(summary.node_count, 3);
        assert_eq!(summary.total_loops, 9);
        assert_eq!(summary.buffers.shared_hit_blocks, 12);
        assert_eq!(summary.buffers.shared_read_blocks, 3);
        assert!((summary.total_io_read_time - 0.75).abs() < 1e-9);
        assert_eq!(summary.total_cost, 12.0);
        assert_eq!(summary.planning_time, 1.5);
        assert_eq!(summary.execution_time, 9.0);
    }

    #[test]
    fn summary_buffer_totals_is_empty_when_no_io() {
        let explain = Explain::default();
        let summary = PlanSummary::from_explain(&explain);
        assert!(summary.buffers.is_empty());
    }
}
