CURDIR := $(shell pwd)
RUST_TARGET := riscv64gc-unknown-none-elf

BOOTLOADER_DIR := $(CURDIR)/../../bootloader
TARGET_DIR := $(CURDIR)/target

QEMU_BOOTLOADER := $(BOOTLOADER_DIR)/rustsbi-qemu.bin

OS_OUTPUT := $(TARGET_DIR)/$(RUST_TARGET)/release/os
OS_BIN_OUTPUT := $(TARGET_DIR)/$(RUST_TARGET)/release/os.bin

build: $(OS_OUTPUT) $(OS_BIN_OUTPUT)

# If you want to terminate the QEMU, please
# press `ctrl + A, X`.
qemu-gdb: clean $(OS_OUTPUT) $(OS_BIN_OUTPUT)
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(QEMU_BOOTLOADER) \
		-device loader,file=$(OS_BIN_OUTPUT),addr=0x80200000 \
		-s -S

qemu: clean $(OS_OUTPUT) $(OS_BIN_OUTPUT)
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(QEMU_BOOTLOADER) \
		-device loader,file=$(OS_BIN_OUTPUT),addr=0x80200000

gdb: clean $(OS_OUTPUT)
	@riscv64-unknown-elf-gdb \
		-ex "file $(OS_OUTPUT)" \
		-ex "set arch riscv:rv64" \
		-ex "target remote localhost:1234"

.DEFAULT: default

$(OS_OUTPUT):
	@cargo build --release

$(OS_BIN_OUTPUT): $(OS_OUTPUT)
	@rust-objcopy --strip-all $< -O binary $@

.PHONY: \
	clean

clean: 
	@rm -rf $(TARGET_DIR)
