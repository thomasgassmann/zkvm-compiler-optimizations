use ed25519_dalek::{Signature, Verifier, VerifyingKey};

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn main_core(items: Vec<(VerifyingKey, Vec<u8>, Signature)>) {
    for (encoded_verifying_key, message, signature) in items {
        verify_inner(encoded_verifying_key, message, signature);
    }
}

fn verify_inner(signer: VerifyingKey, message: Vec<u8>, signature: Signature) {
    signer
        .verify(&message, &signature)
        .expect("Ed25519 signature verification failed");

    core::hint::black_box(&(signer, message));
}
