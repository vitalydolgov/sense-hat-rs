# Sense Hat library for Rust

A Rust library for interacting with [Sense Hat](https://www.raspberrypi.com/products/sense-hat/), a Raspberry Pi add-on board. This is work in progress, I have no intention to finish the library, but still maybe there is some benefit for you in it.

## Building

Current repo includes a submodule of RTIMULib, the following document assumes that the source of this library is located in the project directory:
```
> git submodule update --init
```

Building consists of two phases:
1. build original C++ library with custom C bindings
1. build current library linking against RTIMULib

### Build original C++ library

First, we need to add custom C wrapper, as Rust cannot use C++ code directly, patch RTIMULib with the following script:
```
> ./RTIMULib-c/patch.sh
```

Then build RTIMULib, for details see its repo:
```
> cd RTIMULib/RTIMULib
> mkdir build
> cd build
> cmake ..
> make
```

Export path to shared library, later you need it for running (assuming you are still at `build` directory):
```
bash> export LD_LIBRARY_PATH=$PWD
fish> set -x LD_LIBRARY_PATH $PWD
```

### Build library and run example

See `build.rs` if you need to change paths.

```
> cargo build
> cargo run
```

If you didn't export `LD_LIBRARY_PATH`, set it for particular command on running:
```
> LD_LIBRARY_PATH=RTIMULib/RTIMULib/build cargo run
```

## What's covered

- LED matrix: set pixel, clear matrix
- Humidity sensor (reads humidity and temperature)
- Pressure sensor (reads pressure and temperature)
