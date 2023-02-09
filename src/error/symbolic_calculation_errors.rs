This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
Copyright (c) 2023 Rafael de Conde Reis. All rights reserved.

#![warn(clippy::all,
clippy::pedantic,
clippy::perf,
clippy::nursery,
// clippy::cargo,
clippy::unwrap_used,
clippy::expect_used)]

#[derive(Debug)]
pub enum SymbolicCalculationError {
    InvalidSymbolicOperation,
    InvalidSymbolInEquation,
    InternalSymbolicExtractionFailed,
}
