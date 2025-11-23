# AGENTS.md - Coding Guidelines for 3D Print Repository

## Build/Lint/Test Commands
- **Run backend**: `cd backend && uvicorn app.main:app --reload`
- **Docker build**: `docker-compose -f infra/docker-compose.yml up --build`
- **Fix permissions**: `./fix_permissions.sh` (run after Docker creates files with wrong ownership)
- **No linting configured** - consider adding black, flake8, mypy
- **No tests configured** - add pytest for testing
- **Run single test**: `pytest tests/test_file.py::test_function` (when tests are added)

## Code Style Guidelines
- **Language**: Python 3.11+ with FastAPI, SQLAlchemy, Pydantic
- **Imports**: Standard library → third-party → local modules
- **Naming**: snake_case for functions/variables, PascalCase for classes
- **Types**: Use type hints on all function parameters and return values
- **Docstrings**: Add docstrings to all public functions
- **Error handling**: Use try/finally for database sessions, raise HTTPException for API errors
- **Database**: Use SQLAlchemy ORM, avoid raw SQL
- **Schemas**: Inherit from Pydantic BaseModel, use `from_attributes=True` for ORM conversion
- **File structure**: Keep models, schemas, crud operations separate
- **Formatting**: 4-space indentation, consistent with existing code

## Docker Setup
- **Permissions**: Container sets umask 002 for proper file permissions
- **Volumes**: Host Products directory should have 775 permissions
- **User**: Runs as root with proper umask (more secure than non-root for volume mounting)