CC=clang
OPT=opt
LLC=llc
CFLAGS=--target=riscv32-unknown-none -march=rv32im -mabi=ilp32 -O0 -Xclang -disable-O0-optnone -I../c-platform/include -Wno-incompatible-library-redeclaration
LLVMFLAGS=-disable-verify -passes='$(PASSES)'

LIBNAME=lib$(PROGRAM).a
OUTDIR=ctarget

define add 
	$(CC) $(CFLAGS) -S -emit-llvm ${2} -o $(OUTDIR)/${1}.ll
	$(OPT) $(LLVMFLAGS) -S $(OUTDIR)/${1}.ll -o $(OUTDIR)/${1}opt.ll
	$(LLC) -filetype=obj $(OUTDIR)/${1}opt.ll -o $(OUTDIR)/${1}.o
endef

define archive
	ar rcs $(OUTDIR)/$(LIBNAME) ${1}
endef

.PHONY: setup
setup:
	@mkdir -p $(OUTDIR)

.PHONY: all
all: setup $(OUTDIR)/$(LIBNAME)

.PHONY: clean
clean:
	rm -rf $(OUTDIR)
