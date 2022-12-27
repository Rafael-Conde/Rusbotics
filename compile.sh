SECONDS=0 
echo generating artifacts
pyoxidizer generate-python-embedding-artifacts pyembedded
cargo vcpkg build
export VCPKG_ROOT="${CARGO_TARGET_DIR:-$(pwd)/target}/vcpkg"
export RUSTFLAGS='-Ctarget-feature=+crt-static'  # Windows only
# export VCPKGRS_TRIPLET='x64-windows-static-release'  # Windows only
export TECTONIC_DEP_BACKEND=vcpkg
cargo vcpkg build
echo compiling
PYO3_CONFIG_FILE=$(pwd)/pyembedded/pyo3-build-config-file.txt PYTHONPATH=pyembedded/stdlib cargo build --release
echo $SECONDS
