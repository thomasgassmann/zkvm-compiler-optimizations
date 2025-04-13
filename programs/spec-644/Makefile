include ../c-build/Makefile

CFLAGS+=-DSPEC

$(OUTDIR)/$(LIBNAME): $(SRC)
	$(call add,$(PROGRAM),guest/main.c)
	$(call add,lbm,guest/lbm.c)
	$(call archive,$(OUTDIR)/lbm.o,$(OUTDIR)/$(PROGRAM).o)
