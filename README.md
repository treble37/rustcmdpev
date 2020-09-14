# Rustcmdpev

A command-line Rust Postgres query visualizer, heavily inspired by the excellent (web-based) [pev](https://github.com/AlexTatiyants/pev).
It started out being ported from [gocmdpev](https://github.com/simon-engledew/gocmdpev)

# Demo

# Install

# Overview

## Usage

## Local development

cargo run -- '[{"Plan":{"Alias":"c0","Node Type":"Seq Scan","Parallel Aware":false,"Plan Rows":50,"Plan Width":1572,"Relation Name":"coaches","Startup Cost":0.0,"Total Cost":10.5}}]'

## Testing

To see output from print statements, run with nocapture flag:

`cargo test -- --nocapture`
