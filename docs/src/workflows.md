# psql Workflows

These examples show common ways to feed PostgreSQL `EXPLAIN (FORMAT JSON)` output into `rustcmdpev`.

All examples assume:

- `psql` is available on `PATH`.
- The target database connection is supplied through your shell environment.
- `rustcmdpev` is already installed, or you replace `rustcmdpev` with `cargo run -p rustcmdpev --`.

## Linux

Run a query directly from the shell and pipe the JSON plan into `rustcmdpev`:

```bash
printf '%s\n' \
  'EXPLAIN (ANALYZE, COSTS, VERBOSE, BUFFERS, FORMAT JSON) SELECT * FROM coaches;' \
  | psql -qXAt "$DATABASE_URL" \
  | rustcmdpev
```

## macOS

Use the clipboard-driven workflow to explain the query you already copied:

```bash
pbpaste \
  | sed '1s/^/EXPLAIN (ANALYZE, COSTS, VERBOSE, BUFFERS, FORMAT JSON) /' \
  | psql -qXAt "$DATABASE_URL" \
  | rustcmdpev
```

## Windows PowerShell

Send a multi-line query from a here-string into `psql` and render the plan:

```powershell
@'
EXPLAIN (ANALYZE, COSTS, VERBOSE, BUFFERS, FORMAT JSON)
SELECT * FROM coaches;
'@ | psql -qXAt $env:DATABASE_URL | rustcmdpev.exe
```

## Windows CMD

Use a one-line `echo` pipeline when working from `cmd.exe`:

```bat
(echo EXPLAIN (ANALYZE, COSTS, VERBOSE, BUFFERS, FORMAT JSON) SELECT * FROM coaches;) | psql -qXAt %DATABASE_URL% | rustcmdpev.exe
```

## Local fixture fallback

If you want to verify the renderer without a live database session:

```bash
cat example.json | rustcmdpev
```
