# kernel
Flandre OS Microkernel


Prepare:
```bash
rustup component add llvm-tools-preview [--toolchain nightly]
cargo install cargo-xbuild
cargo install bootimage
sudo apt install qemu qemu-system
```

Build:
```bash
bootimage build
```

Run:
```bash
bootimage run
qemu-system-x86_64 -drive format=raw,file=target/x86_64-flandre_os/debug/bootimage-flandre_os_kernel.bin
```

