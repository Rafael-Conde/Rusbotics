def main [compile_mode?: string] {
  with-env {CARGO_TARGET_DIR: $"($env.PWD)/target"} {
    with-env {RUSTC_WRAPPER : "sccache", , VCPKG_ROOT: $"($env.CARGO_TARGET_DIR)/vcpkg", RUSTFLAGS: "-Ctarget-feature=+crt-static", TECTONIC_DEP_BACKEND: "vcpkg"} {
      cargo vcpkg build
      if ($compile_mode == null) {
        cargo build
      } else {
        cargo build $compile_mode
      }
    }
  }
}
