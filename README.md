# Potato CHIP

A minimal CHIP-8 VM

### Installation

#### Pre-built binaries
Download the appropriate binary from the latest release and extract it.

For example, on Linux you might do:
```bash
curl -o potato.tar.gz https://github.com/AnActualEmerald/potato/releases/download/v0.1.0/potato_x86_64_gnu_linux.tar.gz
tar xzfv potato.tar.gz potato
```

Optionally move the binary to a location in your PATH:
```bash
sudo mv potato/potato /usr/bin/potato
```

#### From source
Install [Rust](https://rustup.rs) if you don't already have it on your system.

The simplest way to build the project is using `cargo` to install directly from the git repo:
```bash
cargo install --git https://github.com/AnActualEmerald/potato
```

Or you can manually clone the repo and build it locally:
```bash
git clone https://github.com/AnActualEmerald/potato
cd potato
cargo build --release
# Or run the project directly with cargo run
cargo run -- roms/test_opcode.ch8
```

### Usage
```bash
potato /path/to/rom/file
```

### Running the tests

To run both the IBM logo test and [Corax89's test ROM](https://github.com/corax89/chip8-test-rom): 

```bash
git clone https://github.com/AnActualEmerald/potato
cd potato
make test-all
```

