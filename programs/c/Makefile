CC=clang
OPT=opt
LLC=llc
CFLAGS=--target=riscv32-unknown-none -march=rv32im -mabi=ilp32 -O0 -Xclang -disable-O0-optnone
LLVMFLAGS=-disable-verify -passes=$(PASSES)

# Target library (static object file)
LIBNAME=libhello.a

# Output directory for the library
OUTDIR=ctarget

# Path to the source file
SRC=src/hello.c

all: $(OUTDIR)/$(LIBNAME)

# Build the static library
$(OUTDIR)/$(LIBNAME): $(SRC)
	@mkdir -p $(OUTDIR)
	$(CC) $(CFLAGS) -S -emit-llvm $(SRC) -o $(OUTDIR)/hello.ll
	$(OPT) $(LLVMFLAGS) -S $(OUTDIR)/hello.ll -o $(OUTDIR)/helloopt.ll
	$(LLC) -filetype=obj $(OUTDIR)/helloopt.ll -o $(OUTDIR)/hello.o
	ar rcs $(OUTDIR)/$(LIBNAME) $(OUTDIR)/hello.o

# Clean target
clean:
	rm -rf $(OUTDIR)
