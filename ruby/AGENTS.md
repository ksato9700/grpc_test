# Ruby Agent Development Guide

## Coding Style & Conventions

This project uses [StandardRB](https://github.com/standardrb/standard) for linting and formatting. AI Agents should adhere to these rules when generating code to minimize churn.

### Formatting Rules
- **Quotes**: Use **double quotes** (`"`) for all string literals, unless the string contains double quotes (then use single quotes or `%Q`).
- **Indentation**: Use **2 spaces** for indentation. No tabs.
- **Hash Syntax**: Use the new 1.9 hash syntax (`{ key: "value" }`) where possible.
- **Trailing Commas**: Avoid trailing commas in argument lists or arrays unless multiline (StandardRB rules apply).

### Commands
- **Lint**: `make lint`
- **Format**: `make fmt` (Run this if you are unsure)
