#[cfg(test)]
mod tests {
    use rustcmdpev::*;
    #[test]
    fn basic_process_explain_works() {
        //need to make struct fields serialize from words with space in names
        let str_input = "[{\"Plan\":{\"Alias\":\"c0\",\"Node Type\":\"Seq Scan\",\"Parallel Aware\":false,\"Plan Rows\":50,\"Plan Width\":1572,\"Relation Name\":\"coaches\",\"Startup Cost\":0.0,\"Total Cost\":10.5}}]";
        let explains: Vec<Explain> = serde_json::from_str(str_input).unwrap();
        let mut explain: Explain = explains.into_iter().nth(0).unwrap();
        explain = rustcmdpev::process_all(explain);
        assert_eq!(explain.total_cost, 10.5);
        assert_eq!(explain.max_cost, 10.5);
        assert_eq!(explain.max_rows, 0);
        assert_eq!(explain.max_duration, 0.0);
        assert_eq!(explain.execution_time, 0.0);
        assert_eq!(explain.planning_time, 0.0);
        assert_eq!(explain.plan.alias, "c0");
        assert_eq!(explain.plan.node_type, "Seq Scan");
        assert_eq!(explain.plan.plan_rows, 50);
        assert_eq!(explain.plan.plan_width, 1572);
        assert_eq!(explain.plan.relation_name, "Coaches");
    }
}
