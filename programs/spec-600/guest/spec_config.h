#ifndef _specconfig_h_
#define _specconfig_h_

/****************************************************************************
 * Architecture and OS flags
 *
 * Given a SPEC_OS_ARCH flag (specified by the user), provide
 * a SPEC_OS flag and a SPEC_ARCH flag.  Also define
 * SPEC_LITTLEENDIAN or SPEC_BIGENDIAN as appropriate.
 * For systems with only one data model (e.g. 64-bit Windows), also set
 * the data model flag, which is one of
 * SPEC_ILP32
 * SPEC_ILP64
 * SPEC_LP64
 * SPEC_P64
 ****************************************************************************/

/* For reference, here are the list of pre-defined arch/os flags that
 * 500.perlbench expects to see.  Please define only one.  Some (*BSD)
 * have not been tested/ported at all.
 *
 * SPEC_AIX		IBM AIX on PowerPC hardware
 * SPEC_BSD_X64		*BSD on AMD64-compatible hardware in 64-bit mode
 * SPEC_BSD_IA32	*BSD on Intel x86 hardware
 * SPEC_BSD_IA64	*BSD on Intel IA-64 hardware
 * SPEC_BSD_SPARC	*BSD on Sun SPARC hardware
 * SPEC_BSD_PPC		*BSD on PowerPC hardware
 * SPEC_BSD_ALPHA	*BSD on Compaq Alpha hardware
 * SPEC_BSD_MIPS 	*BSD on MIPS hardware
 * SPEC_HPUX_IA64	HP-UX on Intel IA-64 hardware
 * SPEC_HPUX_HPPA	HP-UX on HPPA hardware
 * SPEC_IRIX		SGI IRIX on MIPS hardware
 * SPEC_LINUX_X64	Linux (glibc 2.1+) on AMD64-compatible hardware in 64-bit mode
 * SPEC_LINUX_AARCH32	Linux (glibc 2.1+) on ARM hardware in 32-bit mode
 * SPEC_LINUX_AARCH64	Linux (glibc 2.1+) on 64-bit ARM hardware in 64-bit mode
 * SPEC_LINUX_IA32	Linux (glibc 2.1+) on Intel x86 hardware
 * SPEC_LINUX_IA64	Linux (glibc 2.1+) on Intel IA-64 hardware
 * SPEC_LINUX_SPARC	Linux (glibc 2.1+) on Sun SPARC hardware
 * SPEC_LINUX_PPC	Linux (glibc 2.1+) on big-endian PowerPC hardware
 * SPEC_LINUX_PPC_LE	Linux (glibc 2.1+) on little-endian PowerPC hardware
 * SPEC_LINUX_ALPHA	Linux (glibc 2.1+) on Compaq Alpha hardware
 * SPEC_LINUX_MIPS	Linux (glibc 2.1+) on MIPS hardware
 * SPEC_MACOSX_IA32	Apple Mac OS X on Intel x86 hardware
 * SPEC_MACOSX_X64	Apple Mac OS X on Intel x86 hardware (64-bit)
 * SPEC_RELIANT_MIPS	Reliant UNIX on MIPS hardware
 * SPEC_SOLARIS_SPARC	Sun Solaris running on SPARC hardware
 * SPEC_SOLARIS_IA32	Sun Solaris running on Intel x86 hardware
 * SPEC_SOLARIS_X64	Sun Solaris running on AMD64-compatible hardware
 * SPEC_TRU64		Compaq Tru64 Unix on Alpha hardware
 * SPEC_WIN32		Microsoft Windows 7+ on x86 hardware
 * SPEC_WIN64		Microsoft Windows 7+ on AMD64-compatible hardware
 * SPEC_WIN64_IA64	Microsoft Windows 7+ on IA-64 hardware
 * SPEC_MANUAL_CONFIG	Configure all required settings manually
 *
 * Almost every word in the above section is someone's trademark
 */

#if !defined(SPEC_AIX) && !defined(SPEC_BSD_X64) && !defined(SPEC_BSD_IA32) \
    && !defined(SPEC_BSD_IA64) && !defined(SPEC_BSD_SPARC)                  \
    && !defined(SPEC_BSD_PPC) && !defined(SPEC_BSD_ALPHA)                   \
    && !defined(SPEC_BSD_MIPS) && !defined(SPEC_HPUX_IA64)                  \
    && !defined(SPEC_HPUX_HPPA) && !defined(SPEC_IRIX)                      \
    && !defined(SPEC_LINUX_X64) && !defined(SPEC_LINUX_AARCH32)             \
    && !defined(SPEC_LINUX_AARCH64) && !defined(SPEC_LINUX_IA32)            \
    && !defined(SPEC_LINUX_IA64) && !defined(SPEC_LINUX_SPARC)              \
    && !defined(SPEC_LINUX_PPC) && !defined(SPEC_LINUX_PPC_LE)              \
    && !defined(SPEC_LINUX_ALPHA) && !defined(SPEC_LINUX_MIPS)              \
    && !defined(SPEC_MACOSX_IA32) && !defined(SPEC_MACOSX_X64)              \
    && !defined(SPEC_RELIANT_MIPS) && !defined(SPEC_SOLARIS_SPARC)          \
    && !defined(SPEC_SOLARIS_IA32) && !defined(SPEC_SOLARIS_X64)            \
    && !defined(SPEC_TRU64) && !defined(SPEC_WIN32) && !defined(SPEC_WIN64) \
    && !defined(SPEC_WIN64_IA64)                                            \
    && !defined(SPEC_MANUAL_CONFIG)
# error Please use one of the platform selection defines in spec_config.h
#endif


/* per-OS flags, for OSes that run on multiple architectures */

#if defined(SPEC_LINUX_IA32) || defined(SPEC_LINUX_IA64) || \
    defined(SPEC_LINUX_SPARC) || defined(SPEC_LINUX_PPC) || \
    defined(SPEC_LINUX_ALPHA) || defined(SPEC_LINUX_MIPS) || \
    defined(SPEC_LINUX_X64) || defined(SPEC_LINUX_AARCH64) || \
    defined(SPEC_LINUX_AARCH32) || defined(SPEC_LINUX_PPC_LE)
# define SPEC_LINUX
#endif

#if defined(SPEC_BSD_IA32) || defined(SPEC_BSD_IA64) || \
    defined(SPEC_BSD_SPARC) || defined(SPEC_BSD_PPC) || \
    defined(SPEC_BSD_ALPHA) || defined(SPEC_BSD_MIPS) || \
    defined(SPEC_BSD_X64)
# define SPEC_BSD
#endif

#if defined(SPEC_SOLARIS_SPARC) || \
    defined(SPEC_SOLARIS_IA32) || \
    defined(SPEC_SOLARIS_X64)
# define SPEC_SOLARIS
#endif

#if defined(SPEC_HPUX_IA64) || defined(SPEC_HPUX_HPPA)
# define SPEC_HPUX
#endif

#if defined(SPEC_LINUX_X64) || defined (SPEC_BSD_X64) || \
    defined(SPEC_WIN64) || defined (SPEC_SOLARIS_X64) ||\
    defined(SPEC_MACOSX_X64)
# define SPEC_X64
#endif

#if defined(SPEC_LINUX_AARCH64)
# define SPEC_AARCH64
#endif

#if defined(SPEC_LINUX_AARCH32)
# define SPEC_AARCH32
#endif

#if defined(SPEC_RELIANT_MIPS)
# define SPEC_RELIANT
#endif

#if defined(SPEC_MACOSX_IA32) || defined(SPEC_MACOSX_X64)
# define SPEC_MACOSX
#endif

#if defined(SPEC_WIN32) || defined(SPEC_WIN64_IA64) || defined(SPEC_WIN64)
  /* This should be defined in Makefile.defaults.
   * But just in case it isn't...
   */
# if !defined(SPEC_WINDOWS)
#  define SPEC_WINDOWS
# endif
#endif

#if defined(SPEC_WINDOWS) && !defined(WIN32) && !defined(SPEC_NO_WIN32)
  /* Refugee from src/Makefile */
# define WIN32
#endif

/* per-architecture flags, for architectures that run multiple OSes */

#if defined(SPEC_LINUX_MIPS) || defined(SPEC_IRIX) || \
    defined(SPEC_BSD_MIPS) || defined(SPEC_RELIANT_MIPS)
# define SPEC_MIPS
#endif

#if defined(SPEC_LINUX_PPC) || defined(SPEC_AIX) || \
    defined(SPEC_BSD_PPC)
# define SPEC_PPC
#endif

#if defined(SPEC_LINUX_PPC_LE)
# define SPEC_PPC_LE
#endif

#if defined(SPEC_LINUX_IA32) || defined(SPEC_WIN32) || \
    defined(SPEC_BSD_IA32) || defined(SPEC_SOLARIS_IA32) || \
    defined(SPEC_MACOSX_IA32)
# define SPEC_IA32
#endif

#if defined(SPEC_LINUX_ALPHA) || defined(SPEC_TRU64) || \
    defined(SPEC_BSD_ALPHA)
# define SPEC_ALPHA
#endif

#if defined(SPEC_LINUX_SPARC) || defined(SPEC_SOLARIS_SPARC) ||\
    defined(SPEC_BSD_SPARC)
# define SPEC_SPARC
#endif

#if defined(SPEC_LINUX_IA64) || \
    defined(SPEC_BSD_IA64) || \
    (defined(SPEC_WIN64_IA64) && !defined(SPEC_WIN64))
# define SPEC_IA64LE
# define SPEC_IA64
#endif

#if defined(SPEC_HPUX_IA64)
# define SPEC_IA64BE
# define SPEC_IA64
#endif

#if defined(SPEC_HPUX_HPPA)
# define SPEC_HPPA
#endif

#if defined(SPEC_PPC) || defined(SPEC_SPARC) || \
    defined(SPEC_HPPA) || defined(SPEC_IA64BE) || \
    defined(SPEC_MIPS)
# define SPEC_BIGENDIAN
#else
# define SPEC_LITTLEENDIAN
#endif

/* For clarity, each compilation environment data model will have its own
 * section.
 */

#if defined(SPEC_LP64)
/****************************************************************************
 * LP64 defines
 ****************************************************************************/
# if defined(SPEC_BIGENDIAN)
#   define BYTEORDER 0x87654321
# else
#   define BYTEORDER 0x12345678
# endif
# define INTSIZE 4
# define LONGSIZE 8
# define SHORTSIZE 2
# define DOUBLESIZE 8
# define PTRSIZE 8
# define Size_t_size 8
# define QUADKIND QUAD_IS_LONG
# define Quad_t long
# define Uquad_t unsigned long
# if !defined(SPEC_INT32_T)
#  define SPEC_INT32_T   unsigned int
# endif
# define	IVTYPE		long
# define	IVSIZE		8
# define	UVTYPE		unsigned long
# define	UVSIZE		8
# define	I8TYPE		signed char
# define	I8SIZE		1
# define	U8TYPE		unsigned char
# define	U8SIZE		1
# define	I16TYPE		short
# define	I16SIZE		2
# define	U16TYPE		unsigned short
# define	U16SIZE		2
# define	I32TYPE		int
# define	I32SIZE		4
# define	U32TYPE		unsigned int
# define	U32SIZE		4
# define	I64TYPE		long
# define	I64SIZE		8
# define	U64TYPE		unsigned long
# define	U64SIZE		8
# define	NVTYPE		double
# define	NVSIZE		8
# if defined(SPEC_RELIANT_MIPS) || defined(SPEC_TRU64) || \
     defined(SPEC_MACOSX) || defined(SPEC_LINUX) || \
     defined(SPEC_SOLARIS)
#  if !defined(SPEC_NO_USE_64_BIT_INT)
#   define USE_64_BIT_INT
#  endif
# endif
# if defined(SPEC_MACOSX) || defined(SPEC_LINUX) || \
     defined(SPEC_SOLARIS)
#  if !defined(SPEC_NO_USE_64_BIT_ALL)
#   define USE_64_BIT_ALL
#  endif
# endif

#elif defined(SPEC_ILP64)
/****************************************************************************
 * ILP64 defines
 ****************************************************************************/
# if defined(SPEC_BIGENDIAN)
#   define BYTEORDER 0x87654321
# else
#   define BYTEORDER 0x12345678
# endif
# define INTSIZE 8
# define LONGSIZE 8
# define SHORTSIZE 2
# define DOUBLESIZE 8
# define PTRSIZE 8
# define Size_t_size 8
# define QUADKIND QUAD_IS_INT
# define Quad_t int
# define Uquad_t unsigned int
# if !defined(SPEC_INT32_T)
#  error You must have a type that is exactly 32 bits wide!
# endif
# define	IVTYPE		int
# define	IVSIZE		8
# define	UVTYPE		unsigned int
# define	UVSIZE		8
# define	I8TYPE		signed char
# define	I8SIZE		1
# define	U8TYPE		unsigned char
# define	U8SIZE		1
# define	I16TYPE		short
# define	I16SIZE		2
# define	U16TYPE		unsigned short
# define	U16SIZE		2
# define	I32TYPE		int
# define	I32SIZE		8
# define	U32TYPE		unsigned int
# define	U32SIZE		8
# define	I64TYPE		long
# define	I64SIZE		8
# define	U64TYPE		unsigned long
# define	U64SIZE		8
# define	NVTYPE		double
# define	NVSIZE		8

#elif defined(SPEC_WIN64_IA64) || defined(SPEC_WIN64)
/****************************************************************************
 * 64-bit Windows defines.  Technically, Win64 is a P64 model, but the Perl
 * port to this platform is more IP64.
 ****************************************************************************/
# define WIN64
# define CONSERVATIVE
# if defined(SPEC_BIGENDIAN)
#   define BYTEORDER 0x87654321
# else
#   define BYTEORDER 0x12345678
# endif
# define INTSIZE 4
# define LONGSIZE 4
# define SHORTSIZE 2
# define DOUBLESIZE 8
# define PTRSIZE 8
# define SSize_t __int64
# define Size_t_size 8
# define HAS_QUAD
# define USE_64_BIT_INT
# define QUADKIND QUAD_IS_INT64_T
# define Quad_t __int64
# define Uquad_t unsigned __int64
# define Off_t __int64
# define LSEEKSIZE 8
# define Off_t_size 8
# if !defined(SPEC_INT32_T)
#  define SPEC_INT32_T   unsigned long
# endif
# define	IVTYPE		__int64
# define	IVSIZE		8
# define	UVTYPE		unsigned __int64
# define	UVSIZE		8
# define	I8TYPE		signed char
# define	I8SIZE		1
# define	U8TYPE		unsigned char
# define	U8SIZE		1
# define	I16TYPE		short
# define	I16SIZE		2
# define	U16TYPE		unsigned short
# define	U16SIZE		2
# define	I32TYPE		long
# define	I32SIZE		4
# define	U32TYPE		unsigned long
# define	U32SIZE		4
# define	I64TYPE		__int64
# define	U64TYPE		unsigned __int64
# define	I64SIZE		8
# define	U64SIZE		8
# define	NVTYPE		double
# define	NVSIZE		8
# define	IVdf		"I64d"
# define	UVuf		"I64u"
# define	UVof		"I64o"
# define	UVxf		"I64x"
# define	UVXf		"I64X"
# if defined(SPEC_WINDOWS) && !defined(SPEC_NO_STAT64)
#  define Stat_t struct _stat64
# endif /* SPEC_WINDOWS && !SPEC_NO_STAT64 */

#elif defined(SPEC_P64)
/****************************************************************************
 * P64 defines
 ****************************************************************************/
# if defined(SPEC_BIGENDIAN)
#   define BYTEORDER 0x4321
# else
#   define BYTEORDER 0x1234
# endif
# define INTSIZE 4
# define LONGSIZE 4
# define SHORTSIZE 2
# define DOUBLESIZE 8
# define PTRSIZE 8
# define Size_t_size 8
# define QUADKIND QUAD_IS_INT64_T
# define Quad_t int64_t
# define Uquad_t uint64_t
# if !defined(SPEC_INT32_T)
#  define SPEC_INT32_T   unsigned long
# endif
# define	IVTYPE		long
# define	IVSIZE		4
# define	UVTYPE		unsigned long
# define	UVSIZE		4
# define	I8TYPE		signed char
# define	I8SIZE		1
# define	U8TYPE		unsigned char
# define	U8SIZE		1
# define	I16TYPE		short
# define	I16SIZE		2
# define	U16TYPE		unsigned short
# define	U16SIZE		2
# define	I32TYPE		long
# define	I32SIZE		4
# define	U32TYPE		unsigned long
# define	U32SIZE		4
# if defined(SPEC_WINDOWS)
#  define	I64TYPE		__int64
#  define	U64TYPE		unsigned __int64
#  if !defined(SPEC_NO_STAT64)
#   define Stat_t struct _stat64
#  endif /* !SPEC_NO_STAT64 */
# else
#  define	I64TYPE		long long
#  define	U64TYPE		unsigned long long
# endif
# define	I64SIZE		8
# define	U64SIZE		8
# define	NVTYPE		double
# define	NVSIZE		8

#elif !defined(SPEC_GOOFY_DATAMODEL)
/****************************************************************************
 * ILP32 defines
 ****************************************************************************/
# if defined(SPEC_BIGENDIAN)
#   define BYTEORDER 0x4321
# else
#   define BYTEORDER 0x1234
# endif
# define INTSIZE 4
# define LONGSIZE 4
# define SHORTSIZE 2
# define DOUBLESIZE 8
# define PTRSIZE 4
# define Size_t_size 4
# define QUADKIND QUAD_IS_LONG_LONG
# define Quad_t long long
# define Uquad_t unsigned long long
# if !defined(SPEC_INT32_T)
#  define SPEC_INT32_T   unsigned long
# endif
# define	IVTYPE		long
# define	IVSIZE		4
# define	UVTYPE		unsigned long
# define	UVSIZE		4
# define	I8TYPE		signed char
# define	I8SIZE		1
# define	U8TYPE		unsigned char
# define	U8SIZE		1
# define	I16TYPE		short
# define	I16SIZE		2
# define	U16TYPE		unsigned short
# define	U16SIZE		2
# define	I32TYPE		long
# define	I32SIZE		4
# define	U32TYPE		unsigned long
# define	U32SIZE		4
# define	I64TYPE		long long
# define	I64SIZE		8
# define	U64TYPE		unsigned long long
# define	U64SIZE		8
# define	NVTYPE		double
# define	NVSIZE		8
#endif /* ILP32 */

/* Stuff everyone needs */

/****************************************************************************
 * Various system tweaks
 ****************************************************************************/
#define ST_INO_SIZE     IVSIZE
#define ST_INO_SIGN     0

#if defined(SPEC_CHARBITS)
# define CHARBITS SPEC_CHARBITS
#else
# define CHARBITS 8
#endif

#if defined(SPEC_LINUX_IA32)
# define MEM_ALIGNBYTES 4
#else
# define MEM_ALIGNBYTES 8
#endif

#if !defined(SPEC_NO_I_STDBOOL)
# define I_STDBOOL
#endif

#if defined(SPEC_LINUX) && !defined(SPEC_NO__XOPEN_SOURCE) && !defined(_XOPEN_SOURCE)
/* Get prototypes for ftruncate(), truncate(), and putenv() */
# define _XOPEN_SOURCE 700
#endif

#if defined(SPEC_AIX)
# define I_SYS_PARAM
#endif

#if !defined(SPEC_NO_I_STDDEF)
# define I_STDDEF
#endif

#if defined(SPEC_SOLARIS) || defined(SPEC_WINDOWS) || \
    defined(SPEC_IRIX)
# if !defined(SPEC_NO_I_FCNTL)
#  define I_FCNTL
# endif
#elif !defined(SPEC_NO_I_SYS_FILE)
# define I_SYS_FILE
#endif

#if !defined(SPEC_NO_DIRNAMLEN)
# if defined(SPEC_MACOSX) || defined(SPEC_TRU64) || \
     defined(SPEC_HPUX) || defined(SPEC_AIX) || \
     defined(SPEC_WINDOWS)
#  define DIRNAMLEN
# endif
#endif
#if !defined(Direntry_t)
# if !defined(SPEC_WINDOWS)
#  define Direntry_t struct dirent
# else
#  define Direntry_t struct direct
# endif
#endif

#if !defined(SPEC_NO_I_TIME)
# if defined(SPEC_AIX) || defined(SPEC_LINUX) || \
     defined(SPEC_WINDOWS)
#  define I_TIME
# endif
#endif

#if !defined(SPEC_NO_CASTNEGFLOAT)
# if defined(SPEC_MACOSX) || defined(SPEC_SOLARIS) || \
  (defined(SPEC_LINUX) && \
    !(defined(SPEC_LINUX_PPC_LE) || \
      defined(SPEC_LINUX_AARCH32) || defined(SPEC_LINUX_AARCH64)) \
  )
#  define CASTNEGFLOAT
# endif
#endif

#if !defined(CASTFLAGS)
# if defined(SPEC_AIX) || defined(SPEC_LINUX_IA64) || \
     defined(SPEC_HPUX_IA64) || defined(SPEC_LINUX_PPC_LE) || \
     defined(SPEC_LINUX_AARCH32) || defined(SPEC_LINUX_AARCH64)
#  define CASTFLAGS 1
# else
#  define CASTFLAGS 0
# endif
#endif

#if !defined(LONG_DOUBLESIZE)
# if defined(SPEC_LINUX_ALPHA) || defined(SPEC_PPC) || \
     defined(SPEC_LINUX_SPARC)
#  define LONG_DOUBLESIZE 8
# elif defined(SPEC_WINDOWS)
#  define LONG_DOUBLESIZE 10
# elif defined(SPEC_LINUX_IA32) || defined(SPEC_SOLARIS_IA32)
#  define LONG_DOUBLESIZE 12
# else
#  define LONG_DOUBLESIZE 16
# endif
#endif

#if !defined(LONG_DOUBLEKIND)
/* XXX TODO: This is sort of a cop-out; it'd be best to figure out what the
 * XXX TODO: correct setting for each system is.
 */
# define LONG_DOUBLEKIND LONG_DOUBLE_IS_UNKNOWN_FORMAT
# if 0
# if LONG_DOUBLESIZE == DOUBLESIZE
#  define LONG_DOUBLEKIND LONG_DOUBLE_IS_DOUBLE
# elif LONG_DOUBLESIZE == 10 || LONG_DOUBLESIZE == 12
#  if defined(SPEC_LITTLENDIAN)
#   define LONG_DOUBLEKIND LONG_DOUBLE_IS_X86_80_BIT_LITTLE_ENDIAN
#  else
   /* Is this even possible? */
#   define LONG_DOUBLEKIND LONG_DOUBLE_IS_X86_80_BIT_BIG_ENDIAN
#  endif
# elif LONG_DOUBLESIZE == 16
   /* These assumptions are probably wrong */
#  if defined(SPEC_LITTLENDIAN)
#   define LONG_DOUBLEKIND LONG_DOUBLE_IS_IEEE_754_128_BIT_LITTLE_ENDIAN
#  else
   /* Is this even possible? */
#   define LONG_DOUBLEKIND LONG_DOUBLE_IS_IEEE_754_128_BIT_BIG_ENDIAN
#  endif
# else
#  define LONG_DOUBLEKIND LONG_DOUBLE_IS_UNKNOWN_FORMAT
# endif
#endif
#endif

#if !defined(DOUBLEKIND)
/* XXX TODO: This is sort of a cop-out; it'd be best to figure out what the
 * XXX TODO: correct setting for each system is.
 */
# define DOUBLEKIND DOUBLE_IS_UNKNOWN_FORMAT
# if 0
# if DOUBLESIZE == 4
#  if defined(SPEC_LITTLENDIAN)
#   define DOUBLEKIND DOUBLE_IS_IEEE_754_32_BIT_LITTLE_ENDIAN
#  else
#   define DOUBLEKIND DOUBLE_IS_IEEE_754_32_BIT_BIG_ENDIAN
#  endif
# elif DOUBLESIZE == 8
#  if defined(SPEC_LITTLENDIAN)
#   define DOUBLEKIND DOUBLE_IS_IEEE_754_64_BIT_LITTLE_ENDIAN
#  else
#   define DOUBLEKIND DOUBLE_IS_IEEE_754_64_BIT_BIG_ENDIAN
#  endif
# elif DOUBLESIZE == 16
#  if defined(SPEC_LITTLENDIAN)
#   define DOUBLEKIND DOUBLE_IS_IEEE_754_128_BIT_LITTLE_ENDIAN
#  else
#   define DOUBLEKIND DOUBLE_IS_IEEE_754_128_BIT_BIG_ENDIAN
#  endif
# else
#  define DOUBLEKIND DOUBLE_IS_UNKNOWN_FORMAT
# endif
#endif
#endif

#if !defined(SPEC_HPUX) && !defined(SPEC_SOLARIS) && \
    !defined(SPEC_NO_FFLUSH_NULL)
# define FFLUSH_NULL
#endif

#if !defined(STDCHAR)
# if defined(SPEC_AIX) || defined(SPEC_TRU64) || \
     defined(SPEC_HPUX) || defined(SPEC_SOLARIS) || \
     defined(SPEC_IRIX) || defined(SPEC_AARCH32) || defined(SPEC_AARCH64)
#  define STDCHAR unsigned char
# else
#  define STDCHAR signed char
# endif
#endif

#if !defined(SPEC_WINDOWS) && !defined(SPEC_NO_HAS_QUAD)
# define HAS_QUAD
#endif

#if !defined(SPEC_WINDOWS) && !defined(SPEC_NO_HAS_FORK)
# define HAS_FORK
#endif

#if !defined(SPEC_NO_NEED_VA_COPY) && \
    defined(SPEC_X64)
# define NEED_VA_COPY
#endif

#if !defined(SPEC_WINDOWS)
# define SH_PATH "/bin/sh"
#else
# define SH_PATH "cmd /x /c"
#endif

#if !defined(SPEC_NO_HAS_DUP2)
# if defined(SPEC_MACOSX) || defined(SPEC_WINDOWS)
#  define HAS_DUP2
# endif
#endif

/* Handle the gconvert mess */
#if !defined(Gconvert)
# if defined(SPEC_TRU64) || defined(SPEC_MACOSX) || \
     defined(SPEC_WINDOWS) || defined(SPEC_LINUX)
#  define Gconvert(x,n,t,b) sprintf((b),"%.*g",(n),(x))
# elif defined(SPEC_SOLARIS)
#  define Gconvert(x,n,t,b) gconvert((x),(n),(t),(b))
# else
#  define Gconvert(x,n,t,b) gcvt((x),(n),(b))
# endif
#endif

/* Everyone's sprintf() should be working this way by now */
#if !defined(SPEC_NO_SPRINTF_RETURNS_STRLEN)
# define SPRINTF_RETURNS_STRLEN
#endif

/* This shouldn't matter, since we shouldn't be messing with UIDs and GIDs.
 * Still, better safe than sorry...  Reinhold's compiler might bite me again.
 */
#if !defined(Uid_t_sign)
# if defined(SPEC_HPUX) || defined(SPEC_IRIX) || \
     defined(SPEC_MIPS) || defined(SPEC_WINDOWS)
#  define Uid_t_sign -1
# else
#  define Uid_t_sign 1
# endif
#endif
#if !defined(Gid_t_sign)
# if defined(SPEC_HPUX) || defined(SPEC_IRIX) || \
     defined(SPEC_MIPS) || defined(SPEC_WINDOWS)
#  define Gid_t_sign -1
# else
#  define Gid_t_sign 1
# endif
#endif

#if !defined(SPEC_NO_NV_PRESERVES_UV) && \
     !(defined(SPEC_AIX) && defined(SPEC_LP64)) && \
     !defined(SPEC_ALPHA) && \
     !defined(SPEC_WIN64) && \
     !defined(SPEC_WIN64_IA64) && \
     !(defined(SPEC_IRIX) && defined(SPEC_LP64)) && \
     !defined(SPEC_X64) && \
     !defined(SPEC_AARCH64) && \
     !defined(SPEC_LINUX_IA64) && \
     !(defined(SPEC_SOLARIS) && defined(SPEC_LP64)) && \
     !(defined(SPEC_LINUX) && defined(SPEC_LP64))
# define NV_PRESERVES_UV
#endif

#if !defined(NV_PRESERVES_UV_BITS)
# if defined(SPEC_ALPHA) || \
     defined(SPEC_WIN64) || \
     defined(SPEC_WIN64_IA64) || \
     defined(SPEC_LINUX_IA64) || \
     (defined(SPEC_LINUX) && defined(SPEC_LP64)) || \
     (defined(SPEC_AIX) && defined(SPEC_LP64)) || \
     (defined(SPEC_WINDOWS) && defined(SPEC_P64)) || \
     (defined(SPEC_IRIX) && defined(SPEC_LP64)) || \
     (defined(SPEC_SOLARIS) && defined(SPEC_LP64)) || \
     (defined(SPEC_MACOSX) && defined(SPEC_LP64)) || \
     defined(SPEC_AARCH64)
#  define NV_PRESERVES_UV_BITS	53
#  define NV_OVERFLOWS_INTEGERS_AT	256.0*256.0*256.0*256.0*256.0*256.0*2.0*2.0*2.0*2.0*2.0
# elif (defined(SPEC_MACOSX) && !defined(SPEC_LP64)) || \
       (defined(SPEC_SOLARIS) && !defined(SPEC_LP64)) || \
       (defined(SPEC_LINUX) && !defined(SPEC_LP64)) || \
       (defined(SPEC_AIX) && !defined(SPEC_LP64)) || \
       (defined(SPEC_WINDOWS) && !defined(SPEC_LP64) && !defined(SPEC_P64)) || \
       defined(SPEC_AARCH32)
#  define NV_PRESERVES_UV_BITS	32
#  define NV_OVERFLOWS_INTEGERS_AT	256.0*256.0*256.0*256.0*256.0*256.0*2.0*2.0*2.0*2.0*2.0
# else
#  define NV_PRESERVES_UV_BITS	32
#  define NV_OVERFLOWS_INTEGERS_AT	256.0*256.0*256.0*256.0
# endif
#endif

#if !defined(SPEC_NO_NV_ZERO_IS_ALLBITS_ZERO)
# define NV_ZERO_IS_ALLBITS_ZERO
#endif

#if !defined(SELECT_MIN_BITS)
# if defined(SPEC_AIX)
#   define SELECT_MIN_BITS 1
# elif defined(SPEC_LINUX_ALPHA) || \
       defined(SPEC_LINUX_X64) ||\
       defined(SPEC_LINUX_AARCH64) ||\
       defined(SPEC_LINUX_IA64) || \
       (defined(SPEC_LINUX_PPC_LE) && defined(SPEC_LP64)) || \
       (defined(SPEC_SOLARIS_SPARC) && defined(SPEC_LP64)) || \
       defined(SPEC_SOLARIS_X64)
#   define SELECT_MIN_BITS 64
# else
#   define SELECT_MIN_BITS 32
# endif
#endif

/* This really shouldn't matter, either, since we won't be doing anything
 * with sockets.  But just in case I don't excise all the sockets code,
 * here it is:
 */
#if !defined(Sock_size_t)
# if defined(SPEC_LINUX)
#  define Sock_size_t socklen_t
# else
#  define Sock_size_t int
# endif
#endif

/* Ugh... the formatting mess!  Still, in the interest of validation, this
 * may be important.
 */
#if !defined(PERL_PRIfldbl)
# if defined(SPEC_LINUX_ALPHA) || defined(SPEC_PPC) || \
     defined(SPEC_LINUX_SPARC) || defined(SPEC_WINDOWS)
#  define PERL_PRIfldbl "f"
# else
#  define PERL_PRIfldbl "Lf"
# endif
#endif
#if !defined(PERL_PRIgldbl)
# if defined(SPEC_LINUX_ALPHA) || defined(SPEC_PPC) || \
     defined(SPEC_LINUX_SPARC) || defined(SPEC_WINDOWS)
#  define PERL_PRIgldbl "g"
# else
#  define PERL_PRIgldbl "Lg"
# endif
#endif
#if !defined(PERL_PRIeldbl)
# if defined(SPEC_LINUX_ALPHA) || defined(SPEC_PPC) || \
     defined(SPEC_LINUX_SPARC) || defined(SPEC_WINDOWS)
#  define PERL_PRIeldbl "e"
# else
#  define PERL_PRIeldbl "Le"
# endif
#endif
#if !defined(PERL_SCNfldbl)
# if defined(SPEC_LINUX_ALPHA) || defined(SPEC_PPC) || \
     defined(SPEC_LINUX_SPARC)
#  define PERL_SCNfldbl "f"
# elif !defined(SPEC_WINDOWS)
#  define PERL_SCNfldbl "Lf"
# endif
#endif

#if !defined(IVdf)
# define IVdf "ld"
#endif
#if !defined(UVuf)
# define UVuf "lu"
#endif
#if !defined(UVof)
# define UVof "lo"
#endif
#if !defined(UVxf)
# define UVxf "lx"
#endif
#if !defined(UVXf)
#  define UVXf "lX"
#endif
#if !defined(NVef)
#  define NVef "e"
#endif
#if !defined(NVff)
#  define NVff "f"
#endif
#if !defined(NVgf)
#  define NVgf "g"
#endif

/* Evidently SpamAssassin likes to make directories.  Lots and lots of
 * directories.  Defining HAS_MKDIR will keep Perl from doing a popen to
 * run 'mkdir'.
 */
#if !defined(SPEC_NO_HAS_MKDIR)
#  define HAS_MKDIR
#endif

/* Stuff that Windows doesn't have */
#if !defined(SPEC_WINDOWS)

# if !defined(SPEC_SOLARIS) && !defined(SPEC_NO_HAS_SIGSETJMP)
#  define HAS_SIGSETJMP
# endif

# if !defined(SPEC_NO_I_SYS_IOCTL)
#  define I_SYS_IOCTL
# endif

# if !defined(SPEC_NO_I_UNISTD)
#  define I_UNISTD
# endif

# if !defined(SPEC_NO_I_INTTYPES)
#  define I_INTTYPES
# endif

# if !defined(SPEC_NO_HAS_LONG_LONG)
#  define HAS_LONG_LONG
# endif

# if !defined(SPEC_NO_HAS_FCNTL)
#  define HAS_FCNTL
# endif

# if !defined(SPEC_NO_I_SYS_TIME)
#  define I_SYS_TIME
# endif

# if !defined(SPEC_NO_I_SYS_TIMES)
#  define I_SYS_TIMES
# endif

# if !defined(SPEC_NO_I_SYS_WAIT)
#  define I_SYS_WAIT
# endif

# if !defined(SPEC_NO_HAS_TRUNCATE)
#  define HAS_TRUNCATE
# endif

#endif /* !SPEC_WINDOWS */

#if !defined(SSize_t)
# if defined(SPEC_WINDOWS)
#  define SSize_t int
# else
#  define SSize_t ssize_t
# endif
#endif

#if !defined(Pid_t)
# if defined(SPEC_WINDOWS)
#  define Pid_t int
# else
#  define Pid_t pid_t
# endif
#endif

#if !defined(Select_fd_set_t)
# if defined(SPEC_WINDOWS)
#  define Select_fd_set_t 	Perl_fd_set *
# else
#  define Select_fd_set_t 	fd_set *
# endif
#endif

#if !defined(LSEEKSIZE)
#  define LSEEKSIZE 8
#endif
#if !defined(Off_t_size)
#  define Off_t_size 8
#endif

#if defined(SPEC) && !defined(SPEC_WINDOWS)
 /* Select definitions from unixish.h */
# ifndef PERL_SYS_INIT
#  define PERL_SYS_INIT_BODY(c,v)					\
	MALLOC_CHECK_TAINT2(*c,*v) PERL_FPU_INIT; PERLIO_INIT; MALLOC_INIT
# endif
# if !defined(BIT_BUCKET)
#  define BIT_BUCKET "/dev/null"
# endif
# if !defined(Stat)
# define Stat(fname,bufptr) stat((fname),(bufptr))
# endif
# if !defined(SPEC_NO_I_ARPA_INET)
#  define I_ARPA_INET
# endif
 /* End inclusion from unixish.h */
#elif defined(SPEC_WINDOWS)
 /* Select definitions from dosish.h */
# ifndef PERL_SYS_INIT_BODY
#  define PERL_SYS_INIT_BODY(c,v)					\
	MALLOC_CHECK_TAINT2(*c,*v) Perl_win32_init(c,v); PERLIO_INIT
# endif
# ifndef PERL_SYS_TERM_BODY
#  define PERL_SYS_TERM_BODY()   Perl_win32_term()
# endif
# if !defined(BIT_BUCKET)
#  define BIT_BUCKET "nul"
# endif
# if !defined(SPEC_NO_USE_STAT_RDEV)
#  define USE_STAT_RDEV
# if !defined(Off_t)
#  define Off_t __int64
# endif
# endif
 /* End inclusion from dosish.h */
#endif

/* Grrr...stdio optimizations
 * Normally these would just be turned off, but since some platforms
 * REQUIRE them, it's best that everyone have them.
 */
#if defined(SPEC_SOLARIS) && defined(SPEC_LP64)
# define SPEC_NO_USE_STDIO_PTR
# define SPEC_NO_USE_STDIO_BASE
#endif

#if !defined(SPEC_NO_USE_STDIO_PTR)
# define USE_STDIO_PTR

# if !defined(FILE_ptr)
#  if defined(SPEC_LINUX)
#   define FILE_ptr(fp) ((fp)->_IO_read_ptr)
#  elif defined(SPEC_TRU64) || defined(SPEC_HPUX) || \
       defined(SPEC_IRIX) || defined(SPEC_AIX) || \
       defined(SPEC_SOLARIS)
#   define FILE_ptr(fp) ((fp)->_ptr)
#  elif defined(SPEC_MACOSX)
#   define FILE_ptr(fp) ((fp)->_p)
#  else
#   undef USE_STDIO_PTR
#  endif
# endif
# if !defined(FILE_cnt)
#  if defined(SPEC_LINUX)
#   define FILE_cnt(fp) ((fp)->_IO_read_end - (fp)->_IO_read_ptr)
#  elif defined(SPEC_TRU64) || defined(SPEC_HPUX) || \
       defined(SPEC_IRIX) || defined(SPEC_AIX) || \
       defined(SPEC_SOLARIS)
#   define FILE_cnt(fp) ((fp)->_cnt)
#  elif defined(SPEC_MACOSX)
#   define FILE_cnt(fp) ((fp)->_r)
#  else
#   undef USE_STDIO_PTR
#  endif
# endif
#endif

#if !defined(SPEC_NO_USE_STDIO_BASE)
# define USE_STDIO_BASE

# if !defined(FILE_base)
#  if defined(SPEC_LINUX)
#   define FILE_base(fp) ((fp)->_IO_read_base)
#  elif defined(SPEC_TRU64) || defined(SPEC_HPUX) || \
       defined(SPEC_IRIX) || defined(SPEC_AIX) || \
       defined(SPEC_SOLARIS)
#   define FILE_base(fp) ((fp)->_base)
#  elif defined(SPEC_MACOSX)
#   define FILE_base(fp) ((fp)->_ub._base ? (fp)->_ub._base : (fp)->_bf._base)
#  else
#   undef USE_STDIO_BASE
#  endif
# endif
# if !defined(FILE_bufsiz)
#  if defined(SPEC_LINUX)
#   define FILE_bufsiz(fp) ((fp)->_IO_read_end - (fp)->_IO_read_base)
#  elif defined(SPEC_TRU64) || defined(SPEC_HPUX) || \
       defined(SPEC_IRIX) || defined(SPEC_AIX) || \
       defined(SPEC_SOLARIS)
#   define FILE_bufsiz(fp) ((fp)->_cnt + (fp)->_ptr - (fp)->_base)
#  elif defined(SPEC_MACOSX)
#   define FILE_bufsiz(fp) ((fp)->_ub._base ? (fp)->_ub._size : (fp)->_bf._size)
#  else
#   undef USE_STDIO_BASE
#  endif
# endif
#endif

/* EVERYONE uses SPEC's random number generator! */
#define Drand01()		spec_rand()
#define Rand_seed_t		unsigned long
#define seedDrand01(x)		spec_init_genrand((Rand_seed_t)x)
#define RANDBITS		31

/* NOBODY gets randomized hash keys! */
#define PERL_HASH_SEED (const unsigned char * const)"Too Hot To Hoot!"

/****************************************************************************
 * This section includes general defines (and undefs) to help in not utilizing
 * parts of the OS that we don't want to benchmark, as well as specifying the
 * bits of the compilation environment (like ANSI compliance) that SPEC
 * requires.
 * All of the symbols here have an explanatory section farther down in the
 * file.
 ****************************************************************************/

#if defined(SPEC)
/* Things to never use */
# undef USE_DYNAMIC_LOADING
# undef HAS_CRYPT
# undef HASATTRIBUTE
# undef HAS_CHOWN
# undef HAS_CHROOT
# undef HAS_PHOSTNAME
# undef HAS_CSH
# undef PERL_MALLOC_WRAP
# if !defined(SPEC_REALLY_USE_LOCALE)
#  define NO_LOCALE
# endif

/* Things everyone has, needs, or can do (probably ANSI; I don't know) */
# if !defined(SPEC_NO_HASCONST)
#  define HASCONST
# endif
# if !defined(SPEC_NO_CAN_PROTOTYPE)
#  define CAN_PROTOTYPE
# endif
# if !defined(SPEC_NO_HAS_GETCWD)
#  define HAS_GETCWD
# endif
# if !defined(SPEC_NO_HAS_ISNAN)
#  define HAS_ISNAN
# endif
# if !defined(SPEC_NO_HAS_ISINF)
#  define HAS_ISINF
# endif
# if !defined(SPEC_NO_HAS_MEMCHR)
#  define HAS_MEMCHR
# endif
# if !defined(SPEC_NO_HAS_MEMCMP)
#  define HAS_MEMCMP
# endif
# if !defined(SPEC_NO_HAS_SANE_MEMCMP)
#  define HAS_SANE_MEMCMP
# endif
# if !defined(SPEC_NO_HAS_MEMCPY)
#  define HAS_MEMCPY
# endif
# if !defined(SPEC_NO_HAS_MEMMOVE)
#  define HAS_MEMMOVE
# endif
# if !defined(SPEC_NO_HAS_MEMSET)
#  define HAS_MEMSET
# endif
# if !defined(SPEC_NO_HAS_SELECT)
#  define HAS_SELECT
# endif
# if !defined(SPEC_NO_HAS_SIGNBIT)
#  define HAS_SIGNBIT
# endif
# if !defined(SPEC_NO_HAS_STRCHR)
#  define HAS_STRCHR
# endif
# if !defined(SPEC_NO_HAS_STRTOD)
#  define HAS_STRTOD
# endif
# if !defined(SPEC_NO_HAS_STRTOL)
#  define HAS_STRTOL
# endif
# if !defined(SPEC_NO_HAS_STRXFRM)
#  define HAS_STRXFRM
# endif
# if !defined(SPEC_NO_HASVOLATILE)
#  define HASVOLATILE
# endif
# if !defined(SPEC_NO_HAS_VPRINTF)
#  define HAS_VPRINTF
# endif
# if !defined(SPEC_NO_HAS_DBL_DIG)
#  define HAS_DBL_DIG
# endif
# if !defined(SPEC_NO_HAS_LDBL_DIG)
#  define HAS_LDBL_DIG
# endif
# if !defined(SPEC_NO_HAS_LONG_DOUBLE)
#  define HAS_LONG_DOUBLE
# endif
# if defined(HAS_LONG_LONG) && !defined(LONGLONGSIZE)
#  define LONGLONGSIZE 8
# endif
# if !defined(SPEC_NO_FLEXFILENAMES)
#  define FLEXFILENAMES
# endif
# if !defined(SPEC_NO_U32_ALIGNMENT_REQUIRED)
#  define U32_ALIGNMENT_REQUIRED
# endif
# if !defined(SPEC_NO_I_UTIME)
#  define I_UTIME
# endif
# if !defined(SPEC_NO_I_DIRENT)
#  define I_DIRENT
# endif
# if !defined(SPEC_NO_I_FLOAT)
#  define I_FLOAT
# endif
# if !defined(SPEC_NO_I_LIMITS)
#  define I_LIMITS
# endif
# if !defined(SPEC_NO_I_MATH)
#  define I_MATH
# endif
# if !defined(SPEC_NO_I_SYS_STAT)
#  define I_SYS_STAT
# endif
# if !defined(SPEC_NO_I_STDARG)
#  define I_STDARG
# endif
# if !defined(SPEC_NO_I_STDLIB)
#  define I_STDLIB
# endif
# if !defined(SPEC_NO_I_STRING)
#  define I_STRING
# endif
# if !defined(SPEC_NO_STRUCT_COPY)
#  define USE_STRUCT_COPY
# endif
# if !defined(SPEC_NO_HAS_RMDIR)
#  define HAS_RMDIR
# endif
# if !defined(SPEC_NO_HAS_GETTIMEOFDAY)
#  define HAS_GETTIMEOFDAY
# endif

/* Stuff that's the same in unixish.h and dosish.h */
# if !defined(Fstat)
# define Fstat(fd,bufptr)   fstat((fd),(bufptr))
# endif
# if !defined(Fflush)
# define Fflush(fp)         fflush(fp)
# endif
# if !defined(Mkdir)
# define Mkdir(path,mode)   mkdir((path),(mode))
# endif
# if !defined(Stat_t)
#  define Stat_t struct stat
# endif
# if !defined(SPEC_NO_dXSUB_SYS)
#  define dXSUB_SYS
# endif
# ifndef PERL_SYS_TERM_BODY
#  define PERL_SYS_TERM_BODY() \
    HINTS_REFCNT_TERM; OP_REFCNT_TERM; PERLIO_TERM; MALLOC_TERM;
# endif
# if !defined(SPEC_NO_HAS_IOCTL)
#  define HAS_IOCTL
# endif
# if !defined(SPEC_NO_HAS_WAITPID)
#  define HAS_WAITPID
# endif
/* End common stuff from unixish.h and dosish.h */

/* Safe, but overrideable defaults */
# if !defined(Strerror)
#  define Strerror(e) strerror(e)
# endif
# if !defined(Time_t)
#  define Time_t time_t
# endif
# if !defined(Signal_t)
#  define Signal_t void
# endif
# if !defined(Fpos_t)
#  define Fpos_t fpos_t
# endif
# if !defined(Off_t)
#  define Off_t off_t
# endif
# if !defined(Malloc_t)
#  define Malloc_t void *
# endif
# if !defined(Free_t)
#  define Free_t void
# endif
# if !defined(Mode_t)
#  define Mode_t mode_t
# endif
# if !defined(Size_t)
#  define Size_t size_t
# endif
# if !defined(VAL_O_NONBLOCK)
#  define VAL_O_NONBLOCK O_NONBLOCK
# endif
# if !defined(VAL_EAGAIN)
#  define VAL_EAGAIN EAGAIN
# endif
# if !defined(RD_NODATA)
#  define RD_NODATA -1
# endif
# if !defined(Uid_t_size)
#  define Uid_t_size 4
# endif
# if !defined(Uid_t)
#  define Uid_t uid_t
# endif
# if !defined(Gid_t_size)
#  define Gid_t_size 4
# endif
# if !defined(Gid_t)
#  define Gid_t gid_t
# endif

/* SPECification */
# define OSNAME		"spec-cpu"
# define OSVERS		"2015"
# define	USE_PERLIO

/* Generificiation :) */
# define LOC_SED		""
# define CSH		""
/* It's a good thing we never use this; it wouldn't work anyway :) */
# define STARTPERL	"#!./perlbench"
# define PHOSTNAME	""
# define ARCHLIB		""
# define ARCHLIB_EXP	""
# define ARCHNAME	"spec-cpu"
# define BIN		""
# define BIN_EXP		""
# define INSTALL_PREFIX		""
# define INSTALL_PREFIX_EXP	""
# define PRIVLIB		""
# define PRIVLIB_EXP	""
# define SITEARCH	""
# define SITEARCH_EXP	""
# define SITELIB		""
# define SITELIB_EXP	""
# define SITELIB_STEM	""
# define SIG_NAME "ZERO", "HUP", "INT", "QUIT", "ILL", "TRAP", "ABRT", "BUS", "FPE", "KILL", "USR1", "SEGV", "USR2", "PIPE", "ALRM", "TERM", 0
# define SIG_NUM  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0
# if !defined(SIG_SIZE)
#  define SIG_SIZE	16
# endif
# define PERL_INC_VERSION_LIST 0
#endif /* SPEC */

/*****************************************************************************/
/*****************************************************************************/
/*****************************************************************************/
/*****************************************************************************/
/*****************************************************************************/
/*****************************************************************************/
#include "perl-orig-config.h"

#endif /* _specconfig_h */
