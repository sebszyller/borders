# Agent Instructions for ig-resize

## Build Commands
- Run script: `python3 resize.py *.jpg`
- Build all: `make all`
- Clean up: `make clean`

## Code Style Guidelines

### Python Style
- Type hints: Use explicit type annotations (e.g., `def main(args: argparse.Namespace):`)
- Imports: Group standard library imports first, then third-party, then local
- Functions: Use snake_case for function names
- Error handling: Use assertions for validation, exit() for fatal errors
- Constants: Define constants at module level when appropriate

### Formatting
- Follow PEP 8 conventions
- Use 4-space indentation
- Maximum line length should be 88 characters (Black default)

### Testing
- No explicit test framework found, but validate correct functionality by:
  - Testing with sample images
  - Checking output dimensions match the expected ratio
  - Verifying ImageMagick dependency is available (`which magick`)


## AGENT Guidelines

- Use `AGENT:` or `AGENT-QUESTION:` (all-caps prefix) for comments aimed at AI and developers.
- Keep them concise (< 120 chars).
- **Update relevant anchors** when modifying associated code.
- **Do not remove `AGENT*` notes** without explicit human instruction.

Example:
# AGENT: perf-hot-path; avoid extra allocations (see ISSUE-1234)
