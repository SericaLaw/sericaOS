target := serica_os
mode := debug
kernel := target/$(target)/$(mode)/serica_os
bin := target/$(target)/$(mode)/serica_os.bin
usr_path := usr/build/rust
export img = $(usr_path)/main

.PHONY: all clean run build qemu kernel asm

all: build

build: $(bin)

run: build qemu-virt

test: build qemu-sifive

kernel:
	@cargo xbuild --target serica_os.json

$(bin): kernel
	@riscv64-unknown-elf-objcopy $(kernel) --strip-all -O binary $@

asm:
	@riscv64-unknown-elf-objdump -d $(kernel) | less

qemu-virt:
	qemu-system-riscv32 -M virt \
		-serial mon:stdio \
		-kernel opensbi/virt.elf \
		-device loader,file=$(bin),addr=0x80400000 \
		-device virtio-gpu-device \
		-device virtio-mouse-device

# cannot run yet
qemu-sifive:
	@qemu-system-riscv64 -M sifive_u -serial stdio \
		-kernel opensbi/sifive.elf \
		-device loader,file=$(bin),addr=0x80200000