# Zeon GN Build System

## Overview

Zeon uses the [GN (Generate Ninja)](https://gn.googlesource.com/gn) build system for fast, scalable builds.

## Requirements

```bash
# Install GN
git clone https://gn.googlesource.com/gn
cd gn
python3 build.py
sudo cp out/gn /usr/local/bin/
```

## Building with GN

```bash
# Generate build files
gn gen out/gn

# Build all targets
ninja -C out/gn

# Build specific target
ninja -C out/gn zeon

# Build with release config
gn gen out/gn --args="is_debug=false"
ninja -C out/gn
```

## Build Targets

| Target | Description |
|--------|-------------|
| `zeon` | Main kernel binary |
| `zeon_tests` | Unit tests |
| `usertest` | User-space tests |
| `libkernel` | Core library |
| `zeon-macros` | Procedural macros |

## Directory Structure

```
.
├── .gn              # GN configuration
├── BUILD.gn         # Root build file
├── build/
│   ├── config/      # Build configuration
│   └── toolchain/   # Toolchain definitions
├── libkernel/       # Core library (GN build)
└── zeon-macros/    # Macros (GN build)
```

## Configuration Options

| Arg | Default | Description |
|-----|---------|-------------|
| `is_debug` | true | Debug build |
| `target_arch` | aarch64 | Target architecture |
| `rustc_target` | aarch64-unknown-none-softfloat | Rust target |
| `enable_tests` | false | Enable tests |