# Change Log

## [Unreleased]

## [0.2.0] - 2024-12-17

### Added

- Added support for using signed integer values as struture fields. The signed
    value is written into the array with the sign bit toggled so that the order
    of the key array is preserved.
- Added MIN_KEY constant and added ability to set the minimum and maximum value
    for each field. The minimum and maximum values are informational only. They
    define the MIN_KEY and MAX_KEY, and crate field constants for the default,
    minimum and maximum values are created. They are not checked against each
    other (i.e. the minimum can be greater than the maximum, and the default
    value doesn't have to be in range). A future change may add bounds
    checking.
- Added option to not generate the MIN_KEY and MAX_KEY constants with the
    `no_min` and `no_max` options.
- Added better support for integers when dealing with values passed as string
    literals. For example, `#[default="u32::MAX"]` was not supported, but it is
    now.

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
