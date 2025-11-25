# AGENTS.md - 3D Print Database Coding Guidelines

This document provides coding guidelines and requirements for AI agents working on the 3D Print Database project. AI agents must adhere to these guidelines to ensure consistency, quality, and proper project management.

## Build/Lint/Test Commands
- **Backend dev**: `cd Code/backend && uvicorn app.main:app --reload --host 0.0.0.0 --port 8000`
- **Rust TUI**: `cd frontend_tui_rust && cargo run --release`
- **Python TUI**: `cd frontend_TUI && python main.py`
- **Format Python**: `cd Code/backend && black .`
- **Lint Python**: `cd Code/backend && flake8 .`
- **Type check**: `cd Code/backend && mypy .`
- **All tests**: `cd Code/backend && python -m pytest tests/ -v`
- **Single test**: `cd Code/backend && python -m pytest tests/test_api.py::test_create_product_api -v`
- **Rust format**: `cd frontend_tui_rust && cargo fmt` (if rustfmt.toml exists)
- **Rust clippy**: `cd frontend_tui_rust && cargo clippy -- -D warnings`

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
- git commit
- recompile
- verify that there is only one compiled binary - Walker launcher must run latest compiled version
- current version number must be informed in final response
- update test_routine.txt if any functionality changes (add new test cases, modify existing ones, or remove obsolete tests)
- ensure test_routine.txt reflects current application capabilities and features

## Project Focus
- **Project focus**: 3d print database TUI (Rust)
- **Objective**: mimic the python front end that is already working
- **Backend/Database changes**: do not make changes to backend or database without first consulting with user (to avoid breaking the working python front end)