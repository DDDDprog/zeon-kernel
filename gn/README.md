# Zeon GN Build System

Advanced modular GN build system for the Zeon Operating System.

## Structure

```
gn/
├── BUILD.gn          # Main build file
├── arch/             # Architecture-specific builds
│   └── arm64/        # ARM64 architecture
├── config/           # Build configurations
├── drivers/          # Driver modules
├── fs/               # Filesystem modules
├── net/              # Networking modules
└── platform/         # Platform-specific builds
```

## Building

```bash
# Install GN if needed
git clone https://gn.googlesource.com/gn
cd gn && python3 build.py && sudo cp out/gn /usr/local/bin/

# Generate build
gn gen out/gn -q

# Build kernel
ninja -C out/gn

# Build specific targets
ninja -C out/gn gn:zeon
ninja -C out/gn gn:usertest
ninja -C out/gn gn:zeon_tests
```

## Modules

| Module | Description |
|--------|-------------|
| `gn:arch:arm64` | ARM64 CPU, memory, exceptions, boot, proc |
| `gn:drivers` | Block, char, net, video, audio, USB, I2C, SPI |
| `gn:fs` | VFS, tmpfs, fat32, ext4, proc, devfs, sysfs |
| `gn:net` | TCP/IP, Unix sockets, VirtIO networking |
| `gn:platform` | QEMU, Raspberry Pi, VirtIO, FDT |

## Features

- Modular architecture
- Conditional compilation via feature flags
- Platform-specific builds (QEMU, RPi, VirtIO)
- Driver abstraction
- Filesystem plugins
- Network stack