APP_NAME = app_cortex_a9
TARGET = armv7-unknown-linux-gnueabihf
CPU= cortex-a9
MACHINE= xilinx-zynq-a9
OBJCOPY = cargo objcopy --
OBJCOPY_PARAMS = --strip-all -O binary
SOURCES = $(wildcard **/*.rs) $(wildcard **/*.S) link.ld
OUT_DIR = target/$(TARGET)/debug/

all: $(APP_NAME).bin

$(OUT_DIR)$(APP_NAME): $(SOURCES)
	xargo build --target=$(TARGET) 

$(APP_NAME).bin: $(OUT_DIR)$(APP_NAME)
	cp $< $(OUT_DIR)$(APP_NAME).elf
	$(OBJCOPY) $(OBJCOPY_PARAMS) $< $(APP_NAME).bin

disassemble:
	cargo objdump --target $(TARGET) -- -disassemble -print-imm-hex $(APP_NAME)

nm:
	cargo nm --target $(TARGET) -- $(OUT_DIR)$(APP_NAME).elf | sort

qemu:
	qemu-system-arm -M $(MACHINE) -cpu $(CPU) -kernel $(OUT_DIR)$(APP_NAME).elf -nographic -S -s

debug:
	gdb-multiarch -q $(OUT_DIR)$(APP_NAME).elf

size:
	cargo size --bin $(APP_NAME) --target=$(TARGET) -- -A

clean:
	@cargo clean
	@rm -f app_cortex_a9.bin

.PHONY: all disassemble nm qemu debug size clean
