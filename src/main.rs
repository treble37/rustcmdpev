use rustcmdpev::*;
use std::env;
// a little smoke test...
fn write_explain_stub() {
    println!("○ Total Cost: {}\n", 4.265_f64);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: &str = &args[1];
    println!("args {}", input);
    let explains: Vec<Explain> = serde_json::from_str(input).unwrap();
    for explain in explains.iter() {
        println!("explain {:#?}", explain)
    }
    write_explain_stub()
}
