# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## Unreleased

### Changed

- Main function returns a proper `Result`.

### Fixed

- Fixed incorrect rendering of large negative integer labels.


## [0.2.0] - 2024-10-26

### Added

- Add an extra digit of one of the axis labels would be jumping around too much
  due to rounding.
- Set up CI via GitHub actions.
- Added `padding_left` option to add space on the lft.
- Added `unit` option to append a string with the unit to each axis label.
- Added `vertical_direction` option.

### Fixed

- Incorrect offset for longer labels.


## [0.1.0] - 2024-09-28

### Added

- Initial translation of the uniplot Python source to Rust.
