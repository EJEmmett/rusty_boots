# Rusty Boots

A small x86_64 hobby kernel written in Rust, originally built as a student project
to learn how an operating system gets from power-on to running code. It is published
as-is and is not actively maintained.

### Features
- Boots through [`bootloader`](https://crates.io/crates/bootloader) on a custom `no_std` target
- GDT and TSS with dedicated stacks for double faults and stack overflows
- Hand-written Interrupt Descriptor Table with CPU exception handlers
- Timer and keyboard interrupts through the chained 8259 PICs
- VGA text-mode output with colored `cprint!` / `cprintln!` macros, plus serial output
- Real-time clock reads from CMOS (prints the boot time)
- 4-level paging with a bitmap physical frame allocator that can free and reuse frames
- 128 KiB kernel heap with a buddy allocator that merges freed blocks back together
- Cooperative async executor with an async keyboard input task

`src/showcases` holds small demos (page faults, double faults, address translation,
heap allocation, async tasks). Uncomment them in `src/main.rs` to try them.

### Requirements
> - Rust `nightly-2021-09-29` (pinned by `rust-toolchain.toml`, installed automatically)
> - bootimage 0.10.3 (newer versions fail with this nightly; `make setup` installs it)
> - QEMU and/or Bochs

Install `bootimage` with:
```shell
$ make setup
```

On Debian/Ubuntu, install the emulators with `make setup-qemu` and/or `make setup-bochs`
(both use `sudo apt-get`).

### Run
| Command      | Description                                     |
|--------------|-------------------------------------------------|
| `make build` | Build the bootable disk image                   |
| `make run`   | Boot in QEMU (GDB server on `:1234`)            |
| `make debug` | Boot in QEMU, paused until GDB attaches         |
| `make bochs` | Boot in Bochs                                   |
| `make clean` | Remove build artifacts                          |

Pass `PROFILE=release` for a release build. Emulator paths can be overridden,
e.g. `make run QEMU=/path/to/qemu-system-x86_64`.

### Debugging
Start the kernel with `make debug`, then attach GDB from another terminal:
```shell
$ gdb target/x86_64-rusty_boots/debug/rusty_boots -ex "target remote :1234"
```

### Layout
| Path                 | Contents                                         |
|----------------------|--------------------------------------------------|
| `src/gdt.rs`         | GDT, TSS and interrupt stacks                    |
| `src/interrupts/`    | IDT, exception and hardware interrupt handlers   |
| `src/memory/paging/` | Page tables and the bitmap frame allocator       |
| `src/memory/kalloc/` | Kernel heap and the buddy allocator              |
| `src/multitasking/`  | Async executor and keyboard task                 |
| `src/vga/`           | VGA text-mode writer and print macros            |
| `src/serial.rs`      | Serial port output                               |
| `src/time/`          | CMOS real-time clock                             |
| `src/showcases/`     | Demo functions                                   |

Known issues and unfinished ideas are tracked in [`TODO.md`](TODO.md).

### Credits
Much of this kernel's structure follows Philipp Oppermann's excellent
[Writing an OS in Rust](https://os.phil-opp.com/) series
([blog_os](https://github.com/phil-opp/blog_os)). Other references used along the way:
- [AMD64 Architecture Programmer's Manual, Volume 2](https://www.amd.com/system/files/TechDocs/24593.pdf)
- [MIT 6.828: Intel 80386 Reference, Chapter 9 (Exceptions and Interrupts)](https://pdos.csail.mit.edu/6.828/2008/readings/i386/c09.htm)
- [OSDev Wiki: Exceptions](https://wiki.osdev.org/Exceptions)

### License
Released under the [MIT License](LICENSE). Use it however you like.
Parts derived from [blog_os](https://github.com/phil-opp/blog_os) remain
Copyright (c) 2019 Philipp Oppermann, also under the MIT License.
