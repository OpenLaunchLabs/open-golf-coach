# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-01-10

### Added

- Initial release of OpenGolfCoach Python bindings
- `calculate_derived_values()` function for processing golf shot data
- Trajectory calculation with carry, total distance, and offline distance
- Spin component conversion (total spin/axis <-> backspin/sidespin)
- Shot classification with names (Straight, Draw, Fade, etc.) and quality rankings (S+, S, A, B, C, D, E)
- US customary unit conversions (mph, yards)
- Clubhead speed estimation and smash factor calculation
- Club path and face angle estimation
- Landing position and velocity vectors
- Descent angle calculation
- Support for Python 3.8, 3.9, 3.10, 3.11, 3.12, and 3.13
- Pre-built wheels for:
  - Linux (x86_64, aarch64)
  - macOS (x86_64, arm64/Apple Silicon)
  - Windows (x64)
- Type stubs for IDE support and type checking
- Comprehensive test suite with pytest

[Unreleased]: https://github.com/OpenLaunchLabs/open-golf-coach/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/OpenLaunchLabs/open-golf-coach/releases/tag/v0.1.0
