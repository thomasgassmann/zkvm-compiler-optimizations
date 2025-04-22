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

extern void printf(const char *fmt, ...);
extern void *malloc(size_t size);
extern void *calloc(size_t nmemb, size_t size);
extern void *realloc(void *ptr, size_t size);
extern void free(void *ptr);
extern void exit(int status);
extern double sqrt(double x);
extern void *memset(void *s, int c, size_t n);
extern char *strcpy(char *dest, const char *src);
extern int atoi(const char *str);
extern int isdigit(int arg);
extern int isalpha(int argument);
extern size_t strlen(const char *str);
extern int strncmp(const char *s1, const char *s2, size_t n);
extern char *strstr(const char *haystack, const char *needle);
extern int sprintf(char *str, const char *format, ...);
extern char *strcat(char *dest, const char *src);
extern void *rmemcpy(void *dest, const void *src, size_t n);
extern int abs(int x);
extern int strcmp(const char *s1, const char *s2);
extern void *rmemset(void *ptr, int x, size_t n);


extern int read_int();

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
