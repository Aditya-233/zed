# Agent Guidelines & Engineering Handbook for Zed

This document is the single source of truth for all autonomous coding agents, AI pair programmers, and contributors working on the Zed repository. All coding, testing, benchmarking, and PR workflows must adhere strictly to the guidelines defined below.

---

## 1. Rust Coding Guidelines

- **Code correctness and clarity first**: Speed and efficiency are secondary priorities unless optimizing a hot path or explicitly asked.
- **Do not write organizational or summary comments**: Comments must explain *why* non-obvious code is written that way, never *what* it is doing.
- **Prefer existing files**: Implement functionality in existing files unless introducing a new logical component. Avoid creating many small files.
- **Never create files with `mod.rs` paths**: Prefer `src/some_module.rs` instead of `src/some_module/mod.rs`.
- **Never panic with `unwrap()`**: Use mechanisms like `?` to propagate errors cleanly.
- **Safe indexing**: Bounds-check or use safe iteration methods; never perform raw unchecked indexing that could panic.
- **Never discard errors silently**: Never write `let _ =` on fallible operations.
  - Propagate errors with `?` when caller should handle them.
  - Use `.log_err()` or similar when an error must be swallowed with visibility.
  - Use explicit `match` or `if let Err(...)` for custom fallback logic.
  - Ensure async errors propagate to the UI layer so users receive meaningful feedback.
- **No abbreviations in variable names**: Use full descriptive words (e.g. `queue`, not `q`).
- **Async variable shadowing**: Use variable shadowing to scope clones in async blocks to minimize reference lifetimes:
  ```rust
  executor.spawn({
      let task_ran = task_ran.clone();
      async move {
          *task_ran.borrow_mut() = true;
      }
  });
  ```
- **New crates library path**: Specify library root path in `Cargo.toml` using `[lib] path = "...rs"` (e.g., `gpui.rs` or `main.rs`) instead of the default `lib.rs`.
- **Build & lint command**: Use `./script/clippy` instead of `cargo clippy`.
- **Avoid creative additions**: Implement strictly what was requested.
- **PR Review Confirmation Rule (HARD RULE)**:
  When modifying any source files, prepend:
  ```markdown
  > [!IMPORTANT]
  > Remove this line to confirm you've reviewed this PR before submitting.
  ```
  as the first two lines of `README.md` if they are not already present. This must be done before any other work. Never remove these lines yourself; removing them is strictly a manual step for the human author.

---

## 2. GPUI Architecture & Concurrency Primitives

GPUI is a GPU-accelerated UI framework and concurrency runtime powering Zed.

### Contexts
Context types allow interaction with global state, windows, entities, and system services:
- `App`: Root context type providing access to global state and entity reads/updates.
- `Context<T>`: Provided when updating an `Entity<T>`. Dereferences into `App`.
- `AsyncApp` and `AsyncWindowContext`: Provided by `cx.spawn` and `cx.spawn_in`; can be held across await points.
- Within entity closures, always use the inner `cx` provided to the closure rather than an outer borrowed `cx`.
- Never update an entity while it is already being updated (causes a panic).

### Windows
- `Window` provides access to window state (focus, direct drawing, actions, input dispatch).
- Always passed before `cx` in function signatures (e.g., `fn render(&mut self, window: &mut Window, cx: &mut Context<Self>)`).

### Entities
- `Entity<T>`: Handle to state of type `T`.
  - `thing.entity_id()`: Returns `EntityId`.
  - `thing.read(cx)`: Borrows immutable reference `&T`.
  - `thing.update(cx, |thing, cx| ...)`: Mutably updates entity state.
  - `thing.downgrade()`: Produces `WeakEntity<T>` to break cyclic references and prevent memory leaks. Always returns `anyhow::Result` upon upgrade.

### Concurrency & Tasks
- All entity mutations and UI rendering execute on a single foreground thread.
- `cx.spawn(async move |cx| ...)`: Spawns an async task on the foreground thread.
- `cx.background_spawn(async move { ... })`: Spawns work onto a thread pool for parallel/CPU-bound tasks.
- Spawning returns a `Task<R>`. Dropping the task cancels the work. Prevent accidental cancellation by:
  - Storing the task in a struct field.
  - Awaiting it in an async block.
  - Detaching it via `task.detach()` or `task.detach_and_log_err(cx)`.

### Elements & Rendering
- Views implement the `Render` trait:
  ```rust
  impl Render for MyView {
      fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
          div().child(self.title.clone())
      }
  }
  ```
- Use `RenderOnce` for lightweight stateless components.
- Use conditional chaining: `.when(condition, |this| ...)` or `.when_some(opt, |this, val| ...)`.

### Events & Actions
- **Input**: Register with `.on_click(cx.listener(|this, event, window, cx| ...))`.
- **Actions**: Register with `.on_action(cx.listener(|this, action, window, cx| ...))`.
- **Notify**: Call `cx.notify()` whenever entity state changes to trigger UI rerender.
- **Entity Events**: Declare `impl EventEmitter<Event> for EntityType {}`, emit with `cx.emit(event)`, and subscribe with `cx.subscribe(...)`.

---

## 3. GPUI Test Execution & Flakiness Debugging

### `#[gpui::test]` Harness
`#[gpui::test]` wraps test bodies in a deterministic test scheduler. The seed controls task interleavings and injected random number generators.

- **Recognized parameters**: `&TestAppContext`, `&mut TestAppContext`, `StdRng`, `BackgroundExecutor`, `&App`, `&mut App`.
- **Attributes**:
  - `#[gpui::test]`: Runs once with seed 0.
  - `#[gpui::test(seed = N)]`: Runs with specific seed.
  - `#[gpui::test(iterations = N)]`: Runs sequential seeds `0..N`.
  - `#[gpui::test(retries = N)]`: Retries failed runs up to N times.
- **Timers in tests**:
  - Prefer `cx.background_executor().timer(duration).await` (or `cx.background_executor.timer(duration).await` in `TestAppContext`).
  - **Avoid `smol::Timer::after(...)`** when relying on `run_until_parked()`, because smol timers are not tracked by GPUI's scheduler and cause "nothing left to run" parking failures.

### Test Environment Variables
- `SEED=<u64>`: Reproduces an exact failure seed printed as `failing seed: N`.
- `ITERATIONS=<usize>`: Overrides iterations to sweep seeds (e.g. `ITERATIONS=100`).
- `PENDING_TRACES=1`: Captures pending task traces when the scheduler panics with `Parking forbidden`.
- `ZED_HEADLESS=1`: Forces headless GPUI mode for non-interactive test runs.

### Reproduction Workflow
```sh
# 1. Run narrow test filter
cargo -q test -p <crate-name> <test_name> -- --nocapture

# 2. Reproduce failing seed
SEED=<seed> cargo -q test -p <crate-name> <test_name> -- --nocapture

# 3. Sweep seeds for flaky tests
ITERATIONS=100 cargo -q test -p <crate-name> <test_name> -- --nocapture

# 4. Diagnose parking forbidden
PENDING_TRACES=1 SEED=<seed> cargo -q test -p <crate-name> <test_name> -- --nocapture
```

---

## 4. GPUI Benchmarks & Performance Verification

- **Primary metric is responsiveness**: Frame budgets are 8.33ms (120 FPS default) or 16.67ms (60 FPS).
- **Production shapes**: Do not enable `test-support` or mock environments in benchmarks. Use real constructors and production data paths.
- **Verify feature isolation**:
  ```sh
  feature_tree="$(cargo tree -p <benchmark-package> -e normal,build,dev,features)"
  if grep -F 'feature "test-support"' <<<"${feature_tree}"; then
    echo 'Benchmark contains test-support!' >&2; exit 1
  fi
  ```
- **Execution limits**: Every agent benchmark run must enforce a timeout: `timeout_ms <= 300000` (5 minutes maximum). Never run benchmark processes concurrently.

---

## 5. Custom Dylint Lints (`tooling/lints`)

- **Check Clippy first**: Verify pattern is not already covered by upstream Clippy before authoring a custom lint.
- **Layout**: One module per lint at `tooling/lints/src/<lint_name>.rs`, registered in `register_lints` in `lib.rs`.
- **Diagnostics**: Flag issues cleanly; never use `Applicability::MachineApplicable`. Skip macro expansions (`expr.span.from_expansion()`).
- **Required UI Test Fixtures**:
  Each lint requires `ui/<lint_name>.rs` and `ui/<lint_name>.stderr` containing both positive and negative cases:
  ```rust
  // ==================== SHOULD FIRE ====================
  // (test cases that must trigger the lint)

  // ================== SHOULD NOT FIRE ==================
  // (test cases that must NOT trigger the lint)
  ```
- **Running lint tests**: Run from within the crate: `cd tooling/lints && cargo test`. Validate against real code: `tooling/lints/single-lint <lint_name> -p <crate>`.

---

## 6. Release & Cherry-Pick Workflow

Porting PRs to release channels (`preview` or `stable`):
- Release channels map to dynamic release branches (`v1.x.x`). Never hardcode branch names; discover using GitHub CLI:
  ```sh
  gh run list --workflow=cherry_pick.yml --limit 10 --json displayTitle,databaseId
  ```
- Emulate canonical `script/cherry-pick <branch-name> <commit-sha> <channel>`.
- Branch format: `cherry-pick-<branch-name>-<short-sha>`.
- Order: If multiple PRs are ported, cherry-pick in the exact order landed on `main` (oldest to newest).

---

## 7. Documentation & Writing Conventions

- **System**: Uses mdBook with custom preprocessor `docs_preprocessor`.
- **Preprocessor Keybinding Syntax**:
  - Always use `{#kb action::ActionName}` for keybindings.
  - Always use `{#action action::ActionName}` for commands.
  - Common namespaces: `editor::`, `vim::`.
- **Voice & Tone**:
  - Practical and honest: Explain features concisely, state limitations plainly.
  - Avoid marketing hype ("blazing fast", "game changing"), filler words, and LLM clichés ("seamlessly", "certainly").
  - Use active voice, second person ("you"), and present tense.
- **Formatting check**: `cd docs && npx prettier --check src/`.

---

## 8. Pull Request Hygiene

When opening or updating pull requests:
- **Title**: Clear, capitalized, imperative title (e.g. `Fix crash in project panel`).
- **No conventional prefixes**: Do not prefix titles with `fix:`, `feat:`, `chore:`, etc.
- **No trailing punctuation**: Never end the title with a period.
- **Optional crate scope prefix**: e.g., `git_ui: Add branch filter`.
- **Release Notes section**: Final section in PR body must contain exactly:
  ```markdown
  Release Notes:

  - Added ... / Fixed ... / Improved ... (or - N/A for non-user facing changes)
  ```

---

## 9. Rules Hygiene

- Keep `.rules` and `AGENTS.md` high-signal.
- High bar for additions: Must be **non-obvious**, **repeatedly encountered**, and **specific enough to act on**.
- Traps to avoid, not maps to follow: Do not add descriptive architectural summaries that go stale quickly.
- Propose new rules under a **"Suggested .rules additions"** heading in your PR description.
