# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3](https://github.com/timvw/badass/compare/v0.1.2...v0.1.3) - 2025-11-24

### Added

- materialize instead of generating sql
- allow user to specify a specific model
- configure pre-commits
- extract model name from filename

### Fixed

- *(deps)* update rust crate minijinja to v2
- *(deps)* update all non-major dependencies
- rename struct field such that it's possible to override with an env variable
- *(test)* handle special case

### Other

- Fix config key mapping for query_engine
- Add settings to delete merged branches
- refactor into mods
- fxi test
- update revision of hook
- clippy
- lint

## [0.1.2](https://github.com/timvw/badass/compare/v0.1.1...v0.1.2) - 2024-03-22

### Added
- add command to display settings and documented how they can be overridden

### Other
- add test to verify that we can create a new (default) settings instance

## [0.1.1](https://github.com/timvw/badass/compare/v0.1.0...v0.1.1) - 2024-03-22

### Fixed
- *(deps)* update all non-major dependencies
- *(deps)* update rust crate clap to 4.5.3
- *(deps)* update rust crate minijinja to 1.0.15 ([#8](https://github.com/timvw/badass/pull/8))

### Other
- move renovate config to .github and add some rules to group upgrades
- Feature/show ([#11](https://github.com/timvw/badass/pull/11))

## [0.1.0](https://github.com/timvw/badass/releases/tag/v0.1.0) - 2024-03-22

### Added
- some progress on figure out what happens
- add tests for flattening vec of results
- move things around

### Other
- added release_plz
- added some logging
- added readme
- draft on creating tables
- make some changes such that settings are handled in a user friendly manner
- movethings around
- move flattening of errrs to infra as well
- Add renovate.json
- improve github workflow
- Feature/compile ([#1](https://github.com/timvw/badass/pull/1))
- Update README.md
- Initial commit
