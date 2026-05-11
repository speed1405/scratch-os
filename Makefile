KERNEL := target/x86_64-unknown-none/debug/scratch-os
ISO := target/scratch-os.iso

.PHONY: all clean run iso

all: $(KERNEL)

$(KERNEL): src/*.rs src/*.S src/linker.ld Cargo.toml x86_64-unknown-none.json
	cargo build -Zbuild-std=core,compiler_builtins --target x86_64-unknown-none.json -Zjson-target-spec

iso: $(ISO)

$(ISO): $(KERNEL) src/grub.cfg
	mkdir -p target/isofiles/boot/grub
	cp $(KERNEL) target/isofiles/boot/kernel.bin
	cp src/grub.cfg target/isofiles/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO) target/isofiles

run: $(ISO)
	qemu-system-x86_64 -cdrom $(ISO)

clean:
	cargo clean
	rm -rf target/isofiles
