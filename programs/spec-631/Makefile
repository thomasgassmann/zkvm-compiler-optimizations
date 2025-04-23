include ../c-build/Makefile

CC=clang++
CFLAGS+=-DSPEC -DSMALL_MEMORY

$(OUTDIR)/$(LIBNAME): $(SRC)
	$(call add,$(PROGRAM),guest/sjeng.cpp)
	$(call add,epd,guest/epd.cpp)
	$(call add,utils,guest/utils.cpp)
	$(call add,initp,guest/initp.cpp)
	$(call add,ttable,guest/ttable.cpp)
	$(call add,preproc,guest/preproc.cpp)
	$(call add,bitboard,guest/bitboard.cpp)
	$(call add,state,guest/state.cpp)
	$(call add,attacks,guest/attacks.cpp)
	$(call add,pawn,guest/pawn.cpp)
	$(call add,bits,guest/bits.cpp)
	$(call add,board,guest/board.cpp)
	$(call add,make,guest/make.cpp)
	$(call add,neval,guest/neval.cpp)
	$(call add,moves,guest/moves.cpp)
	$(call add,generate,guest/generate.cpp)
	$(call add,search,guest/search.cpp)
	$(call add,draw,guest/draw.cpp)
	$(call add,endgame,guest/endgame.cpp)
	$(call add,see,guest/see.cpp)
	$(call archive,$(OUTDIR)/see.o,$(OUTDIR)/endgame.o,$(OUTDIR)/draw.o,$(OUTDIR)/search.o,$(OUTDIR)/generate.o,$(OUTDIR)/moves.o,$(OUTDIR)/neval.o,$(OUTDIR)/make.o,$(OUTDIR)/board.o,$(OUTDIR)/bits.o,$(OUTDIR)/pawn.o,$(OUTDIR)/attacks.o,$(OUTDIR)/state.o,$(OUTDIR)/bitboard.o,$(OUTDIR)/preproc.o,$(OUTDIR)/ttable.o,$(OUTDIR)/initp.o,$(OUTDIR)/utils.o,$(OUTDIR)/epd.o,$(OUTDIR)/$(PROGRAM).o)
