use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: &str = &args[1];
    rustcmdpev::visualize(input.to_string(), 60);
}
