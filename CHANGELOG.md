# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2](https://github.com/NLipatov/snake/compare/v0.1.1...v0.1.2) - 2026-04-10

### Fixed

- *(renderer)* handle grid resize correctly
- *(renderer)* recreate
- restore terminal state if cursor hiding
- allow moving into vacated tail cell and
- *(snake)* avoid needless borrow in occupies, add grid helper function index
- *(snake)* avoid needless borrow in occupies

### Other

- fix clippy warnings and
- format code
- fix typo in grid bounds panic message
- centralize grid dimensions in
- cover grid geometry and option-based grid
- remove render state entity
- allocation free renderer
- optimize snake occupancy and movement with VecDeque
- *(game)* change game tick to 115 ms
- *(renderer)* increased test coverage
- *(game)* unused method removed
- *(renderer)* implement cursor based rendering
- *(renderer)* each render forms a prev frame
- *(renderer)* introduce Frame and TerminalCell
- *(renderer)* run cargo fmt
- *(game)* 2x more food spawn probability
- *(renderer)* cover render state and terminal output helpers
- *(render)* update header formatting
- *(game)* introduce render state
- *(game)* extract terminal input handling into Terminal
- *(model)* replace tuple coordinates with Point
- *(grid)* introduce GridPoint

## [0.1.1](https://github.com/NLipatov/snake/compare/v0.1.0...v0.1.1) - 2026-04-04

### Added

- *(game)* add pause mode
- *(game)* add pause mode

### Fixed

- *(input)* only handle key events on press
- *(ci)* checkout repo before creating GitHub release
- *(ci)* resolve releases from latest merged release PR
- *(ci)* publish GitHub releases from release PR merges

### Other

- *(game)* add coverage for key event filtering and food spawning
- *(game)* simplify pause handling

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
