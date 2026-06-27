# Security Policy

`knifer-rs` is a Safe Rust utility toolkit. The crate forbids `unsafe` code
through Cargo lints and keeps the core dependency-light by default.

## Supported Versions

Security fixes are applied to the active development line until the project
publishes a stable release policy.

| Version | Supported |
| --- | --- |
| `0.1.x` | Yes |

## Reporting a Vulnerability

Do not publish exploit details in public issues before maintainers have had a
chance to triage. Report vulnerabilities privately to the project maintainers
or through the hosting platform's private vulnerability reporting channel when
available.

Please include:

- affected API or module
- minimal reproduction
- expected impact
- Rust version and target platform

## Security Bar

Changes that affect parsing, escaping, path matching, file handling, crypto, or
network-facing helpers must include tests for invalid input and boundary cases.
Future dependencies must be justified in code review and covered by dependency
audit tooling before release.
