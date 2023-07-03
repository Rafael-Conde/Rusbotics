# Rusbotics

  This is a repo for my undergraduate thesis, which is meant to be a sofware to help analyse and solve industrial robotics problems.

  this software was inpired by toolboxes inside the MatLab enviroment, such as the one developed by the Professor Peter Cork.
  
  The aim of this software though is to provid a Rust library for this robotics problems and a intuitive and high performant softare o solve such problems.

## Disclaimer

This is still work in progress, currently experimenting and changing the library interface. Once a release tag is commited, then I'll commit to the stated interface.

## Requirements

To build this software yourself you'll need the rust stable toolchain, which can be easely obtained from the [rustup installer](https://rustup.rs/).

To make the process seamless, one could also intall [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg), which could be a simple command:

```shell
cargo install cargo-vcpkg
```
On windows, MSYS2 will simplify the process overall too.

One could then use the `*.sh` files to execute the desired task. Otherwise, one could use them as reference to perform other tasks.

# External Dependencies

in order for the software to run, one will need pdfium and python 3.10 dynamic libraries(.dll on windows and .so on linux) in the same directory as the executable.

# Issues

Currently, this software uses Pyo3 to run an embedded Python interpreter inside the program to perform symbolic calculations, the issue with this is that in order to run the desired sympy code, the library needs to be installed. I found not trivial to install the library in the embedded python interpreter using pyo3, so this far, in order for the software to run, the user actually still needs an independent python installation. This is here to state that I'm aware of such issue and I'm actively trying to solve this, but I wasn't successful this far.






