# loop fission

Next, we present an example of loop fission, a compiler optimization that splits a loop into multiple loops to improve performance by enhancing locality and reducing cache misses. Below we show the example code before and after the optimization is applied:

```c
int i, a[N], b[N];
for (i = 0; i < N; i++) {
    a[i] = 1;
    b[i] = 2;
}
```

And after:

```c
int i, a[N], b[N];
for (i = 0; i < N; i++) {
    a[i] = 1;
}
for (i = 0; i < N; i++) {
    b[i] = 2;
}
```

## Conclusions

- Data locality is still important on zkVMs, even if memory access is relatively cheaper compared to traditional hardware architectures.
- Depending on array size, optimization can still be beneficial. In general however less so than on traditional architectures.
