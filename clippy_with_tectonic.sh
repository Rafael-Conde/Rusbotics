# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
# Copyright (c) 2023 Rafael de Conde Reis. All rights reserved.

# cargo vcpkg build
export VCPKG_ROOT="${CARGO_TARGET_DIR:-$(pwd)/target}/vcpkg"
export RUSTFLAGS='-Ctarget-feature=+crt-static'  # Windows only
# export VCPKGRS_TRIPLET='x64-windows-static-release'  # Windows only
export TECTONIC_DEP_BACKEND=vcpkg
cargo vcpkg build
cargo clippy
