use crate::constants::{MAX_PLAN_DEPTH, MAX_PLAN_NODES};
use crate::structure::data::plan::Plan;
use crate::VisualizeError;

/// Summary information derived while validating a plan tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeStats {
    pub node_count: usize,
    pub max_depth: usize,
}

/// A validated plan tree with explicit root node semantics.
#[derive(Debug, Clone)]
pub struct PlanTree {
    root: Plan,
    stats: TreeStats,
}

impl PlanTree {
    /// Build a validated plan tree rooted at the supplied plan node.
    pub fn new(root: Plan) -> Result<Self, VisualizeError> {
        let mut node_count = 0;
        let stats = validate_node(&root, 0, "Plan", &mut node_count)?;
        Ok(Self { root, stats })
    }

    /// Return the validated root plan node.
    pub fn root(&self) -> &Plan {
        &self.root
    }

    /// Return aggregate tree statistics gathered during validation.
    pub fn stats(&self) -> TreeStats {
        self.stats
    }

    /// Consume the wrapper and return the validated root node.
    pub fn into_root(self) -> Plan {
        self.root
    }
}

fn validate_node(
    plan: &Plan,
    depth: usize,
    path: &str,
    node_count: &mut usize,
) -> Result<TreeStats, VisualizeError> {
    if depth > MAX_PLAN_DEPTH {
        return Err(VisualizeError::InvalidPlan(format!(
            "{path} exceeds maximum supported plan depth of {MAX_PLAN_DEPTH}"
        )));
    }
    if plan.identity.node_type.trim().is_empty() {
        return Err(VisualizeError::InvalidPlan(format!(
            "{path}.Node Type must be populated"
        )));
    }
    *node_count += 1;
    if *node_count > MAX_PLAN_NODES {
        return Err(VisualizeError::InvalidPlan(format!(
            "plan exceeds maximum supported node count of {MAX_PLAN_NODES}"
        )));
    }
    validate_non_negative(plan.estimates.startup_cost, &format!("{path}.Startup Cost"))?;
    validate_non_negative(plan.estimates.total_cost, &format!("{path}.Total Cost"))?;
    validate_non_negative(
        plan.actuals.actual_startup_time,
        &format!("{path}.Actual Startup Time"),
    )?;
    validate_non_negative(
        plan.actuals.actual_total_time,
        &format!("{path}.Actual Total Time"),
    )?;
    validate_non_negative(plan.io_timing.io_read_time, &format!("{path}.I/O Read Time"))?;
    validate_non_negative(plan.io_timing.io_write_time, &format!("{path}.I/O Write Time"))?;

    let mut stats = TreeStats {
        node_count: 1,
        max_depth: depth,
    };

    for (index, child) in plan.plans.iter().enumerate() {
        let child_stats = validate_node(
            child,
            depth + 1,
            &format!("{path}.Plans[{index}]"),
            node_count,
        )?;
        stats.node_count += child_stats.node_count;
        stats.max_depth = stats.max_depth.max(child_stats.max_depth);
    }

    Ok(stats)
}

fn validate_non_negative(value: f64, path: &str) -> Result<(), VisualizeError> {
    if !value.is_finite() {
        return Err(VisualizeError::InvalidPlan(format!(
            "{path} must be finite"
        )));
    }
    if value < 0.0 {
        return Err(VisualizeError::InvalidPlan(format!(
            "{path} must be non-negative"
        )));
    }
    Ok(())
}
