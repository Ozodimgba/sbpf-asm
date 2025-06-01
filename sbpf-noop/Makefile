OUT_DIR ?= ./out
SRC_DIR ?= ./asm
DEPLOY_DIR ?= ./deploy
LLVM_DIR := ./solana-llvm/llvm
LLVM_CLANG := $(LLVM_DIR)/bin/clang
LLD := $(LLVM_DIR)/bin/ld.lld

.PHONY: all clean

all: $(DEPLOY_DIR)/log.so

clean:
	rm -rf $(OUT_DIR)
	rm -rf $(DEPLOY_DIR)

# Generate linker script dynamically
$(OUT_DIR)/sbf.ld:
	mkdir -p $(OUT_DIR)
	if [ ! -f $@ ]; then \
		echo "PHDRS { \
			text PT_LOAD ; \
			rodata PT_LOAD ; \
			data PT_LOAD ; \
			dynamic PT_DYNAMIC ; \
		} \
		SECTIONS { \
			. = SIZEOF_HEADERS; \
			.text : { *(.text*) } :text \
			.rodata : { *(.rodata*) } :rodata \
			.data.rel.ro : { *(.data.rel.ro*) } :rodata \
			.dynamic : { *(.dynamic) } :dynamic \
			.dynsym : { *(.dynsym) } :data \
			.dynstr : { *(.dynstr) } :data \
			.rel.dyn : { *(.rel.dyn) } :data \
			/DISCARD/ : { *(.eh_frame*) *(.gnu.hash*) *(.hash*) } \
		}" > $@; \
	fi

# Compile assembly to object file
$(OUT_DIR)/log.o: $(SRC_DIR)/log.s
	mkdir -p $(OUT_DIR)
	$(LLVM_CLANG) -target sbf -c -o $(OUT_DIR)/log.o $(SRC_DIR)/log.s

# Linker flags
SBF_LLD_FLAGS := \
  -z notext \
  -shared \
  --image-base 0x100000000 \
  -T $(OUT_DIR)/sbf.ld \
  --entry e

# Link object file to final shared object
$(DEPLOY_DIR)/log.so: $(OUT_DIR)/log.o $(OUT_DIR)/sbf.ld
	mkdir -p $(DEPLOY_DIR)
	$(LLD) $(SBF_LLD_FLAGS) -o $(DEPLOY_DIR)/log.so $(OUT_DIR)/log.o