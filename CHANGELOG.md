# Changelog - dustpkg (DPL Package Manager)

All notable changes to dustpkg are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-02-12 (DPL v0.2)

### Added

- **DPL v0.2 Compliance**: Full support for v0.2 specification
- Workspace support for multi-sector projects
- Dependency resolution for dustlib and dustlib_k
- Version management with semantic versioning
- Lock file support (dustpkg.lock)
- Target-specific dependencies
- Cross-platform build support
- DPL version field in package manifest (v0.1 or v0.2)
- Automatic dustlib_k dependency for v0.2 projects
- Registry integration (future)

### Changed

- Updated project structure support (Dust.toml)
- Improved dependency resolution algorithm
- Better error messages for conflicts
- Default DPL version is now 0.2

### Fixed

- Dependency resolution edge cases
- Version constraint handling

## [0.1.0] - 2026-02-12

### Added

- Initial package manager
- Basic dependency management
- Project manifest parsing (Dust.toml)
- Simple build integration

### Known Issues

- Limited to basic dependency resolution

---

Copyright © 2026 Dust LLC