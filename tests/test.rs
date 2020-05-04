#[cfg(test)]
mod tests {
    use rustcmdpev::*;
    #[test]
    fn basic_process_explain_works() {
        let str_input = "[[{\"Plan\":{\"Alias\":\"c0\",\"Node Type\":\"Seq Scan\",\"Parallel Aware\":false,\"Plan Rows\":50,\"Plan Width\":1572,\"Relation Name\":\"coaches\",\"Startup Cost\":0.0,\"Total Cost\":10.5}}]]";
        let explains: Vec<Explain> = serde_json::from_str(str_input).unwrap();
        for explain in explains.iter() {
            println!("explain {:#?}", explain)
        }
    }
}
