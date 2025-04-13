include ../c-build/Makefile

CFLAGS+=-DSPEC

$(OUTDIR)/$(LIBNAME): $(SRC)
	$(call add,$(PROGRAM),guest/mcf.c)
	$(call add,mcfutil,guest/mcfutil.c)
	$(call add,psimplex,guest/psimplex.c)
	$(call add,pbeampp,guest/pbeampp.c)
	$(call add,implicit,guest/implicit.c)
	$(call add,pbla,guest/pbla.c)
	$(call add,treeup,guest/treeup.c)
	$(call add,readmin,guest/readmin.c)
	$(call add,pflowup,guest/pflowup.c)
	$(call add,pstart,guest/pstart.c)
	$(call add,spec_qsort,guest/spec_qsort.c)
	$(call add,output,guest/output.c)
	$(call archive,$(OUTDIR)/output.o,$(OUTDIR)/spec_qsort.o,$(OUTDIR)/pstart.o,$(OUTDIR)/pflowup.o,$(OUTDIR)/readmin.o,$(OUTDIR)/treeup.o,$(OUTDIR)/pbla.o,$(OUTDIR)/implicit.o,$(OUTDIR)/pbeampp.o,$(OUTDIR)/psimplex.o,$(OUTDIR)/mcfutil.o,$(OUTDIR)/$(PROGRAM).o)
