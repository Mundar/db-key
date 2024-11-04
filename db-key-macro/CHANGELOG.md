# Change Log

## [Unreleased]

## [0.1.1] - 2024-11-03

### Added

- Added default key documentation if none is supplied. Adds "The
    [KeyStructName] structure." to generated keys if no documentation is
    supplied. This mainly prevents "Examples" from being listed as the short
    description for key structures that have no name.

### Fixed

- The field documentation was not being propagated to the generated args
    structure when using `#[db_key]` macro, so when `#![warn(missing_docs)]`
    was enabled, there was no way to disable the warning for the missing field
    documentation.
