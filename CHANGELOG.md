# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.4.1] - 2023-03-14

### Fixes

- Pass `f.alternate()` down to the nodes

## [0.4.0] - 2022-04-07

### Compatibility

- Removed functions in place of direct member access

### Features

- Directly expose `root` and `leaves`

## [0.3.0] - 2022-04-07

### Compatibility

- MSRV bumped to 1.54.0
- `Tree::new` removed
- `Tree::root` renamed to `Tree::new`

### Features

- Accessors for roots and leaves
- Shortcuts for building up a `Tree`

## [0.2.4] - 2022-01-07

#### Fixes

- Adjusted to a cleaner looking continuous middle skip

## [0.2.3] - 2021-10-26

#### Features

- Provide customization of tree glyphs

## [0.2.2] - 2021-10-22

#### Fixes

- Do not overflow the stack

## [0.2.1] - 2021-10-07

#### Features

- Add leaves via `Extend::extend`
- Opt-in multi-line item support

## [0.2.0] - 2021-10-07

Forked from `treeline`

## 0.1.0

* initial release

<!-- next-url -->
[Unreleased]: https://github.com/rust-cli/termtree/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/rust-cli/termtree/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/rust-cli/termtree/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/rust-cli/termtree/compare/v0.2.4...v0.3.0
[0.2.4]: https://github.com/rust-cli/termtree/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/rust-cli/termtree/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/rust-cli/termtree/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/rust-cli/termtree/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/assert-rs/assert_cmd/compare/v0.1.0...v0.2.0
