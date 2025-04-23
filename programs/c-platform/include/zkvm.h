#ifndef ZKVM_H
#define ZKVM_H

typedef unsigned int uint;
#define size_t uint

#ifndef int64_t
#define int64_t long
#endif

#ifndef uint64_t
#define uint64_t unsigned long
#endif

#ifndef int32_t
#define int32_t int
#endif

#ifndef uint32_t
#define uint32_t unsigned int
#endif

#ifndef uint8_t
#define uint8_t unsigned char
#endif

#ifndef NULL
#define NULL 0
#endif

#ifdef __clang__
  #ifdef __cplusplus
    #define DEFINE_FUNCTION(name, ret_type, ...) extern "C" ret_type name(__VA_ARGS__)
  #else
    #define DEFINE_FUNCTION(name, ret_type, ...) extern ret_type name(__VA_ARGS__)
  #endif
#endif

DEFINE_FUNCTION(printf, int, const char *fmt, ...);
DEFINE_FUNCTION(malloc, void *, size_t size);
DEFINE_FUNCTION(calloc, void *, size_t nmemb, size_t size);
DEFINE_FUNCTION(realloc, void *, void *ptr, size_t size);
DEFINE_FUNCTION(free, void, void *ptr);
DEFINE_FUNCTION(exit, void, int status);
DEFINE_FUNCTION(sqrt, double, double x);
DEFINE_FUNCTION(memset, void *, void *s, int c, size_t n);
DEFINE_FUNCTION(strcpy, char *, char *dest, const char *src);
DEFINE_FUNCTION(atoi, int, const char *str);
DEFINE_FUNCTION(isdigit, int, int arg);
DEFINE_FUNCTION(isalpha, int, int argument);
DEFINE_FUNCTION(strlen, size_t, const char *str);
DEFINE_FUNCTION(strncmp, int, const char *s1, const char *s2, size_t n);
DEFINE_FUNCTION(strstr, char *, const char *haystack, const char *needle);
DEFINE_FUNCTION(sprintf, int, char *str, const char *format, ...);
DEFINE_FUNCTION(strcat, char *, char *dest, const char *src);
DEFINE_FUNCTION(memcpy, void *, void *dest, const void *src, size_t n);
DEFINE_FUNCTION(abs, int, int x);
DEFINE_FUNCTION(strcmp, int, const char *s1, const char *s2);
DEFINE_FUNCTION(read_int, int, void);
DEFINE_FUNCTION(read_string, char *, void);
DEFINE_FUNCTION(putchar, int, int);
DEFINE_FUNCTION(puts, int, char *);

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) > (b) ? (b) : (a))

#ifdef NDEBUG
  #define assert(expr) ((void)0)
#else
  #define assert(expr) \
    ((expr) ? (void)0 : \
      (printf("Assertion failed: %s, file %s, line %d\n", \
               #expr, __FILE__, __LINE__), exit(-1)))
#endif

#endif
