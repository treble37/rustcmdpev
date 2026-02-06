# Overview

This is a compact view of the architecture and runtime flow. For the full multi-angle analysis, see `CODEBASE_OVERVIEW.md` in the repo root.

Core pipeline:

```mermaid
flowchart LR
    A[stdin] --> B[main.rs]
    B --> C[visualize]
    C --> D[serde_json::from_str -> Explain]
    D --> E[process_all]
    E --> F[write_explain]
    F --> G[write_plan recursion]
    G --> H[stdout]
```

Key modules:

- `src/main.rs` reads stdin and delegates to the library.
- `src/lib.rs` holds parse, analysis, and render logic.
- `src/display/*` handles formatting and colors.
- `src/structure/*` defines the serde models for EXPLAIN JSON.

Data model (simplified):

```mermaid
classDiagram
    class Explain {
        +Plan plan
        +f64 planning_time
        +f64 execution_time
        +f64 total_cost
        +u64 max_rows
        +f64 max_cost
        +f64 max_duration
    }

    class Plan {
        +String node_type
        +u64 actual_rows
        +u64 plan_rows
        +f64 actual_total_time
        +f64 total_cost
        +Vec~Plan~ plans
        +bool costliest
        +bool slowest
        +bool largest
        +...
    }

    Explain --> Plan : root
    Plan --> Plan : children
```
