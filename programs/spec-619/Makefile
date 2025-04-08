include ../c-build/Makefile

CFLAGS+=-DSPEC

$(OUTDIR)/$(LIBNAME): $(SRC)
	$(call add,$(PROGRAM),guest/main.c)
	$(call archive,$(OUTDIR)/$(PROGRAM).o)
