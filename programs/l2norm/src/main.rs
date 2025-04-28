#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

pub const ISIZ1: usize = 5;
pub const ISIZ2: usize = 5;
pub const ISIZ3: usize = 5;
pub const NZ0: usize = ISIZ3;
pub const NX0: usize = ISIZ1;
pub const NY0: usize = ISIZ2;
pub const NX: usize = NX0;
pub const JST: usize = 1;
pub const NY: usize = NY0;
pub const JEND: usize = NY - 1;
pub const IST: usize = 1;
pub const IEND: usize = NX - 1;

fn main() {
    let mut v = [[[[0.0; 5]; ISIZ1 + 1]; ISIZ2 + 1]; ISIZ3 + 1];
    for k in 0..ISIZ3 + 1 {
        for j in 0..ISIZ2 + 1 {
            for i in 0..ISIZ1 + 1 {
                for m in 0..5 {
                    v[k][j][i][m] = (k + j + i + m) as f64;
                }
            }
        }
    }

    let mut sum = [0.0; 5];
    for k in 1..NZ0 - 1 {
        for j in JST..JEND {
            for i in IST..IEND {
                for m in 0..5 {
                    sum[m] = sum[m] + v[k][j][i][m] * v[k][j][i][m];
                }
            }
        }
    }
    sum.iter_mut().for_each(|sum| {
        *sum = f64::sqrt(*sum / ((NX0 - 2) * (NY0 - 2) * (NZ0 - 2)) as f64);
    });

    println!("L2 Norms: {:?}", sum);
}
