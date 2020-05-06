#[cfg(test)]
mod tests {
    use rustcmdpev::*;
    #[test]
    fn basic_process_explain_works() {
        let str_input = "[[{\"Plan\":{\"Alias\":\"c0\",\"Node Type\":\"Seq Scan\",\"Parallel Aware\":false,\"Plan Rows\":50,\"Plan Width\":1572,\"Relation Name\":\"coaches\",\"Startup Cost\":0.0,\"Total Cost\":10.5}}]]";
        let explains: Vec<Explain> = serde_json::from_str(str_input).unwrap();
        let mut explain: Explain = explains.into_iter().nth(0).unwrap();
        rustcmdpev::process_explain(&mut explain);
        println!("{:?}", explain.plan);
        assert_eq!(explain.total_cost, 0.0);
        assert_eq!(explain.max_cost, 0.0);
        assert_eq!(explain.max_rows, 0);
        assert_eq!(explain.max_duration, 0.0);
        assert_eq!(explain.execution_time, 0.0);
        assert_eq!(explain.planning_time, 0.0);
    }
}
