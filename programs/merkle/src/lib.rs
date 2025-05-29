use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree};

pub fn exec(
    input: Vec<String>,
    range: std::ops::Range<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let leaves: Vec<[u8; 32]> = input.iter().map(|x| Sha256::hash(x.as_bytes())).collect();

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

    let leaves_to_prove = leaves
        .get(range.clone())
        .ok_or("can't get leaves to prove")?;
    let indices_to_prove: Vec<usize> = range.collect();
    let merkle_proof = merkle_tree.proof(&indices_to_prove);
    let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root")?;

    // Serialize proof to pass it to the client
    let proof_bytes = merkle_proof.to_bytes();

    // Parse proof back on the client
    let proof = MerkleProof::<Sha256>::try_from(proof_bytes)?;

    assert!(proof.verify(
        merkle_root,
        &indices_to_prove,
        leaves_to_prove,
        leaves.len()
    ));
    Ok(())
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
#[cfg(feature = "x86")]
pub extern "C" fn main_core(input: Vec<String>, range: std::ops::Range<usize>) -> () {
    exec(input, range).unwrap();
}
