use bls12_381::{
    multi_miller_loop, pairing, G1Affine, G1Projective, G2Affine, G2Prepared, G2Projective, Gt,
    MillerLoopResult, Scalar,
};

pub fn bytes_g1_mul(g1_bytes: [u8; 96], x: u64) -> [u8; 96] {
    let g1 = bytes_to_g1(g1_bytes);
    let x = Scalar::from(x);
    let g1_result = G1Affine::from(g1 * x);
    g1_to_bytes(g1_result)
}

pub fn bytes_g2_mul(g2_bytes: [u8; 192], x: u64) -> [u8; 192] {
    let g2 = bytes_to_g2(g2_bytes);
    let x = Scalar::from(x);
    let g2_result = G2Affine::from(g2 * x);
    g2_to_bytes(g2_result)
}

pub fn bytes_g1_add(g1_1_bytes: [u8; 96], g1_2_bytes: [u8; 96]) -> [u8; 96] {
    let g1_1 = bytes_to_g1(g1_1_bytes);
    let g1_2 = bytes_to_g1(g1_2_bytes);
    let g1_1 = G1Projective::from(&g1_1);
    let g1_2 = G1Projective::from(&g1_2);
    let g1_result = G1Affine::from(g1_1 + g1_2);
    g1_to_bytes(g1_result)
}

pub fn bytes_g2_add(g2_1_bytes: [u8; 192], g2_2_bytes: [u8; 192]) -> [u8; 192] {
    let g2_1 = bytes_to_g2(g2_1_bytes);
    let g2_2 = bytes_to_g2(g2_2_bytes);
    let g2_1 = G2Projective::from(&g2_1);
    let g2_2 = G2Projective::from(&g2_2);
    let g2_result = G2Affine::from(g2_1 + g2_2);
    g2_to_bytes(g2_result)
}

use std::mem::*;
pub fn g1_to_bytes(g1: G1Affine) -> [u8; 96] {
    g1.to_uncompressed()
}

pub fn bytes_to_g1(bytes: [u8; 96]) -> G1Affine {
    G1Affine::from_uncompressed(&bytes).unwrap()
}

pub fn g1_bytes_neg(bytes: [u8; 96]) -> G1Affine {
    let g1 = G1Affine::from_uncompressed(&bytes).unwrap();
    -g1
}

pub fn g2_to_bytes(g2: G2Affine) -> [u8; 192] {
    g2.to_uncompressed()
}

pub fn bytes_to_g2(bytes: [u8; 192]) -> G2Affine {
    G2Affine::from_uncompressed(&bytes).unwrap()
}

pub fn g2_bytes_neg(bytes: [u8; 192]) -> G2Affine {
    let g2 = G2Affine::from_uncompressed(&bytes).unwrap();
    -g2
}

pub fn g1pair(bytes: [u8; 32]) -> G1Affine {
    let x = Scalar::from_bytes(&bytes);
    let result = G1Affine::generator() * x.unwrap();
    G1Affine::from(result)
}

pub fn bytes_to_gt(bytes: [u64; 72]) -> Gt {
    let gt = unsafe { transmute::<[u64; 72], Gt>(bytes) };
    gt
}

pub fn gt_to_bytes(gt: Gt) -> [u64; 72] {
    let bytes = unsafe { transmute::<Gt, [u64; 72]>(gt) };
    bytes
}

pub fn gt_bytes_neg(bytes: [u64; 72]) -> Gt {
    let gt = unsafe { transmute::<[u64; 72], Gt>(bytes) };
    -gt
}

pub fn bytes_multi_miller_loop(g1_bytes: [u8; 96], g2_bytes: [u8; 192]) -> [u64; 72] {
    let g1 = G1Affine::from_uncompressed(&g1_bytes).unwrap();
    let g2 = G2Affine::from_uncompressed(&g2_bytes).unwrap();
    let g2_pre = G2Prepared::from(g2);
    let result = multi_miller_loop(&[(&g1, &g2_pre)]).final_exponentiation();
    gt_to_bytes(result)
}

pub fn bytes_pairing(g1_bytes: [u8; 96], g2_bytes: [u8; 192]) -> [u64; 72] {
    let g1affine = G1Affine::from_uncompressed(&g1_bytes).unwrap();
    let g2affine = G2Affine::from_uncompressed(&g2_bytes).unwrap();
    let result = pairing(&g1affine, &g2affine);
    gt_to_bytes(result)
}

pub fn bytes_gt_add(a: [u64; 72], b: [u64; 72]) -> [u64; 72] {
    let a = bytes_to_gt(a);
    let b = bytes_to_gt(b);
    gt_to_bytes(a + b)
}

#[test]//测试方法要放到测试目录 和src平级
pub fn test_transmut_gt() {
    let x = Gt::default();
    let g1p = G1Projective::generator();
    let y = G1Affine::from(&g1p);
    let fp_x = unsafe { transmute::<Gt, [u64; 72]>(x) };
    let x_test = unsafe { transmute::<[u64; 72], Gt>(fp_x) };
    assert_eq!(x, x_test);
}

#[test]
fn test_g_serialize_deserialize() {
    let g1_identity = G1Affine::identity();
    let g1_bytes = g1_to_bytes(g1_identity);
    let g1_test = bytes_to_g1(g1_bytes);
    let g2_identity = G2Affine::identity();
    let g2_bytes = g2_to_bytes(g2_identity);
    let g2_test = bytes_to_g2(g2_bytes);
    let result = MillerLoopResult::default().final_exponentiation();
    let result_bytes = gt_to_bytes(result);
    let result_test = bytes_to_gt(result_bytes);
    assert_eq!(g2_identity.eq(&g2_test), true);
    assert_eq!(g1_identity.eq(&g1_test), true);
    assert_eq!(result, result_test);
}

#[test]
fn test_bytes_pairing_loop() {
    let a1 = G1Affine::generator();
    let b1 = G2Affine::generator();
    let b1_prepared = G2Prepared::from(b1);
    let expected = pairing(&a1, &b1);
    let test =
        multi_miller_loop(&[(&a1, &b1_prepared), (&a1, &b1_prepared)]).final_exponentiation();
    let result_pairings = expected + expected;
    assert_eq!(result_pairings, test);
}

#[test]
fn test_bytes_pairing() {
    let g1 = G1Affine::from(G1Projective::generator());
    let g2 = G2Affine::from(G2Projective::generator());
    let g2_pre = G2Prepared::from(g2);
    let result = pairing(&g1, &g2);
    let g1_bytes = g1_to_bytes(g1);
    let g2_bytes = g2_to_bytes(g2);
    let result_bytes = bytes_pairing(g1_bytes, g2_bytes);
    let result_test = bytes_to_gt(result_bytes);
    let result_loop = multi_miller_loop(&[(&g1, &g2_pre)]).final_exponentiation();
    assert_eq!(result, result_loop);
    assert_eq!(result, result_test);
}

#[test]
pub fn test_g1_ptr() {
    let g1 = G1Affine::generator();
    let g1_bytes = g1_to_bytes(g1);
    let g1_generator_ptr = &g1_bytes as *const u8;
    let arr1 = unsafe { std::slice::from_raw_parts(g1_generator_ptr, 96) };
    let mut g1_test = [0u8; 96];
    g1_test[0..96].copy_from_slice(&arr1);
    assert_eq!(g1_bytes, g1_test);
}

#[test]

pub fn test_g1_add() {
    let g1 = G1Affine::generator();
    let g1p = G1Projective::from(&g1);
    let result = g1p + g1p;
    let g1b = g1_to_bytes(g1);
    let resultb = bytes_g1_add(g1b, g1b);
    let result_test = bytes_to_g1(resultb);
    assert_eq!(G1Affine::from(result), result_test);
}

#[test]
pub fn test_g1_mul() {
    let g1 = G1Affine::generator();
    let result = g1 * Scalar::from(3);
    let g1b = g1_to_bytes(g1);
    let resultb = bytes_g1_mul(g1b, 3);
    let result_test = bytes_to_g1(resultb);
    assert_eq!(G1Affine::from(result), result_test);
}
