# ShotRS

Simple screenshot utility written in Rust

## Usage

`ShotRS [OPTIONS]`

Options:

`-p, --path` - Path where to store screenshots [Default: home directory (~/)]

`-n, --name` - Name of the file [Default: shotrs-hh:mm:ss-dd.mm.yyyy-monitor:name.png]

`-h, --help` - Print help

`-V, --version` - Print version

## Build from source

### Dependencies

#### Ubuntu

 - `pkg-config`
 - `libclang-dev`
 - `libx11-dev`
 - `libxcb1`
 - `libxcb1-dev`
 - `libxrandr2`
 - `libxrandr-dev`
 - `libdbus-1-3`
 - `libdbus-1-dev`
 - `librust-x11-dev`
 - `libxtst-dev`
 - `libpipewire-0.3-dev`
 - `libwayland-dev`
 - `libegl-dev`
 - `libgbm-dev`

#### Fedora

 - `egl`
 - `gbm`
 - `clang`
 - `libxcb`
 - `libXrandr`
 - `dbus`
 - `libpipewire`
 - `mesa-libGL-devel`

### Instructions

#### Linux (x86-64)

```
git clone https://github.com/mssngtxtrs/ShotRS.git
cd ShotRS
cargo build -r --target x86_64-unknown-linux-gnu
```
