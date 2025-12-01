# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs` wires daily puzzles and input loading; each puzzle lives in its own module (e.g., `src/day1.rs`).
- `inputs/` holds puzzle inputs (`inputs/day1_input`). Keep raw inputs out of `src/`.
- `target/` is build output; avoid committing it. `test/` is reserved for integration tests (currently empty).
- Prefer creating `src/dayN.rs` for new days and expose a `solution` entry if needed.

## Build, Test, and Development Commands
- `cargo fmt` runs rustfmt to normalize style.
- `cargo check` performs fast type checking without building artifacts.
- `cargo test` runs unit and integration tests. Add `-- --nocapture` to see stdout.
- `cargo run` executes the default binary; use `cargo run --release` for speed when inputs are large.

## Coding Style & Naming Conventions
- Use Rust 2024 edition defaults: 4-space indentation, snake_case for functions/variables, CamelCase for types, SCREAMING_SNAKE_CASE for constants.
- Run `cargo fmt` before pushing; avoid manual alignment that rustfmt will change.
- Keep modules small and puzzle-focused; prefer pure functions that accept input strings rather than reading files directly.
- Place shared helpers in a module (e.g., `src/util.rs`) instead of duplicating per day.

## Testing Guidelines
- Unit tests belong beside implementations under `#[cfg(test)]` in the same file; integration tests can go in `test/`.
- Name tests by behavior: `day1_parses_map`, `part2_handles_wraparound`.
- Add at least one test per puzzle part using trimmed sample inputs; avoid relying on large real inputs to keep tests fast.
- When adding parsing helpers, test both happy path and common malformed cases.

## Input Handling & Configuration
- Keep input files in `inputs/`; main currently loads from there. If a puzzle needs derived fixtures, generate them under `target/` or `.cache/`, not under version control.
- Avoid hardcoding absolute paths; use `Path`/`PathBuf` relative to the repository root.
- Do not commit real puzzle answers or personal tokens.

## Commit & Pull Request Guidelines
- Use concise, imperative commit subjects (`Add day2 parser`, `Refine step counting`); include a short body for rationale or edge cases when helpful.
- Group unrelated changes into separate commits (e.g., formatting vs. behavior).
- Pull requests should state the puzzle/day addressed, summarize behavior changes, call out new commands or flags, and note any test coverage gaps.
- Link related issues when applicable and include screenshots only if UI output is involved (not expected here).
