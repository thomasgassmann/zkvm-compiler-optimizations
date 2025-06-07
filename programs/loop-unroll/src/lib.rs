pub fn matmul(mat: &[[f64; 5]; 5], vec: &[f64; 5]) -> [f64; 5] {
    let mut res = [0.0; 5];
    for col in 0..5 {
        for row in 0..5 {
            res[row] += mat[col][row] * vec[col];
        }
    }

    res
}

#[no_mangle]
#[cfg(feature = "x86")]
pub extern "C" fn main_core(reps: usize) -> () {
    let matrix = [
        [1.0, 2.0, 3.0, 4.0, 5.0],
        [2.0, 3.0, 4.0, 5.0, 6.0],
        [3.0, 4.0, 5.0, 6.0, 7.0],
        [4.0, 5.0, 6.0, 7.0, 8.0],
        [5.0, 6.0, 7.0, 8.0, 9.0],
    ];
    let vector = [1.0, 2.0, 4.0, 8.0, 16.0];
    for _ in 0..reps {
        let res = matmul(&matrix, &vector);
        std::hint::black_box(res);
    }
}
