# SLOC of benchmark programs

Evaluated using `tokei -C ...`. The below total SLOC count does not include external library code.

| Program                  | External libraries/crates                                   | Uses precompiles | Rust SLOC | C/C++ SLOC | Total SLOC |
| ------------------------ | ----------------------------------------------------------- | ---------------- | --------- | ---------- | ---------- |
| bigmem                   | None                                                        | No               | 32        | 0          | 32         |
| ecdsa-verify             | k256                                                        | Yes              | 16        | 0          | 16         |
| eddsa-verify             | ed25519_dalek                                               | Yes              | 16        | 0          | 16         |
| factorial                | None                                                        | No               | 26        | 0          | 26         |
| keccak256                | sha3                                                        | Yes              | 13        | 0          | 13         |
| loop-sum                 | None                                                        | No               | 29        | 0          | 29         |
| merkle                   | rs_merkle                                                   | No               | 46        | 0          | 46         |
| npb-bt                   | None                                                        | No               | 2915      | 0          | 2915       |
| npb-cg                   | None                                                        | No               | 827       | 0          | 827        |
| npb-ep                   | None                                                        | No               | 485       | 0          | 485        |
| npb-ft                   | None                                                        | No               | 1151      | 0          | 1151       |
| npb-is                   | None                                                        | No               | 815       | 0          | 815        |
| npb-lu                   | None                                                        | No               | 2746      | 0          | 2746       |
| npb-mg                   | None                                                        | No               | 2121      | 0          | 2121       |
| npb-sp                   | None                                                        | No               | 2728      | 0          | 2728       |
| polybench-2mm            | None                                                        | No               | 113       | 0          | 113        |
| polybench-3mm            | None                                                        | No               | 148       | 0          | 148        |
| polybench-adi            | None                                                        | No               | 107       | 0          | 107        |
| polybench-atax           | None                                                        | No               | 78        | 0          | 78         |
| polybench-bicg           | None                                                        | No               | 80        | 0          | 80         |
| polybench-cholesky       | None                                                        | No               | 63        | 0          | 63         |
| polybench-correlation    | None                                                        | No               | 98        | 0          | 98         |
| polybench-covariance     | None                                                        | No               | 84        | 0          | 84         |
| polybench-deriche        | None                                                        | No               | 146       | 0          | 146        |
| polybench-doitgen        | None                                                        | No               | 83        | 0          | 83         |
| polybench-durbin         | None                                                        | No               | 67        | 0          | 67         |
| polybench-fdtd-2d        | None                                                        | No               | 93        | 0          | 93         |
| polybench-floyd-warshall | None                                                        | No               | 59        | 0          | 59         |
| polybench-gemm           | None                                                        | No               | 90        | 0          | 90         |
| polybench-gemver         | None                                                        | No               | 116       | 0          | 116        |
| polybench-gesummv        | None                                                        | No               | 81        | 0          | 81         |
| polybench-gramschmidt    | None                                                        | No               | 88        | 0          | 88         |
| polybench-heat-3d        | None                                                        | No               | 82        | 0          | 82         |
| polybench-jacobi-1d      | None                                                        | No               | 64        | 0          | 64         |
| polybench-jacobi-2d      | None                                                        | No               | 70        | 0          | 70         |
| polybench-lu             | None                                                        | No               | 64        | 0          | 64         |
| polybench-ludcmp         | None                                                        | No               | 103       | 0          | 103        |
| polybench-mvt            | None                                                        | No               | 79        | 0          | 79         |
| polybench-nussinov       | None                                                        | No               | 99        | 0          | 99         |
| polybench-seidel-2d      | None                                                        | No               | 70        | 0          | 70         |
| polybench-symm           | None                                                        | No               | 88        | 0          | 88         |
| polybench-syr2k          | None                                                        | No               | 85        | 0          | 85         |
| polybench-syrk           | None                                                        | No               | 81        | 0          | 81         |
| polybench-trisolv        | None                                                        | No               | 67        | 0          | 67         |
| polybench-trmm           | None                                                        | No               | 74        | 0          | 74         |
| regex-match              | regex                                                       | No               | 45        | 0          | 45         |
| rsp                      | serde, rsp-client-executor, c-kzg, bytemuck_derive, bincode | Yes              | 11        | 0          | 11         |
| sha2-bench               | sha2                                                        | No               | 36        | 0          | 36         |
| sha2-chain               | sha2                                                        | No               | 44        | 0          | 44         |
| sha3-bench               | sha3                                                        | No               | 35        | 0          | 35         |
| sha3-chain               | sha3                                                        | No               | 44        | 0          | 44         |
| sha256                   | None                                                        | No               | 163       | 0          | 163        |
| spec-605                 | None                                                        | No               | 48        | 2159 (C)   | 2207       |
| spec-619                 | None                                                        | No               | 53        | 203 (C)    | 256        |
| spec-631                 | None                                                        | No               | 46        | 6496 (C++) | 6542       |
| tailcall                 | None                                                        | No               | 56        | 0          | 56         |
| zkvm-mnist               | None                                                        | No               | 181       | 0          | 181        |
