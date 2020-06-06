#[cfg(test)]
mod tests {
    use rustcmdpev::*;
    #[test]
    fn test1_basic_process_explain() {
        /*let input = "[{\"Plan\":{\"Alias\":\"c0\",\"Node Type\":\"Seq Scan\",\"Parallel Aware\":false,\"Plan Rows\":50,\"Plan Width\":1572,\"Relation Name\":\"coaches\",\"Startup Cost\":0.0,\"Total Cost\":10.5}}]";
        let explains: Vec<Explain> = serde_json::from_str(input).unwrap();
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
        assert_eq!(explain.plan.relation_name, "coaches");*/
    }
    #[test]
    fn test2_process_explain_with_one_nested_plan() {
        /*let input = "[{\"Plan\":{\"Alias\":\"c0\",\"Node Type\":\"Seq Scan\",\"Parallel Aware\":false,\"Plan Rows\":50,\"Plan Width\":1572,\"Relation Name\":\"coaches\",\"Startup Cost\":0.0,\"Total Cost\":10.5, \"Plans\":[{\"Plan\":{\"Alias\":\"c1\", \"Actual Cost\":500.0, \"Actual Duration\":300.0, \"Actual Rows\":200, \"Node Type\":\"Seq Scan\",\"Parallel Aware\":false,\"Plan Rows\":50,\"Plan Width\":1572,\"Relation Name\":\"coaches\",\"Startup Cost\":0.0,\"Total Cost\":5.5}}]}}]";
        let explains: Vec<Explain> = serde_json::from_str(input).unwrap();
        let mut explain: Explain = explains.into_iter().nth(0).unwrap();
        explain = rustcmdpev::process_all(explain);
        println!("explain plan {:?}", explain.plan);
        assert_eq!(explain.total_cost, 10.5);
        assert_eq!(explain.max_cost, 500.0);
        assert_eq!(explain.max_rows, 200);
        assert_eq!(explain.max_duration, 300.0);
        assert_eq!(explain.execution_time, 0.0);
        assert_eq!(explain.planning_time, 0.0);
        assert_eq!(explain.plan.alias, "c0");
        assert_eq!(explain.plan.node_type, "Seq Scan");
        assert_eq!(explain.plan.plan_rows, 50);
        assert_eq!(explain.plan.plan_width, 1572);
        assert_eq!(explain.plan.relation_name, "coaches");*/
    }
    #[test]
    fn test_jesus_wheel() {
        let str_input = r#"[{"Wheel Diameter":5.2, "Car":{"Dealer Price": 500.0, "Cars":[{"Car":{"Dealer Price":1500.0, "Cars":[{"Car":{"Dealer Price":1400.0}}]}}]}}]"#;
        let cars: Vec<Wheel> = serde_json::from_str(str_input).unwrap();
        println!("cars {:?}", cars);
    }
}
