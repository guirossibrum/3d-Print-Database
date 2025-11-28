# AGENTS.md - 3D Print Database Coding Guidelines

This document provides coding guidelines and requirements for AI agents working on the 3D Print Database project. AI agents must adhere to these guidelines to ensure consistency, quality, and proper project management.

## Frontend Rebuild Guidelines
- **New folder**: `frontend/` (rename old frontend to `frontend-deprecated/`)
- **Architecture**: Mode-driven, modular, readable.
- **Folder structure**: Clear separation:
  - `handlers/` → normal, edit, create, delete, select
  - `ui/` → draw, popups, layout
  - `api.rs` → all backend calls
  - `utils.rs` → file tree, helper functions
- **Event dispatch**: All key events go through `handle_event()` dispatcher.
- **Popup system**: Consolidated in `delete.rs` with mode-aware states.
- **Backend integration**: Unified product payload including materials.
- **Stepwise rebuild**: Implement one feature at a time, verify backend compatibility, commit each step.
- **Testing**: Add Rust unit tests for every feature before moving to the next.

## Build/Lint/Test Commands
- **Backend dev**: `cd Code/backend && uvicorn app.main:app --reload --host 0.0.0.0 --port 8000`
- **Rust TUI**: `cd Code/frontend && cargo run --release`
- **Python TUI**: `cd frontend_TUI && python main.py`
- **Format Python**: `cd Code/backend && black .`
- **Lint Python**: `cd Code/backend && flake8 .`
- **Type check**: `cd Code/backend && mypy .`
- **All tests**: `cd Code/backend && python -m pytest tests/ -v`
- **Single test**: `cd Code/backend && python -m pytest tests/test_api.py::test_create_product_api -v`
- **Rust format**: `cd Code/frontend && cargo fmt` (if rustfmt.toml exists)
- **Rust clippy**: `cd Code/frontend && cargo clippy -- -D warnings`

## Code Style Guidelines
- **Languages**: Python 3.11+ (FastAPI, SQLAlchemy, Pydantic, Tkinter), Rust 2024 (ratatui, tokio, reqwest)
- **Formatting**: Black (88-char lines, 4-space indent, trailing commas), rustfmt standard for Rust
- **Linting**: flake8 (ignores: E203, W503, E501), clippy with -D warnings for Rust
- **Type checking**: mypy strict mode (no implicit optional, required on all public functions)
- **Imports**: stdlib → third-party → local (blank lines between groups)
- **Naming**: snake_case (functions/vars), PascalCase (classes), UPPERCASE (constants)
- **Docstrings**: Google-style for public functions (params, returns, raises)
- **Error handling**: HTTPException for APIs, try/finally for DB sessions, rollback on exceptions, Result<T> for Rust
- **Database**: SQLAlchemy ORM only, session context managers (`with SessionLocal() as db:`)
- **Security**: Input validation, no raw SQL, environment variables for secrets
- **Async**: Use tokio for Rust async operations, proper error propagation
- **Architecture**: Maintain API consistency across Rust TUI, Python TUI, and Tkinter GUI frontends

## REQUIREMENTS
After each change in the code:
- increase version count

- run the build script in the project folder: `./build.sh`
- git commit
- verify that there is only one compiled binary - omarchy launcher must run latest compiled version
- current version number must be informed in final response
- update test_routine.txt if any functionality changes (add new test cases, modify existing ones, or remove obsolete tests)
- ensure test_routine.txt reflects current application capabilities and features

## Application Launcher System
The application uses **omarchy** (not Walker) as the launcher system. The launcher chain works as follows:

1. **Desktop file**: `~/.local/share/applications/3D_Print_Database_TUI.desktop`
   - Must contain: `Exec=omarchy-launch-or-focus-3d-print-database-tui`
   - Version must match current application version

2. **Focus launcher**: `~/.local/share/omarchy/bin/omarchy-launch-or-focus-3d-print-database-tui`
   - Calls: `omarchy-launch-or-focus "$APP_ID" "$LAUNCH_COMMAND"`

3. **Main launcher**: `~/.local/share/omarchy/bin/omarchy-launch-3d-print-database-tui`
   - Uses: `setsid uwsm-app -- xdg-terminal-exec --app-id=org.omarchy.$APP_NAME -e /path/to/binary`

4. **Binary path**: `/home/grbrum/Work/3d_print/Code/frontend/target/release/frontend_tui_rust`

### Common Launcher Issues & Fixes:
- **Problem**: Desktop file calls binary directly instead of using omarchy
  - **Fix**: Change `Exec=` line to use `omarchy-launch-or-focus-3d-print-database-tui`
- **Problem**: Version mismatch between desktop file and application
  - **Fix**: Update `Version=` field in desktop file to match Cargo.toml
- **Problem**: Binary not found
  - **Fix**: Ensure binary exists at expected path and is executable
- **Problem**: Terminal setup errors
  - **Fix**: This is expected when testing outside terminal; omarchy handles terminal creation

### Launcher Verification:
```bash
# Check desktop file configuration
cat ~/.local/share/applications/3D_Print_Database_TUI.desktop

# Check omarchy launchers exist
ls -la ~/.local/share/omarchy/bin/omarchy-launch-*3d-print*

# Test binary directly (will fail in non-terminal - this is expected)
cd Code/frontend && ./target/release/frontend_tui_rust --version
```

## Key Behavior Specification

### Enter
- **Always confirms and saves record in modes where user is modifying something**
- Save a new record (Create tab)
- Save an edit record (Search tab edit modes)

### Esc
- **Always cancels and returns to previous state/mode**
- Never closes the app
- Edit mode → returns to normal mode
- Tag/Material edit mode → returns to edit mode
- Popups → do nothing (handled with y/n keys)
- Create tab → cancels creation, returns to normal mode

### Tab
- **Always advances to next level**
- Currently valid only in Search tab
- Normal mode → Edit mode → Sub-edit (tags/materials/other) → (later?)
- Create mode → Sub-edit (tags/materials/other)
- Backtab not implemented

### Up/Down Arrows
- **Up/Down navigate whatever the “current focus group” contains.**
- Lists in list mode
- Fields in form mode

### Right/Left Arrows
- **Change tabs in normal mode**
- **Toggle selection for boolean items**

### n Key
- **Always create new record**
- Creates new Tag, Material, or Category

### d Key
- **Always delete selected record**
- Only if not used by any product (backend validation)
- Deletes Tag, Material, or Category

### Space Key
- **Always toggle [x] selection on multi selection items**
- Used in Tag and Material selection lists

## Project Focus
- **Project focus**: 3d print database TUI (Rust)
- **Objective**: mimic the python front end that is already working
- **Backend/Database changes**: do not make changes to backend or database without first consulting with user (to avoid breaking the working python front end)