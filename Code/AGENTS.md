# AGENTS.md - 3D Print Database Coding Guidelines

## Build/Lint/Test Commands
- **Backend dev**: `cd Code/backend && uvicorn app.main:app --reload --host 0.0.0.0 --port 8000`
- **Frontend**: `./3dPrintDB.sh`
- **Docker**: `docker-compose -f Code/infra/docker-compose.yml up --build`
- **Format**: `cd Code/backend && black .`
- **Lint**: `cd Code/backend && flake8 .`
- **Type check**: `cd Code/backend && mypy .`
- **All tests**: `cd Code/backend && python -m pytest tests/ -v`
- **Single test**: `cd Code/backend && python -m pytest tests/test_api.py::test_create_product_api -v`

## Release & Deployment Checklist
- **Version bump**: Update version in `frontend_tui_rust/src/ui.rs` footer
- **Desktop files**: Update version in `.desktop` file names and comments
- **Changelog**: Document new features/fixes in commit messages
- **Test builds**: Ensure both debug and release binaries work
- **Launcher scripts**: Update paths and version references

## Code Style Guidelines
- **Python**: 3.11+ with FastAPI, SQLAlchemy, Pydantic, Tkinter
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