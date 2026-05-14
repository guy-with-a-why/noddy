# NODDY
**Not Obviously Doing Diddly Yet**

A bare-metal x86_64 operating system kernel written in Rust. Built from scratch as a learning project - no standard library, no safety net.

---

## Prerequisites

### 1. Rust (Nightly)

The `rust-toolchain.toml` in this repo automatically selects the correct nightly toolchain. Install `rustup` if you haven't already:

```sh
# macOS / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows - download rustup-init.exe from https://rustup.rs
```

Then install the required components:

```sh
rustup component add rust-src --toolchain nightly
rustup component add llvm-tools-preview --toolchain nightly
```

### 2. bootimage

`bootimage` is a Cargo extension that links the kernel with a bootloader and produces a bootable disk image:

```sh
cargo install bootimage
```

### 3. QEMU

QEMU emulates x86_64 hardware so you can run the kernel without rebooting a real machine.

**macOS:**
```sh
brew install qemu
```

**Ubuntu / Debian:**
```sh
sudo apt install qemu-system-x86
```

**Windows:**
Download the installer from https://www.qemu.org/download/#windows and add the install directory to your `PATH`.

---

## Build

```sh
cargo bootimage
```

This compiles the kernel and bundles it with a bootloader, producing a bootable raw disk image at:

```
target/x86_64-unknown-none/debug/bootimage-noddy-os.bin
```

---

## Run

```sh
cargo run
```

This builds the image and launches it in QEMU automatically (via the `bootimage runner` configured in `.cargo/config.toml`).

To run the image directly with QEMU:

```sh
# Linux / macOS
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-noddy-os.bin

# Windows (PowerShell)
qemu-system-x86_64 -drive format=raw,file=target\x86_64-unknown-none\debug\bootimage-noddy-os.bin
```

Useful QEMU flags:

| Flag | Effect |
|------|--------|
| `-no-reboot` | Halt instead of rebooting on triple fault |
| `-no-shutdown` | Keep the window open after shutdown |
| `-serial stdio` | Pipe the serial port to your terminal |
| `-d int` | Log CPU interrupts to stdout (debugging) |

Example with all of the above:

```sh
qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-noddy-os.bin \
  -no-reboot -no-shutdown \
  -serial stdio
```

---

## Project Structure

```
panic/
├── .cargo/
│   └── config.toml      # Build target (x86_64-unknown-none) and QEMU runner
├── src/
│   ├── main.rs          # Kernel entry point (_start) and panic handler
│   └── vga.rs           # VGA text buffer - write characters to the screen
├── Cargo.toml           # Dependencies and panic=abort profile settings
├── rust-toolchain.toml  # Pins nightly + rust-src + llvm-tools-preview
└── README.md
```

---

## How It Works

The kernel uses `#![no_std]` and `#![no_main]` - there is no Rust standard library and no OS to hand execution to `main`. Instead:

1. The `bootloader` crate provides a stage-1 bootloader that switches the CPU into 64-bit long mode, sets up a basic page table, and calls `_start`.
2. `_start` is our kernel entry point. It runs in ring 0 (kernel mode) with no heap, no threads, and no safety net.
3. Output goes directly to the VGA text buffer at physical address `0xb8000` - 80×25 cells of `[character, color]` byte pairs.

---

## Roadmap

- [x] Boot to 64-bit long mode
- [x] VGA text output
- [ ] VGA cursor tracking and scrolling
- [ ] Global Descriptor Table (GDT)
- [ ] Interrupt Descriptor Table (IDT) and CPU exception handlers
- [ ] Hardware interrupts - PIC, timer, keyboard
- [ ] Physical memory manager (frame allocator)
- [ ] Virtual memory and paging
- [ ] Kernel heap allocator
- [ ] Process scheduler
- [ ] Simple file system
