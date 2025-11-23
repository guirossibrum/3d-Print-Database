# AGENTS.md - 3D Print Database Coding Guidelines

## Development Commands
- **Backend**: `cd Code/backend && uvicorn app.main:app --reload --host 0.0.0.0 --port 8000`
- **Frontend**: `./3dPrintDB.sh` (GUI required)
- **Docker**: `docker-compose -f Code/infra/docker-compose.yml up --build`
- **Reset DB**: `cd Code/backend && python reset_database.py --force`
- **Fix perms**: `./Code/fix_permissions.sh`
- **Format**: `cd Code/backend && black .`
- **Lint**: `cd Code/backend && flake8 .`
- **Type Check**: `cd Code/backend && mypy .`
- **Test**: `cd Code/backend && python -m pytest tests/ -v`
- **Quality**: `cd Code/backend && black . && flake8 . && mypy . && python -m pytest tests/ -v`

## Code Style & Conventions

### Python Standards
- **Version**: Python 3.11+ (FastAPI, SQLAlchemy, Pydantic, Tkinter)
- **Formatting**: Black (88-char lines, 4-space indent, trailing commas)
- **Linting**: flake8 (E9,F63,F7,F82,W503,W504 ignored)
- **Type Checking**: mypy (strict mode, no implicit optional)

### Import Organization
```
# Standard library imports
import os
import json
from typing import List, Optional

# Third-party imports
import requests
from fastapi import FastAPI
from sqlalchemy import Column, Integer, String

# Local imports
from . import crud, schemas
from .database import SessionLocal
```

### Naming Conventions
- **Functions/Variables**: snake_case (get_product, product_name)
- **Classes**: PascalCase (ProductCreate, DatabaseSession)
- **Constants**: UPPERCASE (API_URL, MAX_RETRIES)
- **Files**: snake_case (main.py, tag_utils.py)
- **Directories**: snake_case (backend/, frontend/)

### Type Hints
- Required on all public functions and methods
- Use Union/Optional for nullable types
- Prefer List/Dict over list/dict for collections
- Use pydantic models for API data structures

### Error Handling Patterns
- **Backend**: HTTPException for API errors, try/finally for DB sessions
- **Frontend**: messagebox for user errors, try/except for network calls
- **Database**: Session context managers, rollback on exceptions

### Database Patterns
- SQLAlchemy ORM only (no raw SQL)
- Session context managers: `with SessionLocal() as db:`
- Models inherit from Base, use declarative style
- Relationships: back_populates for bidirectional refs

### API Design
- RESTful endpoints with consistent naming
- Pydantic schemas with `from_attributes=True`
- Path/Query parameters with validation
- Proper HTTP status codes and error responses

### GUI Patterns
- ttk widgets for consistent theming
- Modal dialogs for complex operations
- Event binding with proper cleanup
- Threading for non-blocking operations
- Focus management and validation

### File Structure
```
Code/
├── backend/
│   ├── app/
│   │   ├── __init__.py
│   │   ├── main.py          # FastAPI app & routes
│   │   ├── models.py        # SQLAlchemy models
│   │   ├── schemas.py       # Pydantic schemas
│   │   ├── crud.py          # DB operations
│   │   ├── database.py      # DB connection & setup
│   │   └── tag_utils.py     # Tag processing logic
│   ├── requirements.txt
│   └── reset_database.py
├── frontend/
│   └── 3dPrintDB.py         # Main GUI application
└── infra/
    └── docker-compose.yml
```

### Documentation
- Google-style docstrings for public functions
- Include params, returns, raises sections
- README.md for user documentation
- Inline comments for complex logic only

### Testing Guidelines
- pytest for unit/integration tests
- Test database isolation (fixtures)
- Mock external dependencies
- Test both success and error cases
- GUI testing with tkinter test utilities

### Performance Considerations
- Database indexes on frequently queried fields
- Lazy loading for relationships
- Pagination for large result sets
- Connection pooling for database
- Efficient queries (select only needed columns)

### Security Best Practices
- Input validation on all user data
- SQL injection prevention (ORM handles this)
- No secrets in code (use environment variables)
- Proper error messages (no sensitive data leakage)
- File system operations with permission checks