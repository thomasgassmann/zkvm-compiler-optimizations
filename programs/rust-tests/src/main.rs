#![no_main]

mod zip;
#[path="./vec-slices.rs"]
mod vec_slices;


#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);


fn main() {
    zip::zip_copy(&[1,2,3,4], &mut [2,3,4,1]);
    zip::zip_copy_mapped(&[1,2,3,4], &mut [2,3,4,1]);
    vec_slices::vecslices();
}
