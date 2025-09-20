# ShotRS

Simple screenshot utility written in Rust

## Usage

`ShotRS [OPTIONS]`

Options:

`-p, --path` - Path where to store screenshots [Default: home directory (~/)]

`-n, --name` - Name of the file [Default: hh:mm:ss-dd.mm.yyyy]

`-h, --help` - Print help

`-V, --version` - Print version

## Build from source

### Dependencies

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
