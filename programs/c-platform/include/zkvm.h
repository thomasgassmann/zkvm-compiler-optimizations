#ifndef ZKVM_H
#define ZKVM_H

typedef unsigned int uint;
#define size_t uint
#define NULL 0

extern void printf(const char *fmt, ...);
extern void *malloc(size_t size);
extern void free(void *ptr);
extern void exit(int status);
extern double sqrt(double x);

extern int read_int();

#endif
