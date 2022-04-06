pub mod bls_extern;
use crate::bls_extern::*;
use bls12_381::{pairing, G1Affine, G2Affine, Gt};

#[no_mangle]
pub extern "C" fn gt_add(gt1: *mut gtobject, gt2: *mut gtobject) -> *const gtobject {
    let gt1_bytes = unsafe { gt1.as_mut().expect("gt_add::invalid gt1 ptr").val };
    let gt2_bytes = unsafe { gt2.as_mut().expect("gt_add::invalid gt2 ptr").val };
    let result = bytes_gt_add(gt1_bytes, gt2_bytes);
    println!("gt_add result: {:?}", result);//这里删除
    let resultobj = gtobject { val: result };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn gt_mul(gt1: *mut gtobject, mul: u64) -> *const gtobject {
    use bls12_381::Scalar;
    let gt1_bytes = unsafe { gt1.as_mut().expect("gt_add::invalid gt1 ptr").val };
    let gt = bytes_to_gt(gt1_bytes);
    let gt_r = gt * Scalar::from(mul);
    let result = gt_to_bytes(gt_r);
    let resultobj = gtobject { val: result };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn g1_g2_pairing(g1: *mut g1object, g2: *mut g2object) -> *const gtobject {
    let g1 = unsafe { g1.as_mut().expect("g1_g2_pairing::invalid g1 ptr").val };
    let g2 = unsafe { g2.as_mut().expect("g1_g2_pairing::invalid g2 ptr").val };
    let result = bytes_pairing(g1, g2);
    println!("g1_g2_pairing result: {:?}", result);//这里删除
    let resultobj = gtobject { val: result };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}
#[no_mangle]
pub extern "C" fn gt_neg(gt: *mut gtobject) -> *const gtobject {
    let gt_bytes = unsafe { gt.as_mut().expect("gt_neg::invalid gt ptr").val };
    let result = gt_bytes_neg(gt_bytes);
    let result_bytes = gt_to_bytes(result);
    let resultobj = gtobject { val: result_bytes };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}

pub struct g1object {
    pub val: [u8; 96],
}

pub struct g2object {
    pub val: [u8; 192],
}

pub struct gtobject {
    pub val: [u64; 72],
}

#[no_mangle]
pub extern "C" fn gt_neg_mul() -> *const gtobject {
    use bls12_381::Scalar;
    let mut gt1 = pairing(&G1Affine::generator(), &G2Affine::generator());
    gt1 = gt1 * Scalar::from(3);
    let result_bytes = gt_to_bytes(-gt1);
    let resultobj = gtobject { val: result_bytes };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn g1_add(g1_1: *mut g1object, g1_2: *mut g1object) -> *const g1object {
    let g1_1_bytes = unsafe { g1_1.as_mut().expect("g1_add::invalid g1_1 ptr").val };
    let g1_2_bytes = unsafe { g1_2.as_mut().expect("g1_add::invalid g1_2 ptr").val };
    let result_bytes = bytes_g1_add(g1_1_bytes, g1_2_bytes);
    let resultobj = g1object { val: result_bytes };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn g1_neg(g1: *mut g1object) -> *const g1object {
    let g1_bytes = unsafe { g1.as_mut().expect("g1_neg::invalid g1 ptr").val };
    let result = g1_bytes_neg(g1_bytes);
    let result_bytes = g1_to_bytes(result);
    let resultobj = g1object { val: result_bytes };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn g2_add(g2_1: *mut g2object, g2_2: *mut g2object) -> *const g2object {
    let g2_1_bytes = unsafe { g2_1.as_mut().expect("g2_add::invalid g2_1 ptr").val };
    let g2_2_bytes = unsafe { g2_2.as_mut().expect("g2_add::invalid g2_2 ptr").val };
    let result_bytes = bytes_g2_add(g2_1_bytes, g2_2_bytes);
    let resultobj = g2object { val: result_bytes };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}
#[no_mangle]
pub extern "C" fn g2_neg(g2: *mut g2object) -> *const g2object {
    let g2_bytes = unsafe { g2.as_mut().expect("g2_neg::invalid g2 ptr").val };
    let result = g2_bytes_neg(g2_bytes);
    let result_bytes = g2_to_bytes(result);
    let resultobj = g2object { val: result_bytes };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn g1_mul(g1: *mut g1object, x: u64) -> *const g1object {
    let g1_bytes = unsafe { g1.as_mut().expect("g1_mul::invalid g1 ptr").val };
    let result_bytes = bytes_g1_mul(g1_bytes, x);
    let resultobj = g1object { val: result_bytes };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn g2_mul(g2: *mut g2object, x: u64) -> *const g2object {
    let g2_bytes = unsafe { g2.as_mut().expect("g2_mul::invalid g2 ptr").val };
    let result_bytes = bytes_g2_mul(g2_bytes, x);
    let resultobj = g2object { val: result_bytes };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn g1_generator() -> *const g1object {
    let g1_bytes = g1_to_bytes(G1Affine::generator());
    let obj = g1object { val: g1_bytes };
    println!("rust传出的东西： {:?}", obj.val);//这里删除
    let b = Box::new(obj);
    return Box::into_raw(b);
}

#[no_mangle]
pub extern "C" fn g1_dispose(ptr: *mut g1object) {
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn g2_generator() -> *const g2object {
    let g2_bytes = g2_to_bytes(G2Affine::generator());
    let obj = g2object { val: g2_bytes };
    println!("rust传出的东西： {:?}", obj.val);//这里删除
    let b = Box::new(obj);
    return Box::into_raw(b);
}

#[no_mangle]
pub extern "C" fn g2_dispose(ptr: *mut g2object) {
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn gt_dispose(ptr: *mut gtobject) {
    unsafe {
        Box::from_raw(ptr);
    }
}

