include ../c-build/Makefile

CFLAGS+=-DSPEC -DSPEC_MANUAL_CONFIG

$(OUTDIR)/$(LIBNAME): $(SRC)
	$(call add,$(PROGRAM),guest/perlmain.c)
	$(call archive,$(OUTDIR)/$(PROGRAM).o)
