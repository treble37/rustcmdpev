use colored::*;
use phf::phf_map;
pub mod structure;
use structure::explain::explain;
use structure::plan::plan;

const UNDER: &str = "Under";
const OVER: &str = "Over";
const CTE_SCAN: &str = "CTE Scan";

const DELTA_ERROR: f64 = 0.001;

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
    "" => "" // handle case where key isn't populated in initial input
};

// https://www.forrestthewoods.com/blog/should-small-rust-structs-be-passed-by-copy-or-by-borrow/
pub fn calculate_planner_estimate(plan: plan::Plan) -> plan::Plan {
    let mut new_plan: plan::Plan = plan;
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

pub fn calculate_actuals(
    explain: explain::Explain,
    plan: plan::Plan,
) -> (explain::Explain, plan::Plan) {
    let mut new_plan: plan::Plan = plan;
    let mut new_explain: explain::Explain = explain;
    new_plan.actual_duration = new_plan.actual_total_time;
    new_plan.actual_cost = new_plan.total_cost;
    for child_plan in new_plan.plans.iter() {
        if child_plan.node_type != CTE_SCAN {
            new_plan.actual_duration -= child_plan.actual_total_time;
            new_plan.actual_cost -= child_plan.total_cost;
        }
    }
    if new_plan.actual_cost < 0.0 {
        new_plan.actual_cost = 0.0;
    }

    new_explain.total_cost += new_plan.actual_cost;
    new_plan.actual_duration *= new_plan.actual_loops as f64;
    (new_explain, new_plan)
}

pub fn calculate_maximums(explain: explain::Explain, plan: plan::Plan) -> explain::Explain {
    let mut new_explain: explain::Explain = explain;
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

pub fn calculate_outlier_nodes(explain: explain::Explain, plan: plan::Plan) -> plan::Plan {
    let mut new_plan: plan::Plan = plan;
    new_plan.costliest = (new_plan.actual_cost - explain.max_cost).abs() < DELTA_ERROR;
    new_plan.largest = new_plan.actual_rows == explain.max_rows;
    new_plan.slowest = (new_plan.actual_duration - explain.max_duration).abs() < DELTA_ERROR;
    for child_plan in new_plan.plans.iter_mut() {
        *child_plan = calculate_outlier_nodes(explain.clone(), child_plan.clone());
    }
    new_plan
}

fn process_explain(explain: explain::Explain) -> explain::Explain {
    let mut new_explain: explain::Explain = explain;
    new_explain.plan = calculate_planner_estimate(new_explain.plan);
    let (e, p) = calculate_actuals(new_explain.clone(), new_explain.clone().plan);
    new_explain = e;
    new_explain.plan = p;
    new_explain = calculate_maximums(new_explain.clone(), new_explain.plan);
    new_explain
}

fn process_child_plans(
    explain: explain::Explain,
    plans: Vec<plan::Plan>,
) -> (explain::Explain, Vec<plan::Plan>) {
    let mut new_explain: explain::Explain = explain;
    let mut new_plans: Vec<plan::Plan> = plans;
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

pub fn process_all(explain: explain::Explain) -> explain::Explain {
    let mut new_explain: explain::Explain = explain;
    new_explain = process_explain(new_explain.clone());
    if !new_explain.plan.plans.is_empty() {
        let (e, ps) = process_child_plans(new_explain.clone(), new_explain.plan.plans.clone());
        new_explain = e;
        new_explain.plan.plans = ps;
    }
    let outlier_plan: plan::Plan =
        calculate_outlier_nodes(new_explain.clone(), new_explain.clone().plan);
    new_explain.plan = outlier_plan;
    new_explain
}

fn duration_to_string(value: f64) -> colored::ColoredString {
    if value < 100.0 {
        format!("{0:.2} ms", value).green()
    } else if value < 1000.0 {
        format!("{0:.2} ms", value).yellow()
    } else if value < 60000.0 {
        format!("{0:.2} s", value / 2000.0).red()
    } else {
        format!("{0:.2} m", value / 60000.0).red()
    }
}

pub fn write_explain(explain: explain::Explain, width: usize) {
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
    let p: plan::Plan = explain.clone().plan;
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

fn format_details(plan: plan::Plan) -> String {
    let mut details = vec![];

    if plan.scan_direction != "" {
        details.push(plan.scan_direction);
    }

    if plan.strategy != "" {
        details.push(plan.strategy);
    }

    if !details.is_empty() {
        details.join(", ");
    }

    "".to_string()
}

fn format_tags(plan: plan::Plan) -> String {
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
    tags.join(" ")
}

fn get_terminator(index: usize, plan: plan::Plan) -> String {
    if index == 0 {
        if plan.plans.is_empty() {
            "⌡► ".to_string()
        } else {
            "├►  ".to_string()
        }
    } else if plan.plans.is_empty() {
        "   ".to_string()
    } else {
        "│  ".to_string()
    }
}

pub fn format_percent(number: f64, precision: usize) -> String {
    return format!("{:.1$}%", number, precision);
}

pub fn write_plan(
    explain: explain::Explain,
    plan: &plan::Plan,
    prefix: String,
    depth: i32,
    width: usize,
    last_child: bool,
) {
    let mut source_prefix: String = prefix.clone();
    let mut current_prefix: String = prefix;
    println!(
        "{}{}",
        color_format(source_prefix.clone(), "prefix"),
        color_format("│".to_string(), "prefix")
    );
    let joint = if plan.plans.len() > 1 || last_child {
        "└".to_string()
    } else {
        String::from("├")
    };
    println!(
        "{}{} {}{} {}",
        color_format(current_prefix.clone(), "prefix"),
        color_format(joint + "─⌠", "prefix"),
        color_format(plan.clone().node_type, "bold"),
        color_format(format_details(plan.clone()), "muted"),
        color_format(format_tags(plan.clone()), "tag")
    );

    if plan.plans.len() > 1 || last_child {
        source_prefix += "  "
    } else {
        source_prefix += "│ "
    }
    current_prefix = source_prefix.clone() + "│ ";
    let cols: usize = width - current_prefix.len();
    for line in textwrap::fill(DESCRIPTIONS[plan.node_type.as_str()], cols)
        .split('\n')
        .collect::<Vec<&str>>()
    {
        println!(
            "{}{}",
            color_format(current_prefix.clone(), "prefix"),
            color_format(line.to_string(), "muted")
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
    current_prefix += "  ";

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

    if !plan.output.is_empty() {
        let joined_output: String = plan.output.join(" + ");
        let wrapped_output: String = textwrap::fill(joined_output.as_str(), cols);
        let split_output: Vec<&str> = wrapped_output.split('\n').collect();
        for (index, line) in split_output.iter().enumerate() {
            println!(
                "{}{}",
                color_format(current_prefix.clone(), "prefix"),
                color_format(get_terminator(index, plan.clone()) + line, "output")
            )
        }
    }
    for (index, child_plan) in plan.plans.iter().enumerate() {
        write_plan(
            explain.clone(),
            child_plan,
            source_prefix.clone(),
            depth + 1,
            width,
            index == plan.plans.len() - 1,
        )
    }
}

pub fn visualize(input: String, width: usize) -> explain::Explain {
    let explains: Vec<explain::Explain> = serde_json::from_str(input.as_str()).unwrap();
    let mut explain: explain::Explain = explains.into_iter().next().unwrap();
    explain = process_all(explain);
    write_explain(explain.clone(), width);
    explain
}
