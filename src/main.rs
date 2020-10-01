use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input: Vec<String> = vec![];
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard input.");
        input.push(line.clone());
    }
    let joined_input: String = input[0..].join("\n");
    rustcmdpev::visualize(joined_input, 60);
}
