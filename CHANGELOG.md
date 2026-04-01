# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/NLipatov/snake/releases/tag/v0.1.0) - 2026-04-01

### Added

- add colored terminal rendering for snake and food
- display score after game over
- snake game implementation

### Fixed

- *(ci)* handle no-op release-plz runs
- satisfy clippy warnings
- prevent food from spawning on the snake
- correct terminal rendering aspect ratio
- clear consumed food from grid

### Other

- rename release token secret
- add coverage and release workflows
- expand rust test coverage
- add coverage for grid, game, and renderer
- add snake tests
- simplify terminal renderer color handling
- introduce RenderCell in renderer
- return a slice instead of Vec
- update readme
- update .gitignore
- drop unused regex dependency
- add readme
- add LICENSE
