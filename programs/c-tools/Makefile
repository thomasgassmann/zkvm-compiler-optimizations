CC=clang
OPT=opt
LLC=llc
CFLAGS=--target=riscv32-unknown-none -march=rv32im -mabi=ilp32 -O0 -Xclang -disable-O0-optnone
LLVMFLAGS=-disable-verify -passes='$(PASSES)'

LIBNAME=lib$(PROGRAM).a
OUTDIR=ctarget
SRC=guest/main.c

.PHONY: all
all: $(OUTDIR)/$(LIBNAME)

$(OUTDIR)/$(LIBNAME): $(SRC)
	@mkdir -p $(OUTDIR)
	$(CC) $(CFLAGS) -S -emit-llvm $(SRC) -o $(OUTDIR)/$(PROGRAM).ll
	$(OPT) $(LLVMFLAGS) -S $(OUTDIR)/$(PROGRAM).ll -o $(OUTDIR)/$(PROGRAM)opt.ll
	$(LLC) -filetype=obj $(OUTDIR)/$(PROGRAM)opt.ll -o $(OUTDIR)/$(PROGRAM).o
	ar rcs $(OUTDIR)/$(LIBNAME) $(OUTDIR)/$(PROGRAM).o

.PHONY: clean
clean:
	rm -rf $(OUTDIR)
