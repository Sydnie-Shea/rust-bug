use ark_std::test_rng;
use ark_bls12_381::Bls12_381 as ArkBls12_381;

fn main() {
    let mut rng = test_rng();
    let a_coeffs = (0..1)
        .map(|_| ArkBls12_381::G1Projective::prime_subgroup_generator() * ArkBls12_381::F::rand(&mut rng))
        .collect::<Vec<_>>();
    println!("Hello, world!");
}
