use colored::*;
use phf::phf_map;
use serde::{Deserialize, Serialize};
use std::fmt;

const UNDER: &str = "Under";
const OVER: &str = "Over";
const CTE_SCAN: &str = "CTE Scan";

type NodeType = String;
type EstimateDirection = String;

static DESCRIPTIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Append" =>          "Used in a UNION to merge multiple record sets by appending them together.",
    "Limit" =>           "Returns a specified number of rows from a record set.",
    "Sort" =>            "Sorts a record set based on the specified sort key.",
    "Nested Loop" =>      "Merges two record sets by looping through every record in the first set and trying to find a match in the second set. All matching records are returned.",
    "Merge Join" =>       "Merges two record sets by first sorting them on a join key.",
    "Hash" =>            "Generates a hash table from the records in the input recordset. Hash is used by Hash Join.",
    "Hash Join" =>        "Joins to record sets by hashing one of them (using a Hash Scan).",
    "Aggregate" =>       "Groups records together based on a GROUP BY or aggregate function (e.g. sum()).",
    "Hashaggregate" =>   "Groups records together based on a GROUP BY or aggregate function (e.g. sum()). Hash Aggregate uses a hash to first organize the records by a key.",
    "Seq Scan" =>    "Finds relevant records by sequentially scanning the input record set. When reading from a table, Seq Scans (unlike Index Scans) perform a single read operation (only the table is read).",
    "Index Scan" =>       "Finds relevant records based on an Index. Index Scans perform 2 read operations: one to read the index and another to read the actual value from the table.",
    "Index Only Scan" =>   "Finds relevant records based on an Index. Index Only Scans perform a single read operation from the index and do not read from the corresponding table.",
    "Bitmap Heap Scan" =>  "Searches through the pages returned by the Bitmap Index Scan for relevant rows.",
    "Bitmap Index Scan" => "Uses a Bitmap Index (index which uses 1 bit per page) to find all relevant pages. Results of this node are fed to the Bitmap Heap Scan.",
    "CTE Scan" =>         "Performs a sequential scan of Common Table Expression (CTE) query results. Note that results of a CTE are materialized (calculated and temporarily stored).",
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Explain {
    //TODO: add Triggers back, add default for plan?
    #[serde(default, rename(deserialize = "Plan"))]
    pub plan: Plan,
    #[serde(default, rename(deserialize = "Planning Time"))]
    pub planning_time: f64,
    #[serde(default, rename(deserialize = "Execution Time"))]
    pub execution_time: f64,
    #[serde(default, rename(deserialize = "Total Cost"))]
    pub total_cost: f64,
    #[serde(default, rename(deserialize = "Max Rows"))]
    pub max_rows: u64,
    #[serde(default, rename(deserialize = "Max Cost"))]
    pub max_cost: f64,
    #[serde(default, rename(deserialize = "Max Duration"))]
    pub max_duration: f64,
}

//https://github.com/serde-rs/serde/pull/238
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plan {
    #[serde(default, rename(deserialize = "Actual Cost"))]
    pub actual_cost: f64,
    #[serde(default, rename(deserialize = "Actual Duration"))]
    pub actual_duration: f64,
    #[serde(default, rename(deserialize = "Actual Loops"))]
    pub actual_loops: u64,
    #[serde(default, rename(deserialize = "Actual Rows"))]
    pub actual_rows: u64,
    #[serde(default, rename(deserialize = "Actual Startup Time"))]
    pub actual_startup_time: f64,
    #[serde(default, rename(deserialize = "Actual Total Time"))]
    pub actual_total_time: f64,
    #[serde(default, rename(deserialize = "Alias"))]
    pub alias: String,
    #[serde(default, rename(deserialize = "Costliest"))]
    pub costliest: bool,
    #[serde(default, rename(deserialize = "CTE Name"))]
    pub cte_name: String,
    #[serde(default, rename(deserialize = "Filter"))]
    pub filter: String,
    #[serde(default, rename(deserialize = "Group Key"))]
    pub group_key: Vec<String>,
    #[serde(default, rename(deserialize = "Hash Cond"))]
    pub hash_condition: String,
    #[serde(default, rename(deserialize = "Heap Fetches"))]
    pub heap_fetches: u64,
    #[serde(default, rename(deserialize = "Index Cond"))]
    pub index_condition: String,
    #[serde(default, rename(deserialize = "Index Name"))]
    pub index_name: String,
    #[serde(default, rename(deserialize = "I/O Read Time"))]
    pub io_read_time: f64,
    #[serde(default, rename(deserialize = "I/O Write Time"))]
    pub io_write_time: f64,
    #[serde(default, rename(deserialize = "Join Type"))]
    pub join_type: String,
    #[serde(default)]
    pub largest: bool,
    #[serde(default, rename(deserialize = "Local Dirtied Blocks"))]
    pub local_dirtied_blocks: u64,
    #[serde(default, rename(deserialize = "Local Hit Blocks"))]
    pub local_hit_blocks: u64,
    #[serde(default, rename(deserialize = "Local Read Blocks"))]
    pub local_read_blocks: u64,
    #[serde(default, rename(deserialize = "Local Written Blocks"))]
    pub local_written_blocks: u64,
    #[serde(default, rename(deserialize = "Node Type"))]
    pub node_type: NodeType,
    #[serde(default, rename(deserialize = "Output"))]
    pub output: Vec<String>,
    #[serde(default, rename(deserialize = "Parent Relationship"))]
    pub parent_relationship: String,
    #[serde(default)]
    pub planner_row_estimate_direction: EstimateDirection,
    #[serde(default)]
    pub planner_row_estimate_factor: f64,
    #[serde(default, rename(deserialize = "Plan Rows"))]
    pub plan_rows: u64,
    #[serde(default, rename(deserialize = "Plan Width"))]
    pub plan_width: u64,
    #[serde(default, rename(deserialize = "Relation Name"))]
    pub relation_name: String,
    #[serde(default, rename(deserialize = "Rows Removed By Filter"))]
    pub rows_removed_by_filter: u64,
    #[serde(default, rename(deserialize = "Rows Removed By Index Recheck"))]
    pub rows_removed_by_index_recheck: u64,
    #[serde(default, rename(deserialize = "Scan Direction"))]
    pub scan_direction: String,
    #[serde(default, rename(deserialize = "Schema"))]
    pub schema: String,
    #[serde(default, rename(deserialize = "Shared Dirtied Blocks"))]
    pub shared_dirtied_blocks: u64,
    #[serde(default, rename(deserialize = "Shared Hit Blocks"))]
    pub shared_hit_blocks: u64,
    #[serde(default, rename(deserialize = "Shared Read Blocks"))]
    pub shared_read_blocks: u64,
    #[serde(default, rename(deserialize = "Shared Written Blocks"))]
    pub shared_written_blocks: u64,
    #[serde(default)]
    pub slowest: bool,
    #[serde(default, rename(deserialize = "Startup Cost"))]
    pub startup_cost: f64,
    #[serde(default, rename(deserialize = "Strategy"))]
    pub strategy: String,
    #[serde(default, rename(deserialize = "Temp Read Blocks"))]
    pub temp_read_blocks: u64,
    #[serde(default, rename(deserialize = "Temp Written Blocks"))]
    pub temp_written_blocks: u64,
    #[serde(default, rename(deserialize = "Total Cost"))]
    pub total_cost: f64,
    #[serde(default, rename(deserialize = "Plans"))]
    pub plans: Vec<Plan>,
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

impl Default for Plan {
    fn default() -> Plan {
        Plan {
            actual_cost: 0.0,
            actual_duration: 0.0,
            actual_loops: 0,
            actual_rows: 0,
            actual_startup_time: 0.0,
            actual_total_time: 0.0,
            alias: String::from(""),
            costliest: false,
            cte_name: String::from(""),
            filter: String::from(""),
            group_key: Vec::new(),
            hash_condition: String::from(""),
            heap_fetches: 0,
            index_condition: String::from(""),
            index_name: String::from(""),
            io_read_time: 0.0,
            io_write_time: 0.0,
            join_type: String::from(""),
            largest: false,
            local_dirtied_blocks: 0,
            local_hit_blocks: 0,
            local_read_blocks: 0,
            local_written_blocks: 0,
            node_type: String::from(""),
            output: Vec::new(),
            parent_relationship: String::from(""),
            planner_row_estimate_direction: String::from(""),
            planner_row_estimate_factor: 0.0,
            plan_rows: 0,
            plan_width: 0,
            relation_name: String::from(""),
            rows_removed_by_filter: 0,
            rows_removed_by_index_recheck: 0,
            scan_direction: String::from(""),
            schema: String::from(""),
            shared_dirtied_blocks: 0,
            shared_hit_blocks: 0,
            shared_read_blocks: 0,
            shared_written_blocks: 0,
            slowest: false,
            startup_cost: 0.0,
            strategy: String::from(""),
            temp_read_blocks: 0,
            temp_written_blocks: 0,
            total_cost: 0.0,
            plans: Vec::new(),
        }
    }
}

impl Default for Explain {
    fn default() -> Explain {
        Explain {
            plan: Plan {
                ..Default::default()
            },
            planning_time: 0.0,
            execution_time: 0.0,
            total_cost: 0.0,
            max_rows: 0,
            max_cost: 0.0,
            max_duration: 0.0,
        }
    }
}

// https://www.forrestthewoods.com/blog/should-small-rust-structs-be-passed-by-copy-or-by-borrow/
pub fn calculate_planner_estimate(plan: Plan) -> Plan {
    let mut new_plan: Plan = plan;
    new_plan.planner_row_estimate_factor = 0.0;

    if new_plan.plan_rows == new_plan.actual_rows {
        return new_plan;
    }

    new_plan.planner_row_estimate_direction = UNDER.to_string();
    if new_plan.plan_rows != 0 {
        new_plan.planner_row_estimate_factor =
            new_plan.actual_rows as f64 / new_plan.plan_rows as f64;
    }

    if new_plan.planner_row_estimate_factor < 1.0 {
        new_plan.planner_row_estimate_factor = 0.0;
        new_plan.planner_row_estimate_direction = OVER.to_string();
        if new_plan.actual_rows != 0 {
            new_plan.planner_row_estimate_factor =
                new_plan.plan_rows as f64 / new_plan.actual_rows as f64;
        }
    }
    new_plan
}

pub fn calculate_actuals(explain: Explain, plan: Plan) -> (Explain, Plan) {
    let mut new_plan: Plan = plan;
    let mut new_explain: Explain = explain;
    new_plan.actual_duration = new_plan.actual_total_time;
    new_plan.actual_cost = new_plan.total_cost;
    for child_plan in new_plan.plans.iter() {
        if child_plan.node_type != CTE_SCAN {
            new_plan.actual_duration = new_plan.actual_duration - child_plan.actual_total_time;
            new_plan.actual_cost = new_plan.actual_cost - child_plan.total_cost;
        }
    }
    if new_plan.actual_cost < 0.0 {
        new_plan.actual_cost = 0.0;
    }

    new_explain.total_cost = new_explain.total_cost + new_plan.actual_cost;
    new_plan.actual_duration = new_plan.actual_duration * new_plan.actual_loops as f64;
    (new_explain, new_plan)
}

pub fn calculate_maximums(explain: Explain, plan: Plan) -> Explain {
    let mut new_explain: Explain = explain;
    if new_explain.max_rows < plan.actual_rows {
        new_explain.max_rows = plan.actual_rows
    }
    if new_explain.max_cost < plan.actual_cost {
        new_explain.max_cost = plan.actual_cost
    }
    if new_explain.max_duration < plan.actual_duration {
        new_explain.max_duration = plan.actual_duration
    }
    new_explain
}

pub fn calculate_outlier_nodes(explain: Explain, plan: Plan) -> Plan {
    let mut new_plan: Plan = plan.clone();
    new_plan.costliest = new_plan.actual_cost == explain.max_cost;
    new_plan.largest = new_plan.actual_rows == explain.max_rows;
    new_plan.slowest = new_plan.actual_duration == explain.max_duration;
    for child_plan in new_plan.plans.iter_mut() {
        *child_plan = calculate_outlier_nodes(explain.clone(), child_plan.clone());
    }
    new_plan
}

fn process_explain(explain: Explain) -> Explain {
    let mut new_explain: Explain = explain;
    new_explain.plan = calculate_planner_estimate(new_explain.plan);
    let (e, p) = calculate_actuals(new_explain.clone(), new_explain.clone().plan);
    new_explain = e.clone();
    new_explain.plan = p.clone();
    new_explain = calculate_maximums(new_explain.clone(), new_explain.plan);
    new_explain
}

fn process_child_plans(explain: Explain, plans: Vec<Plan>) -> (Explain, Vec<Plan>) {
    let mut new_explain: Explain = explain;
    let mut new_plans: Vec<Plan> = plans;
    for mut child_plan in new_plans.iter_mut() {
        *child_plan = calculate_planner_estimate(child_plan.clone());
        let (e, p) = calculate_actuals(new_explain.clone(), child_plan.clone());
        new_explain = e;
        *child_plan = p;
        new_explain = calculate_maximums(new_explain, child_plan.clone());
        if !(child_plan.plans).is_empty() {
            let (e, ps) = process_child_plans(new_explain.clone(), child_plan.plans.clone());
            child_plan.plans = ps;
            new_explain = e;
        }
    }
    (new_explain, new_plans)
}

pub fn process_all(explain: Explain) -> Explain {
    let mut new_explain: Explain = explain;
    new_explain = process_explain(new_explain.clone());
    if !new_explain.plan.plans.is_empty() {
        let (e, ps) = process_child_plans(new_explain.clone(), new_explain.plan.plans.clone());
        new_explain = e;
        new_explain.plan.plans = ps;
    }
    let outlier_plan: Plan = calculate_outlier_nodes(new_explain.clone(), new_explain.clone().plan);
    new_explain.plan = outlier_plan;
    new_explain
}

fn duration_to_string(value: f64) -> colored::ColoredString {
    if value < 100.0 {
        return format!("{0:.2} ms", value).green();
    } else if value < 1000.0 {
        return format!("{0:.2} ms", value).yellow();
    } else if value < 60000.0 {
        return format!("{0:.2} s", value / 2000.0).red();
    } else {
        return format!("{0:.2} m", value / 60000.0).red();
    }
}

pub fn write_explain(explain: Explain, width: usize) {
    println!("○ Total Cost {}", explain.total_cost);
    println!(
        "○ Planning Time: {}",
        duration_to_string(explain.planning_time)
    );
    println!(
        "○ Execution Time: {}",
        duration_to_string(explain.execution_time)
    );
    println!("{}", color_format("┬".to_string(), "output"));
    let p: Plan = explain.clone().plan;
    write_plan(
        explain.clone(),
        &p,
        "".to_string(),
        0,
        width,
        explain.plan.plans.len() == 1,
    )
}

pub fn color_format(s: String, format: &str) -> colored::ColoredString {
    match format {
        "prefix" => s.bright_black(),
        "muted" => s.bright_black(),
        "bold" => s.bright_white(),
        "good" => s.green(),
        "warning" => s.yellow(),
        "critical" => s.red(),
        "output" => s.cyan(),
        "tag" => s.on_bright_red().bright_white(),
        _ => s.green(),
    }
}

fn format_details(plan: Plan) -> String {
    let mut details = vec![];

    if plan.scan_direction != "" {
        details.push(plan.scan_direction);
    }

    if plan.strategy != "" {
        details.push(plan.strategy);
    }

    if details.len() > 0 {
        details.join(", ");
    }

    return "".to_string();
}

fn format_tags(plan: Plan) -> String {
    let mut tags = vec![];

    if plan.slowest {
        tags.push(" slowest ");
    }
    if plan.costliest {
        tags.push(" costliest ");
    }
    if plan.largest {
        tags.push(" largest ");
    }
    if plan.planner_row_estimate_factor >= 100.0 {
        tags.push(" bad estimate ");
    }
    return tags.join(" ");
}

fn get_terminator(index: usize, plan: Plan) -> String {
    if index == 0 {
        if plan.plans.len() == 0 {
            return "⌡► ".to_string();
        } else {
            return "├►  ".to_string();
        }
    } else {
        if plan.plans.len() == 0 {
            return "   ".to_string();
        } else {
            return "│  ".to_string();
        }
    }
}
pub fn format_percent(number: f64, precision: usize) -> String {
    return format!("{:.1$}%", number, precision);
}

pub fn write_plan(
    explain: Explain,
    plan: &Plan,
    prefix: String,
    depth: i32,
    width: usize,
    last_child: bool,
) {
    let mut source_prefix: String = String::from(prefix.clone());
    let mut current_prefix: String = prefix.clone();
    println!("{}{}", color_format(source_prefix.clone(), "prefix"),
             color_format("│".to_string(), "prefix"));
    let mut joint: String = String::from("├");
    if plan.plans.len() > 1 || last_child {
        joint = "└".to_string();
    }
    println!(
        "{}{} {}{} {}",
        color_format(current_prefix.clone(), "prefix"),
        color_format(joint + "─⌠", "prefix"),
        color_format(plan.clone().node_type, "bold"),
        color_format(format_details(plan.clone()), "muted"),
        color_format(format_tags(plan.clone()), "tag")
    );

    if plan.plans.len() > 1 || last_child {
        source_prefix = source_prefix + "  "
    } else {
        source_prefix = source_prefix + "│ "
    }
    current_prefix = source_prefix.clone() + "│ ";
    let cols: usize = width - current_prefix.len();
    let lines: Vec<String> = vec![format!(
        "{}",
        textwrap::fill(DESCRIPTIONS[plan.node_type.as_str()], cols)
    )
    .split("\n")
    .collect()];
    for line in lines {
        println!(
            "{}{}",
            color_format(current_prefix.clone(), "prefix"),
            color_format(line, "muted")
        )
    }
    println!(
        "{}○ Duration: {} {}",
        color_format(current_prefix.clone(), "prefix"),
        duration_to_string(plan.actual_duration),
        format_percent((plan.actual_duration / explain.execution_time) * 100.0, 1)
    );
    println!(
        "{}○ Cost: {} {}",
        color_format(current_prefix.clone(), "prefix"),
        plan.actual_cost,
        format_percent((plan.actual_cost / explain.total_cost) * 100.0, 1)
    );
    println!(
        "{}○ Rows: {}",
        color_format(current_prefix.clone(), "prefix"),
        plan.actual_rows
    );
    current_prefix = current_prefix + "  ";

    if plan.join_type != "" {
        println!(
            "{}{} {}",
            color_format(current_prefix.clone(), "prefix"),
            color_format("join".to_string(), "muted"),
            color_format(plan.clone().join_type, "muted")
        )
    }
    if plan.relation_name != "" {
        println!(
            "{}{} {} {}",
            color_format(current_prefix.clone(), "prefix"),
            color_format("on".to_string(), "muted"),
            color_format(plan.clone().schema, "muted"),
            color_format(plan.clone().relation_name, "muted")
        )
    }

    if plan.index_name != "" {
        println!(
            "{}{} {}",
            color_format(current_prefix.clone(), "prefix"),
            color_format("using".to_string(), "muted"),
            plan.index_name
        )
    }

    if plan.index_condition != "" {
        println!(
            "{}{} {}",
            color_format(current_prefix.clone(), "prefix"),
            color_format("condition".to_string(), "muted"),
            plan.index_condition
        )
    }

    if plan.filter != "" {
        //outputFn("%v %v %v", mutedFormat("filter"), plan.Filter, mutedFormat(fmt.Sprintf("[-%v rows]", humanize.Comma(int64(plan.RowsRemovedByFilter)))))
        println!(
            "{}{} {} [-{} rows]",
            color_format(current_prefix.clone(), "prefix"),
            color_format("filter".to_string(), "muted"),
            plan.filter,
            color_format(plan.rows_removed_by_filter.to_string(), "muted")
        );
    }
    if plan.hash_condition != "" {
        println!(
            "{}{} {}",
            color_format(current_prefix.clone(), "prefix"),
            color_format("on".to_string(), "muted"),
            plan.hash_condition
        )
    }
    if plan.cte_name != "" {
        println!(
            "{}CTE {}",
            color_format(current_prefix.clone(), "prefix"),
            plan.cte_name
        )
    }

    if plan.planner_row_estimate_factor != 0.0 {
        //outputFn("%v %vestimated %v %.2fx", mutedFormat("rows"), plan.PlannerRowEstimateDirection, mutedFormat("by"), plan.PlannerRowEstimateFactor)
        println!(
            "{}{} {}estimated {} {:.2}x",
            color_format(current_prefix.clone(), "prefix"),
            color_format("rows".to_string(), "muted"),
            plan.planner_row_estimate_direction,
            color_format("by".to_string(), "muted"),
            plan.planner_row_estimate_factor
        )
    }

    current_prefix = source_prefix.clone();

    if plan.output.len() > 0 {
        let joined_output: String = plan.output.join(" + ");
        let wrapped_output: String = textwrap::fill(joined_output.as_str(), cols);
        let split_output: Vec<&str> = wrapped_output.split("\n").collect();
        for (index, line) in split_output.iter().enumerate() {
            println!(
                "{}{}",
                color_format(current_prefix.clone(), "prefix"),
                color_format(get_terminator(index, plan.clone()) + line, "output")
            )
        }
    }
    for (index, child_plan) in plan.plans.iter().enumerate() {
        write_plan(explain.clone(), child_plan, source_prefix.clone(), depth+1, width, index == plan.plans.len()-1)
    }
}

pub fn visualize(input: String, width: usize) -> Explain {
    let explains: Vec<Explain> = serde_json::from_str(input.as_str()).unwrap();
    let mut explain: Explain = explains.into_iter().nth(0).unwrap();
    explain = process_all(explain);
    write_explain(explain.clone(), width);
    explain
}
