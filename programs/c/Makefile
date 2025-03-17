CC=clang
OPT=opt
LLC=llc
CFLAGS=--target=riscv32-unknown-none -march=rv32im -mabi=ilp32 -O0 -Xclang -disable-O0-optnone
LLVMFLAGS=-disable-verify -passes='$(PASSES)'

LIBNAME=libzkvmc.a
OUTDIR=ctarget
SRC=guest/main.c

.PHONY: all
all: $(OUTDIR)/$(LIBNAME)

.PHONY: bindgen
bindgen:
	cbindgen --config ./zkvm-host/cbindgen.toml --crate zkvm-host --output guest/bindings.h

.PHONY: platform
platform:
	RUSTFLAGS="-C passes=loweratomic" cargo +risc0 rustc -p zkvm-platform --target riscv32im-risc0-zkvm-elf --lib --crate-type staticlib --release --target-dir ./guest/out/platform --features risc0
	
$(OUTDIR)/$(LIBNAME): $(SRC) bindgen
	@mkdir -p $(OUTDIR)
	$(CC) $(CFLAGS) -S -emit-llvm $(SRC) -o $(OUTDIR)/zkvmc.ll
	$(OPT) $(LLVMFLAGS) -S $(OUTDIR)/zkvmc.ll -o $(OUTDIR)/zkvmcopt.ll
	$(LLC) -filetype=obj $(OUTDIR)/zkvmcopt.ll -o $(OUTDIR)/zkvmc.o
	ar rcs $(OUTDIR)/$(LIBNAME) $(OUTDIR)/zkvmc.o

.PHONY: clean
clean:
	rm -rf $(OUTDIR)
