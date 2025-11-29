# AGENTS.md - 3D Print Database Coding Guidelines

This document provides coding guidelines and requirements for AI agents working on the 3D Print Database project. AI agents must adhere to these guidelines to ensure consistency, quality, and proper project management.

## Build/Lint/Test Commands
- **Backend dev**: `cd Code/backend && uvicorn app.main:app --reload --host 0.0.0.0 --port 8000`
- **Rust TUI**: `cd Code/frontend && cargo run --release`
- **Python TUI**: `cd frontend_py && python 3dPrintDB.py`
- **Format Python**: `cd Code/backend && black .`
- **Lint Python**: `cd Code/backend && flake8 .`
- **Type check**: `cd Code/backend && mypy .`
- **All tests**: `cd Code/backend && python -m pytest tests/ -v`
- **Single test**: `cd Code/backend && python -m pytest tests/test_api.py::test_create_product_api -v`
- **Rust format**: `cd Code/frontend && cargo fmt`
- **Rust clippy**: `cd Code/frontend && cargo clippy -- -D warnings`

## Code Style Guidelines
- **Languages**: Python 3.11+ (FastAPI, SQLAlchemy, Pydantic), Rust 2021 (ratatui, tokio, reqwest)
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

## Project Structure
- **Backend**: `Code/backend/` - FastAPI application with SQLAlchemy ORM
- **Frontend**: `Code/frontend/` - Rust TUI with ratatui
- **Frontend Legacy**: `frontend_deprecated/` - Old Rust TUI (preserved for reference)
- **Frontend Python**: `frontend_py/` - Python Tkinter GUI
- **Tests**: `Code/backend/tests/` - pytest test suite
- **Infrastructure**: `Code/infra/` - Docker compose configuration

## Key Development Rules
- **Database changes**: Do not modify backend or database without consulting user (to avoid breaking working Python frontend)
- **Version management**: Update version numbers in both Cargo.toml and backend after significant changes
- **Testing**: Add tests for new features before moving to next feature
- **Commits**: Commit changes with descriptive messages after each feature completion
- **File paths**: Use `~/Work/3d_print/Products` for product storage (set via PRODUCTS_DIR env var)

## API Endpoints
- **Unified product endpoint**: `POST /products/` - Handles both create (product_id=null) and update (product_id=N)
- **Product schema**: Uses ID-based relationships (tag_ids, material_ids, category_id)
- **Error responses**: Return proper HTTP status codes with descriptive messages

## Environment Variables
- **PRODUCTS_DIR**: Override default products directory (default: `~/Work/3d_print/Products`)
- **DATABASE_URL**: PostgreSQL connection string for backend
- **PYTHONPATH**: Ensure backend modules are importable

## Testing Requirements
- **Backend tests**: Use pytest with test database fixtures
- **API testing**: Test both create and update operations via HTTP endpoints
- **Database testing**: Verify CRUD operations maintain data integrity
- **Integration testing**: Test frontend-backend communication