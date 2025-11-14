# video_decoders

**Type**: core
**Tech Stack**: Rust, openh264, dav1d, rav1e, vpx
**Version**: 0.1.0

## Responsibility

Video codec implementations (H.264, H.265, VP8, VP9, AV1)

## Structure

```
├── src/           # Source code
├── tests/         # Tests (unit, integration)
├── benches/       # Performance benchmarks
├── Cargo.toml     # Rust package configuration
├── CLAUDE.md      # Component-specific instructions for Claude Code
└── README.md      # This file
```

## Usage

This component is ready for implementation via Task tool orchestration.

**Through Orchestrator:**
The orchestrator will launch an agent using the Task tool to implement this component.

**Direct Work:**
```bash
cd components/video_decoders
claude code
# Claude Code reads local CLAUDE.md and you work directly
```

## Development

See CLAUDE.md for detailed development instructions, quality standards, and TDD requirements.

## Testing

```bash
# Run tests
cargo test

# Run with coverage
cargo tarpaulin --out Html

# Run benchmarks
cargo bench
```

## Dependencies

Dependencies are defined in `Cargo.toml` and will be added during implementation based on requirements specified in `../../docs/ARCHITECTURE.md`.
