#[cfg(test)]
mod tests {
    use rustcmdpev::*;
    #[test]
    fn test_explain_with_one_join() {
        let input = r#"
            [
              {
                "Plan": {
                  "Node Type": "Hash Join",
                  "Parallel Aware": false,
                  "Join Type": "Inner",
                  "Startup Cost": 11.12,
                  "Total Cost": 25.10,
                  "Plan Rows": 231,
                  "Plan Width": 1639,
                  "Actual Startup Time": 1.230,
                  "Actual Total Time": 2.166,
                  "Actual Rows": 231,
                  "Actual Loops": 1,
                  "Output": ["coaches.id", "coaches.first_name", "coaches.last_name", "coaches.email", "coaches.inserted_at", "coaches.updated_at", "swimmers.id", "swimmers.first_name", "swimmers.last_name", "swimmers.email", "swimmers.coach_id", "swimmers.inserted_at", "swimmers.updated_at"],
                  "Inner Unique": true,
                  "Hash Cond": "(swimmers.coach_id = coaches.id)",
                  "Shared Hit Blocks": 6,
                  "Shared Read Blocks": 12,
                  "Shared Dirtied Blocks": 0,
                  "Shared Written Blocks": 0,
                  "Local Hit Blocks": 0,
                  "Local Read Blocks": 0,
                  "Local Dirtied Blocks": 0,
                  "Local Written Blocks": 0,
                  "Temp Read Blocks": 0,
                  "Temp Written Blocks": 0,
                  "Plans": [
                    {
                      "Node Type": "Seq Scan",
                      "Parent Relationship": "Outer",
                      "Parallel Aware": false,
                      "Relation Name": "swimmers",
                      "Schema": "public",
                      "Alias": "swimmers",
                      "Startup Cost": 0.00,
                      "Total Cost": 13.31,
                      "Plan Rows": 231,
                      "Plan Width": 67,
                      "Actual Startup Time": 0.278,
                      "Actual Total Time": 1.147,
                      "Actual Rows": 231,
                      "Actual Loops": 1,
                      "Output": ["swimmers.id", "swimmers.first_name", "swimmers.last_name", "swimmers.email", "swimmers.coach_id", "swimmers.inserted_at", "swimmers.updated_at"],
                      "Shared Hit Blocks": 0,
                      "Shared Read Blocks": 11,
                      "Shared Dirtied Blocks": 0,
                      "Shared Written Blocks": 0,
                      "Local Hit Blocks": 0,
                      "Local Read Blocks": 0,
                      "Local Dirtied Blocks": 0,
                      "Local Written Blocks": 0,
                      "Temp Read Blocks": 0,
                      "Temp Written Blocks": 0
                    },
                    {
                      "Node Type": "Hash",
                      "Parent Relationship": "Inner",
                      "Parallel Aware": false,
                      "Startup Cost": 10.50,
                      "Total Cost": 10.50,
                      "Plan Rows": 50,
                      "Plan Width": 1572,
                      "Actual Startup Time": 0.583,
                      "Actual Total Time": 0.583,
                      "Actual Rows": 11,
                      "Actual Loops": 1,
                      "Output": ["coaches.id", "coaches.first_name", "coaches.last_name", "coaches.email", "coaches.inserted_at", "coaches.updated_at"],
                      "Hash Buckets": 1024,
                      "Original Hash Buckets": 1024,
                      "Hash Batches": 1,
                      "Original Hash Batches": 1,
                      "Peak Memory Usage": 10,
                      "Shared Hit Blocks": 0,
                      "Shared Read Blocks": 1,
                      "Shared Dirtied Blocks": 0,
                      "Shared Written Blocks": 0,
                      "Local Hit Blocks": 0,
                      "Local Read Blocks": 0,
                      "Local Dirtied Blocks": 0,
                      "Local Written Blocks": 0,
                      "Temp Read Blocks": 0,
                      "Temp Written Blocks": 0,
                      "Plans": [
                        {
                          "Node Type": "Seq Scan",
                          "Parent Relationship": "Outer",
                          "Parallel Aware": false,
                          "Relation Name": "coaches",
                          "Schema": "public",
                          "Alias": "coaches",
                          "Startup Cost": 0.00,
                          "Total Cost": 10.50,
                          "Plan Rows": 50,
                          "Plan Width": 1572,
                          "Actual Startup Time": 0.258,
                          "Actual Total Time": 0.260,
                          "Actual Rows": 11,
                          "Actual Loops": 1,
                          "Output": ["coaches.id", "coaches.first_name", "coaches.last_name", "coaches.email", "coaches.inserted_at", "coaches.updated_at"],
                          "Shared Hit Blocks": 0,
                          "Shared Read Blocks": 1,
                          "Shared Dirtied Blocks": 0,
                          "Shared Written Blocks": 0,
                          "Local Hit Blocks": 0,
                          "Local Read Blocks": 0,
                          "Local Dirtied Blocks": 0,
                          "Local Written Blocks": 0,
                          "Temp Read Blocks": 0,
                          "Temp Written Blocks": 0
                        }
                      ]
                    }
                  ]
                },
                "Planning Time": 8.116,
                "Triggers": [
                ],
                "Execution Time": 2.776
              }
            ]"#;
        let explain: Explain = rustcmdpev::visualize(input.to_string(), 60);
        assert_eq!(explain.total_cost, 25.1);
        assert_eq!(explain.max_cost, 13.31);
        assert_eq!(explain.max_rows, 231);
        assert_eq!(explain.max_duration, 1.147);
        assert_eq!(explain.execution_time, 2.776);
        assert_eq!(explain.planning_time, 8.116);
        assert_eq!(explain.plan.alias, "");
        assert_eq!(explain.plan.node_type, "Hash Join");
        assert_eq!(explain.plan.plan_rows, 231);
        assert_eq!(explain.plan.plan_width, 1639);
        assert_eq!(explain.plan.relation_name, "");
    }
    #[test]
    fn test_with_missing_node_type_plan_field() {
        let input = r#"[{"Plan": {"Alias": "c0"}}]"#;
        let explain: Explain = rustcmdpev::visualize(input.to_string(), 60);
        assert_eq!(explain.total_cost, 0.0);
    }
}
