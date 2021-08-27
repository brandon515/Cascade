arch ?= x86_64# if the architecture hasn't been set, set it to x86_64
kernel := build/Cascade-$(arch).bin
iso := build/Cascade-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
asm_src_files := $(wildcard src/arch/$(arch)/*.asm)# get all the files that match the regex
# take all the 3rd argument (in this case asm_src_files) and each one that matches the first argument gets replaced with the second argument
asm_obj_files := $(patsubst src/arch/$(arch)/%.asm, build/arch/$(arch)/%.o, $(asm_src_files))

.PHONY: all clean run iso# telling make that these aren't physical files and to always run them when told

all: $(kernel) 

clean:
	@rm -r build

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -d /usr/lib/grub/i386-pc -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): $(asm_obj_files) $(linker_script)
	@ld -n -T $(linker_script) -o $(kernel) $(asm_obj_files)

# $@ and $< are automatic variables
# $@ is the target, in this case the .o file
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
