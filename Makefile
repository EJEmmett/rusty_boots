# Override on the command line, e.g. `make run QEMU=/path/to/qemu-system-x86_64`
CARGO   ?= cargo
QEMU    ?= qemu-system-x86_64
BOCHS   ?= bochs
PROFILE ?= debug

TARGET := x86_64-rusty_boots
IMAGE  := target/$(TARGET)/$(PROFILE)/bootimage-rusty_boots.bin

CARGO_FLAGS := $(if $(filter release,$(PROFILE)),--release,)
# QEMU >= 7.1 reserves memory near 1 TiB for AMD CPUs (the default qemu64 model reports AMD),
# which trips bootloader 0.9.19's 512 GiB physical memory limit. Report an Intel CPU instead.
QEMU_FLAGS  := -cpu qemu64,vendor=GenuineIntel -serial stdio -drive format=raw,file=$(IMAGE)

.PHONY: all build run debug bochs setup setup-qemu setup-bochs clean

all: build

## build: Build the bootable disk image
build:
	$(CARGO) bootimage $(CARGO_FLAGS)

## run: Boot the image in QEMU (GDB server on :1234)
run: build
	"$(QEMU)" $(QEMU_FLAGS) -s

## debug: Boot in QEMU paused until GDB attaches on :1234
debug: build
	"$(QEMU)" $(QEMU_FLAGS) -s -S

## bochs: Boot the image in Bochs using bochs.bxrc
bochs: build
	@# Bochs leaves a stale lock behind if it's killed rather than quit, and then refuses the image
	rm -f $(IMAGE).lock
	"$(BOCHS)" -q -f bochs.bxrc 'ata0-master: type=disk, path=$(IMAGE), mode=flat'

## setup: Install the tools needed to build the image
setup:
	rustup show active-toolchain
	$(CARGO) install bootimage --version 0.10.3 --locked

## setup-qemu: Install QEMU (Debian/Ubuntu, needs sudo)
setup-qemu:
	sudo apt-get install -y qemu-system-x86

## setup-bochs: Install Bochs with its BIOS images and X11 display (Debian/Ubuntu, needs sudo)
setup-bochs:
	sudo apt-get install -y bochs bochsbios vgabios bochs-x

## clean: Remove build artifacts
clean:
	$(CARGO) clean
