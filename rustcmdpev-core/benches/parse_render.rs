//! G9.5 — Criterion benchmarks for the hot paths: JSON parse → analysis →
//! pretty render. Two payload sizes (small canonical fixture, deep synthetic
//! tree) to detect regressions in either steady-state or worst-case work.

use std::fmt::Write as _;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rustcmdpev_core::display::colors::Theme;
use rustcmdpev_core::parser::parse_explain_document;
use rustcmdpev_core::render::{render_explain, RenderOptions};
use rustcmdpev_core::{analyze_explain, render_visualization_with};

const SMALL_PAYLOAD: &str = include_str!("../../example.json");

fn synthesize_deep_payload(depth: usize, fanout: usize) -> String {
    fn write_node(buf: &mut String, depth: usize, fanout: usize) {
        write!(
            buf,
            r#"{{"Node Type":"Hash Join","Total Cost":{cost},"Plan Rows":{rows},"Actual Rows":{rows},"Actual Total Time":{time},"Actual Loops":1"#,
            cost = (depth as f64) * 1.5 + 1.0,
            rows = depth + 1,
            time = (depth as f64) * 0.1 + 0.05,
        )
        .unwrap();
        if depth > 0 {
            buf.push_str(r#","Plans":["#);
            for i in 0..fanout {
                if i > 0 {
                    buf.push(',');
                }
                write_node(buf, depth - 1, fanout);
            }
            buf.push(']');
        }
        buf.push('}');
    }

    let mut payload = String::with_capacity(64 * 1024);
    payload.push_str(r#"[{"Plan":"#);
    write_node(&mut payload, depth, fanout);
    payload.push_str(r#","Execution Time":10.0,"Planning Time":1.0}]"#);
    payload
}

fn bench_parse(c: &mut Criterion) {
    let deep = synthesize_deep_payload(6, 2); // 2^7 - 1 = 127 nodes
    let mut group = c.benchmark_group("parse_explain_document");
    group.throughput(Throughput::Bytes(SMALL_PAYLOAD.len() as u64));
    group.bench_function("small_fixture", |b| {
        b.iter(|| {
            let parsed = parse_explain_document(black_box(SMALL_PAYLOAD)).expect("parse");
            black_box(parsed);
        })
    });
    group.throughput(Throughput::Bytes(deep.len() as u64));
    group.bench_function("deep_synthetic_127_nodes", |b| {
        b.iter(|| {
            let parsed = parse_explain_document(black_box(&deep)).expect("parse");
            black_box(parsed);
        })
    });
    group.finish();
}

fn bench_render(c: &mut Criterion) {
    let small = analyze_explain(parse_explain_document(SMALL_PAYLOAD).expect("parse small"));
    let deep_payload = synthesize_deep_payload(6, 2);
    let deep = analyze_explain(parse_explain_document(&deep_payload).expect("parse deep"));

    let mut group = c.benchmark_group("render_explain");
    group.bench_function("small_fixture", |b| {
        let options = RenderOptions::new(80).with_theme(Theme::NoColor);
        b.iter(|| {
            let rendered = render_explain(black_box(&small), options);
            black_box(rendered);
        })
    });
    group.bench_function("deep_synthetic_127_nodes", |b| {
        let options = RenderOptions::new(80).with_theme(Theme::NoColor);
        b.iter(|| {
            let rendered = render_explain(black_box(&deep), options);
            black_box(rendered);
        })
    });
    group.finish();
}

fn bench_end_to_end(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_then_render");
    let options = RenderOptions::new(80).with_theme(Theme::NoColor);
    group.bench_function("small_fixture", |b| {
        b.iter(|| {
            let rendered = render_visualization_with(black_box(SMALL_PAYLOAD), options).expect("render");
            black_box(rendered);
        })
    });
    group.finish();
}

criterion_group!(benches, bench_parse, bench_render, bench_end_to_end);
criterion_main!(benches);
