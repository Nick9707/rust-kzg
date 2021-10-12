mod kzg_types;

use rand::{thread_rng, Rng};
use blst::min_pk::*;
use blst::BLST_ERROR;

mod single_commit;
//mod lib;

// wrapper function for secret key generation
fn gen_sk() -> SecretKey {
    let mut ikm = [0u8; 32];
    thread_rng().try_fill(&mut ikm).unwrap();
    SecretKey::key_gen(&ikm, &[]).unwrap()
}

fn main() {
    // Rust Hello World
    let entity= "Rust";
    println!("Hello, {}!", entity);

    // BLS12-381 Hello World
    // Modified example from https://github.com/supranational/blst/tree/master/bindings/rust
    let mut sk = gen_sk();
    let pk = sk.sk_to_pk();

    let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
    let msg = b"blst is such a blast";
    let mut sig = sk.sign(msg, dst, &[]);

    // The following deviates from the example because the example is outdated
    let err = sig.verify(false, msg, dst, &[], &pk, true);
    assert_eq!(err, BLST_ERROR::BLST_SUCCESS);

    // And here we will fail
    sk = gen_sk();
    sig = sk.sign(msg, dst, &[]);
    let err = sig.verify(false, msg, dst, &[], &pk, true);
    assert_eq!(err, BLST_ERROR::BLST_VERIFY_FAIL);

    //single_commit::commit_to_poly(out: P1, poly: kzg_types::Poly, kzg_settings: kzg_types::KZGSettings)
    //let coff: blst::blst_fr = blst::blst_fr::from_str();
    //let a: single_commit::Poly = single_commit::Poly { length: 10, coeffs: coff};
    //kzg::single_commit::commit_to_poly(1, 1, 1);
}
