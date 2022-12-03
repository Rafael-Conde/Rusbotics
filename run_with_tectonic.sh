export VCPKG_ROOT="${CARGO_TARGET_DIR:-$(pwd)/target}/vcpkg"
export RUSTFLAGS='-Ctarget-feature=+crt-static'  # Windows only
# export VCPKGRS_TRIPLET='x64-windows-static-release'  # Windows only
export TECTONIC_DEP_BACKEND=vcpkg
cargo vcpkg build
cargo run --release #--features external-harfbuzz
