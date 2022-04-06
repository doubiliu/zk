#[cfg(test)]
mod test {
    use crate::bls_extern::*;
    use bls12_381::*;
    use std::mem::transmute;

    #[test]
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
}
