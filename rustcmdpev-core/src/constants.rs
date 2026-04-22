use phf::phf_map;

pub const UNDER_LABEL: &str = "Under";
pub const OVER_LABEL: &str = "Over";
pub const CTE_SCAN_NODE: &str = "CTE Scan";

pub const DELTA_ERROR_THRESHOLD: f64 = 0.001;
pub const BAD_ESTIMATE_FACTOR_THRESHOLD: f64 = 100.0;
pub const MAX_PLAN_DEPTH: usize = 32;
pub const MAX_PLAN_NODES: usize = 10_000;

pub const TREE_VERTICAL: &str = "│";
pub const TREE_ELBOW: &str = "└";
pub const TREE_BRANCH: &str = "├";
pub const TREE_NODE_CONNECTOR: &str = "─⌠";
pub const TREE_ROOT_MARKER: &str = "┬";
pub const TREE_OUTPUT_CHILD: &str = "⌡► ";
pub const TREE_OUTPUT_BRANCH: &str = "├►  ";
pub const TREE_OUTPUT_CONTINUATION: &str = "│  ";
pub const TREE_OUTPUT_PADDING: &str = "   ";

pub const TAG_SLOWEST: &str = " slowest ";
pub const TAG_COSTLIEST: &str = " costliest ";
pub const TAG_LARGEST: &str = " largest ";
pub const TAG_BAD_ESTIMATE: &str = " bad estimate ";

pub static DESCRIPTIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Append" => "Used in a UNION to merge multiple record sets by appending them together.",
    "Limit" => "Returns a specified number of rows from a record set.",
    "Sort" => "Sorts a record set based on the specified sort key.",
    "Nested Loop" => "Merges two record sets by looping through every record in the first set and trying to find a match in the second set. All matching records are returned.",
    "Merge Join" => "Merges two record sets by first sorting them on a join key.",
    "Hash" => "Generates a hash table from the records in the input recordset. Hash is used by Hash Join.",
    "Hash Join" => "Joins to record sets by hashing one of them (using a Hash Scan).",
    "Aggregate" => "Groups records together based on a GROUP BY or aggregate function (e.g. sum()).",
    "Hashaggregate" => "Groups records together based on a GROUP BY or aggregate function (e.g. sum()). Hash Aggregate uses a hash to first organize the records by a key.",
    "Seq Scan" => "Finds relevant records by sequentially scanning the input record set. When reading from a table, Seq Scans (unlike Index Scans) perform a single read operation (only the table is read).",
    "Index Scan" => "Finds relevant records based on an Index. Index Scans perform 2 read operations: one to read the index and another to read the actual value from the table.",
    "Index Only Scan" => "Finds relevant records based on an Index. Index Only Scans perform a single read operation from the index and do not read from the corresponding table.",
    "Bitmap Heap Scan" => "Searches through the pages returned by the Bitmap Index Scan for relevant rows.",
    "Bitmap Index Scan" => "Uses a Bitmap Index (index which uses 1 bit per page) to find all relevant pages. Results of this node are fed to the Bitmap Heap Scan.",
    "CTE Scan" => "Performs a sequential scan of Common Table Expression (CTE) query results. Note that results of a CTE are materialized (calculated and temporarily stored).",
    "" => "",
};
