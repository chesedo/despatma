# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.6](https://github.com/chesedo/despatma/compare/despatma-dependency-container-v0.3.5...despatma-dependency-container-v0.3.6) - 2025-03-07

### Other

- dependencies updates ([#36](https://github.com/chesedo/despatma/pull/36))

## [0.3.5](https://github.com/chesedo/despatma/compare/despatma-dependency-container-v0.3.4...despatma-dependency-container-v0.3.5) - 2024-11-06

### Added

- *(di)* Make containers clonable ([#33](https://github.com/chesedo/despatma/pull/33))

### Other

- *(di)* reduce lifetime uses ([#35](https://github.com/chesedo/despatma/pull/35))

## [0.3.4](https://github.com/chesedo/despatma/compare/despatma-dependency-container-v0.3.3...despatma-dependency-container-v0.3.4) - 2024-09-24

### Added

- *(di)* nested impl traits ([#31](https://github.com/chesedo/despatma/pull/31))

### Other

- *(di)* `impl Trait` in generics not fixed correctly ([#30](https://github.com/chesedo/despatma/pull/30))
- *(di)* don't generate a private create method ([#29](https://github.com/chesedo/despatma/pull/29))
- *(di)* always have a generic lifetime ([#28](https://github.com/chesedo/despatma/pull/28))
- *(di)* incorrectly using async_once_cell on async call tree ([#26](https://github.com/chesedo/despatma/pull/26))

## [0.3.3](https://github.com/chesedo/despatma/compare/despatma-dependency-container-v0.3.2...despatma-dependency-container-v0.3.3) - 2024-09-18

### Added

- *(di)* Lifetimes support ([#20](https://github.com/chesedo/despatma/pull/20))

## [0.3.2](https://github.com/chesedo/despatma/compare/despatma-dependency-container-v0.3.1...despatma-dependency-container-v0.3.2) - 2024-08-16

### Other
- *(di)* handle trailing commas correctly ([#18](https://github.com/chesedo/despatma/pull/18))

## [0.3.1](https://github.com/chesedo/despatma/compare/despatma-dependency-container-v0.3.0...despatma-dependency-container-v0.3.1) - 2024-08-16

### Other
- *(di)* improve error shown when types don't match ([#17](https://github.com/chesedo/despatma/pull/17))

## [0.3.0](https://github.com/chesedo/despatma/compare/despatma-dependency-container-v0.2.0...despatma-dependency-container-v0.3.0) - 2024-08-15

### Other
- *(di)* generate the di visitor using our own macro ([#14](https://github.com/chesedo/despatma/pull/14))
