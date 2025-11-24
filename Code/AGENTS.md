# AGENTS.md - 3D Print Database Coding Guidelines

## Build/Lint/Test Commands
- **Backend dev**: `cd Code/backend && uvicorn app.main:app --reload --host 0.0.0.0 --port 8000`
- **Rust TUI**: `cd frontend_tui_rust && cargo run --release`
- **Python TUI**: `cd frontend_TUI && python main.py`
- **Docker**: `docker-compose -f Code/infra/docker-compose.yml up --build`
- **Format Python**: `cd Code/backend && black .`
- **Lint Python**: `cd Code/backend && flake8 .`
- **Type check**: `cd Code/backend && mypy .`
- **All tests**: `cd Code/backend && python -m pytest tests/ -v`
- **Single test**: `cd Code/backend && python -m pytest tests/test_api.py::test_create_product_api -v`

## Code Style Guidelines
- **Python**: 3.11+ with FastAPI, SQLAlchemy, Pydantic, Tkinter
- **Rust**: 2024 edition with ratatui, tokio, reqwest
- **Formatting**: Black (88-char lines, 4-space indent, trailing commas)
- **Linting**: flake8 (ignores: E203,W503,E501)
- **Type checking**: mypy strict mode (no implicit optional)
- **Imports**: stdlib → third-party → local (blank lines between groups)
- **Naming**: snake_case (functions/vars), PascalCase (classes), UPPERCASE (constants)
- **Type hints**: Required on all public functions/methods, use List/Dict over list/dict
- **Docstrings**: Google-style for public functions (params, returns, raises)
- **Error handling**: HTTPException for APIs, try/finally for DB sessions, rollback on exceptions
- **Database**: SQLAlchemy ORM only, session context managers (`with SessionLocal() as db:`)
- **Security**: Input validation, no raw SQL, environment variables for secrets
- **Async**: Use tokio for Rust async operations, proper error propagation
- **Architecture**: Maintain API consistency across Rust TUI, Python TUI, and Tkinter GUI frontends

## Release & Deployment Checklist
- **Version bump**: Update version in `frontend_tui_rust/src/ui.rs` footer
- **Desktop files**: Update version in `.desktop` file names and comments, commit changes
- **System installation**: Copy versioned `.desktop` file to `~/.local/share/applications/`
- **Permissions**: Ensure desktop file has execute permissions (`chmod +x`)
- **Cache clearing**: Clear Walker cache with `rm -rf ~/.cache/walker/*`
- **Launcher scripts**: Update paths and version references, test execution
- **Binary deployment**: Ensure release binary is built and launcher points to correct path
- **Version verification**: Check that UI shows correct version number after deployment
- **Changelog**: Document new features/fixes in commit messages
- **Test builds**: Ensure both debug and release binaries work before release

## Version Management Strategy
- **Semantic versioning**: Use MAJOR.MINOR.PATCH format (e.g., v0.5.0)
- **Version display**: Always visible in Rust TUI footer (bottom right)
- **Desktop file naming**: Include version in filename (e.g., `3D_Print_Database_TUI_RUST_v0.5.0.desktop`)
- **Launcher validation**: Scripts check for binary existence and show version info
- **Backward compatibility**: Maintain API compatibility across frontend versions
- **Deployment isolation**: Use versioned binaries to prevent running old versions

## Development Workflow
- **Incremental commits**: Commit frequently with descriptive messages for each logical change
- **Version control**: Bump version numbers in `frontend_tui_rust/src/ui.rs` for feature releases
- **Build testing**: Run builds before/after changes: `cd frontend_tui_rust && cargo check`
- **Cross-platform**: Test on target platforms, ensure launcher scripts work correctly
- **Documentation**: Update AGENTS.md when adding new tools, patterns, or guidelines
- **Known issues**: Rust TUI has compilation errors (unclosed delimiters in app.rs) - fix before release