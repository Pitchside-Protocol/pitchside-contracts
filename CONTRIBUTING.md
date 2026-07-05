# Contributing

Thank you for your interest in Pitchside! We welcome contributions from the community.

## Getting Started

1. Fork the repository.
2. Create a branch:
   - `feature/*` for new features
   - `fix/*` for bug fixes
3. Make your changes.
4. Ensure the following pass locally:
   - `cargo fmt --check`
   - `cargo clippy -- -D warnings`
   - `cargo test`
5. Open a pull request against `main`.

## Commit Style

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>
```

Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`, `ci`.

Examples:
- `feat(match_result): add settle function`
- `fix(over_under): correct payout calculation`
- `docs(readme): update deployment section`

## PR Checklist

- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt --check`)
- [ ] Clippy is clean (`cargo clippy -- -D warnings`)
- [ ] PR references an existing issue

## Issue-First Policy

Please open an issue before starting work on any significant change. This allows the community to discuss the design and avoid duplicated effort.
