# Async Lab CLI

This package provides the `async-lab` binary used by `cargo run` at the workspace root.

## Design

- Uses `spinners` for its terminal loader and otherwise stays deliberately lightweight.
- Discovers `NN-*/exercises/*.rs` files in lexical order.
- Requires a same-named checker and reference solution; hints are optional.
- Compiles each checker directly with `rustc --test`.
- Shows terminal-aware colors and the `Aesthetic` spinner during compilation and tests.
- Renders hints under an uppercase `💡 HINT` heading and converts simple Markdown emphasis to ANSI
  styling.
- Disables animation for redirected output and respects the standard `NO_COLOR` environment variable.
- Polls the active source's content fingerprint every 200 ms.
- Stores successful source-and-check fingerprints in `.async-lab/progress.tsv`.
- Accepts any implementation that satisfies the checker.

The reference solution is validated as part of repository maintenance, but the CLI never copies it
over a learner's source and never compares the two files as text.
