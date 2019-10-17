target := serica-os
mode := debug
kernel := target/$(target)/$(mode)/serica_os
bin := target/$(target)/$(mode)/serica_os.bin

.PHONY: all clean run build qemu kernel

all: build

build: $(bin)

run: build qemu-virt

test: build qemu-sifive

kernel:
	@cargo xbuild --target serica-os.json

$(bin): kernel
	@riscv64-unknown-elf-objcopy $(kernel) --strip-all -O binary $@

qemu-virt:
	qemu-system-riscv32 -M virt \
		-serial mon:stdio \
		-kernel opensbi/virt.elf \
		-device loader,file=$(bin),addr=0x80400000 \
		-device virtio-gpu-device \
		-device virtio-mouse-device

qemu-sifive:
	@qemu-system-riscv64 -M sifive_u -serial stdio \
		-kernel opensbi/sifive.elf \
		-device loader,file=$(bin),addr=0x80200000