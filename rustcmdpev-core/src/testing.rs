//! Shared test utilities for the workspace.
//!
//! Available unconditionally to keep the surface usable from binary crates,
//! integration tests, and benches. Helpers fall in two buckets:
//!
//! 1. **Builders**: `LeafBuilder`, `with_children` — build [`Plan`] values
//!    without repeating boilerplate `..Default::default()` blocks.
//! 2. **Assertions**: `assert_contains_all`, `strip_ansi`, `node_count` —
//!    encapsulate the multi-line patterns that `#[test]` files in
//!    `tests/` keep re-implementing.
//!
//! These are intended for testing only but are exposed publicly so the
//! `rustcmdpev` binary's integration tests can reuse them via the workspace
//! dependency.

use crate::structure::data::actuals::PlanActuals;
use crate::structure::data::estimates::PlanEstimates;
use crate::structure::data::explain::Explain;
use crate::structure::data::plan::Plan;

/// Builder for a leaf [`Plan`] with the most common fields populated.
#[derive(Debug, Clone)]
pub struct LeafBuilder {
    node_type: String,
    plan_rows: u64,
    actual_rows: u64,
    total_cost: f64,
    actual_total_time: f64,
    actual_loops: u64,
    relation: Option<(String, String)>,
    index: Option<String>,
}

impl LeafBuilder {
    pub fn new(node_type: impl Into<String>) -> Self {
        Self {
            node_type: node_type.into(),
            plan_rows: 1,
            actual_rows: 1,
            total_cost: 1.0,
            actual_total_time: 1.0,
            actual_loops: 1,
            relation: None,
            index: None,
        }
    }

    pub fn rows(mut self, plan_rows: u64, actual_rows: u64) -> Self {
        self.plan_rows = plan_rows;
        self.actual_rows = actual_rows;
        self
    }

    pub fn cost(mut self, total_cost: f64) -> Self {
        self.total_cost = total_cost;
        self
    }

    pub fn time(mut self, actual_total_time: f64) -> Self {
        self.actual_total_time = actual_total_time;
        self
    }

    pub fn loops(mut self, actual_loops: u64) -> Self {
        self.actual_loops = actual_loops;
        self
    }

    pub fn relation(mut self, schema: impl Into<String>, name: impl Into<String>) -> Self {
        self.relation = Some((schema.into(), name.into()));
        self
    }

    pub fn index(mut self, index_name: impl Into<String>) -> Self {
        self.index = Some(index_name.into());
        self
    }

    pub fn build(self) -> Plan {
        let mut plan = Plan::default();
        plan.identity.node_type = self.node_type;
        if let Some((schema, name)) = self.relation {
            plan.identity.schema = schema;
            plan.identity.relation_name = name;
        }
        if let Some(index_name) = self.index {
            plan.identity.index_name = index_name;
        }
        plan.estimates = PlanEstimates {
            plan_rows: self.plan_rows,
            total_cost: self.total_cost,
            ..PlanEstimates::default()
        };
        plan.actuals = PlanActuals {
            actual_total_time: self.actual_total_time,
            actual_rows: self.actual_rows,
            actual_loops: self.actual_loops,
            ..PlanActuals::default()
        };
        plan
    }
}

/// Wrap `parent` so it has `children` attached.
pub fn with_children(mut parent: Plan, children: impl IntoIterator<Item = Plan>) -> Plan {
    parent.plans = children.into_iter().collect();
    parent
}

/// Build a single-plan [`Explain`] with optional execution/total cost.
pub fn explain_for(plan: Plan, execution_time: f64, total_cost: f64) -> Explain {
    Explain {
        plan,
        execution_time,
        total_cost,
        ..Explain::default()
    }
}

/// Strip ANSI escape sequences from `input` for resilient text assertions.
pub fn strip_ansi(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            let _ = chars.next();
            for next in chars.by_ref() {
                if ('@'..='~').contains(&next) {
                    break;
                }
            }
            continue;
        }
        out.push(ch);
    }
    out
}

/// Assert that `text` contains every needle in `needles`. Reports the *first*
/// missing needle with a snippet of the surrounding text so failures point at
/// the exact missing token instead of a wall of text.
#[track_caller]
pub fn assert_contains_all(text: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            text.contains(needle),
            "expected output to contain {needle:?}\n--- output ---\n{text}\n--- end ---"
        );
    }
}

/// Assert that `text` contains *none* of the substrings in `needles`.
#[track_caller]
pub fn assert_contains_none(text: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            !text.contains(needle),
            "expected output to NOT contain {needle:?}\n--- output ---\n{text}\n--- end ---"
        );
    }
}

/// Recursively count nodes in a plan tree.
pub fn node_count(plan: &Plan) -> usize {
    1 + plan.plans.iter().map(node_count).sum::<usize>()
}

/// Recursively collect every node in a plan tree (depth-first, parent-first).
pub fn collect_nodes(plan: &Plan) -> Vec<&Plan> {
    let mut out = Vec::new();
    collect(plan, &mut out);
    out
}

fn collect<'a>(plan: &'a Plan, out: &mut Vec<&'a Plan>) {
    out.push(plan);
    for child in &plan.plans {
        collect(child, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaf_builder_populates_typical_fields() {
        let plan = LeafBuilder::new("Seq Scan")
            .rows(100, 95)
            .cost(12.5)
            .time(0.75)
            .loops(2)
            .relation("public", "users")
            .build();
        assert_eq!(plan.identity.node_type, "Seq Scan");
        assert_eq!(plan.identity.schema, "public");
        assert_eq!(plan.identity.relation_name, "users");
        assert_eq!(plan.estimates.plan_rows, 100);
        assert_eq!(plan.actuals.actual_rows, 95);
        assert_eq!(plan.actuals.actual_loops, 2);
        assert!((plan.estimates.total_cost - 12.5).abs() < 1e-9);
    }

    #[test]
    fn with_children_attaches_subplans() {
        let parent = with_children(
            LeafBuilder::new("Hash Join").build(),
            [
                LeafBuilder::new("Seq Scan").build(),
                LeafBuilder::new("Index Scan").build(),
            ],
        );
        assert_eq!(parent.plans.len(), 2);
        assert_eq!(parent.plans[0].identity.node_type, "Seq Scan");
    }

    #[test]
    fn strip_ansi_removes_escape_sequences() {
        let input = "\u{1b}[31mhello\u{1b}[0m world";
        assert_eq!(strip_ansi(input), "hello world");
    }

    #[test]
    fn assert_contains_all_passes_when_all_present() {
        assert_contains_all("alpha bravo charlie", &["alpha", "charlie"]);
    }

    #[test]
    #[should_panic(expected = "delta")]
    fn assert_contains_all_panics_with_missing_needle() {
        assert_contains_all("alpha bravo", &["alpha", "delta"]);
    }

    #[test]
    fn node_count_walks_full_tree() {
        let plan = with_children(
            LeafBuilder::new("Hash Join").build(),
            [
                with_children(
                    LeafBuilder::new("Sort").build(),
                    [LeafBuilder::new("Seq Scan").build()],
                ),
                LeafBuilder::new("Index Scan").build(),
            ],
        );
        assert_eq!(node_count(&plan), 4);
        assert_eq!(collect_nodes(&plan).len(), 4);
    }
}
