use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::vec::Vec;

type NodeType = String;
type EstimateDirection = String;
#[derive(Serialize, Deserialize)]
struct Plans<T>(Vec<T>);

//https://blog.guillaume-gomez.fr/articles/2017-03-09+Little+tour+of+multiple+iterators+implementation+in+Rust
pub struct PlansIter<'a, T: 'a> {
    inner: &'a Plans<T>,
    pos: usize
}

impl<'a, T> Iterator for PlansIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
if self.pos >= self.inner.0.len() {
            // Obviously, there isn't any more data to read so let's stop here.
            None
        } else {
            // We increment the position of our iterator.
            self.pos += 1;
            // We return the current value pointed by our iterator.
            self.inner.0.get(self.pos - 1)
        }
    }
}

impl<T> Plans<T> {
    fn iter<'a>(&'a self) -> PlansIter<'a, T> {
        PlansIter {
            inner: self,
            pos: 0,
        }
    }
}

//https://github.com/serde-rs/serde/pull/238
#[derive(Serialize, Deserialize)]
struct Plan {
    #[serde(default)]
    actual_cost: f64,
    #[serde(default)]
    actual_duration: f64,
    #[serde(default)]
    actual_loops: u64,
    #[serde(default)]
    actual_rows: u64,
    #[serde(default)]
    actual_startup_time: f64,
    #[serde(default)]
    actual_total_time: f64,
    #[serde(default)]
    alias: String,
    #[serde(default)]
    costliest: bool,
    #[serde(default)]
    cte_name: String,
    #[serde(default)]
    filter: String,
    #[serde(default)]
    group_key: Vec<String>,
    #[serde(default)]
    hash_condition: String,
    #[serde(default)]
    heap_fetches: u64,
    #[serde(default)]
    index_condition: String,
    #[serde(default)]
    index_name: String,
    #[serde(default)]
    io_read_time: f64,
    #[serde(default)]
    io_write_time: f64,
    #[serde(default)]
    join_type: String,
    #[serde(default)]
    largest: bool,
    #[serde(default)]
    local_dirtied_blocks: u64,
    #[serde(default)]
    local_hit_blocks: u64,
    #[serde(default)]
    local_read_blocks: u64,
    #[serde(default)]
    local_written_blocks: u64,
    #[serde(default)]
    node_type: NodeType,
    #[serde(default)]
    output: Vec<String>,
    #[serde(default)]
    parent_relationship: String,
    #[serde(default)]
    planner_row_estimate_direction: EstimateDirection,
    #[serde(default)]
    planner_row_estimate_factor: f64,
    #[serde(default)]
    plan_rows: u64,
    #[serde(default)]
    plan_width: u64,
    #[serde(default)]
    relation_name: String,
    #[serde(default)]
    rows_removed_by_filter: u64,
    #[serde(default)]
    rows_removed_by_index_recheck: u64,
    #[serde(default)]
    scan_direction: String,
    #[serde(default)]
    schema: String,
    #[serde(default)]
    shared_dirtied_blocks: u64,
    #[serde(default)]
    shared_hit_blocks: u64,
    #[serde(default)]
    shared_read_blocks: u64,
    #[serde(default)]
    shared_written_blocks: u64,
    #[serde(default)]
    slowest: bool,
    #[serde(default)]
    startup_cost: f64,
    #[serde(default)]
    strategy: String,
    #[serde(default)]
    temp_read_blocks: u64,
    #[serde(default)]
    temp_written_blocks: u64,
    #[serde(default)]
    total_cost: f64,
}

impl fmt::Display for Plan {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        //write!(f, "{}", self.0)
        write!(f, "{}", self)
    }
}
//https://stackoverflow.com/questions/30633177/implement-fmtdisplay-for-vect
/*impl fmt::Display for Plans {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Values:\n")?;
        for v in &self.0 {
            write!(f, "\t{}", v)?;
        }
        Ok(())
    }
}*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: &str = &args[1];
    println!("args {}", input);
    let plans: Plans<Plan> = serde_json::from_str(input).unwrap();
    for v in plans.iter() {
        println!("plan {}", v)
    } 
    //println!("input {}", plans);
}
