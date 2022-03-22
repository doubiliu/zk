pub mod bls_extern;
use crate::bls_extern::*;
use bls12_381::{
    multi_miller_loop, pairing, G1Affine, G1Projective, G2Affine, G2Prepared, G2Projective, Gt,
    MillerLoopResult,
};
use std::slice::*;

#[no_mangle]
pub extern "C" fn gt_add(gt1: *mut gtobject, gt2: *mut gtobject) -> *const gtobject {
    let gt1_bytes = unsafe { gt1.as_mut().expect("gt_add::invalid gt1 ptr").val };
    let gt2_bytes = unsafe { gt2.as_mut().expect("gt_add::invalid gt2 ptr").val };
    let result = bytes_gt_add(gt1_bytes, gt2_bytes);
    println!("gt_add result: {:?}", result);
    let resultobj = gtobject { val: result };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn gt_mul(gt1: *mut gtobject, mul: u64) -> *const gtobject {
    let gt1_bytes = unsafe { gt1.as_mut().expect("gt_add::invalid gt1 ptr").val };
    let mut result = [0u64; 72];
    for i in 0..mul {
        result = bytes_gt_add(gt1_bytes, gt1_bytes);
    }
    println!("gt_add result: {:?}", result);
    let resultobj = gtobject { val: result };
    let b = Box::new(resultobj);
    Box::into_raw(b)
}

#[no_mangle] //不写这个会把函数名编译成其他乱七八糟的
pub extern "C" fn g1_g2_pairing(g1: *mut g1object, g2: *mut g2object) -> *const gtobject {
    let g1 = unsafe { g1.as_mut().expect("g1_g2_pairing::invalid g1 ptr").val };
    let g2 = unsafe { g2.as_mut().expect("g1_g2_pairing::invalid g2 ptr").val };
    let result = bytes_pairing(g1, g2);
    println!("g1_g2_pairing result: {:?}", result);
    let resultobj = gtobject { val: result };
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
pub extern "C" fn g1_add(g1_1: *mut g1object, g1_2: *mut g1object) -> *const g1object {
    let g1_1_bytes = unsafe { g1_1.as_mut().expect("g1_add::invalid g1_1 ptr").val };
    let g1_2_bytes = unsafe { g1_2.as_mut().expect("g1_add::invalid g1_2 ptr").val };
    let result_bytes = bytes_g1_add(g1_1_bytes, g1_2_bytes);
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
    println!("rust传出的东西： {:?}", obj.val);
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
pub extern "C" fn g1_check(ptr: *mut g1object) {
    let obj = unsafe { ptr.as_mut().expect("invalid ptr") };
    println!("传回rust的东西： {:?}", obj.val)
}

#[no_mangle]
pub extern "C" fn g2_generator() -> *const g2object {
    let g2_bytes = g2_to_bytes(G2Affine::generator());
    let obj = g2object { val: g2_bytes };
    println!("rust传出的东西： {:?}", obj.val);
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
pub extern "C" fn g2_check(ptr: *mut g2object) {
    let obj = unsafe { ptr.as_mut().expect("invalid ptr") };
    println!("传回rust的东西： {:?}", obj.val)
}

#[no_mangle]
pub extern "C" fn gt_identity() -> *const gtobject {
    let gt_bytes = gt_to_bytes(Gt::identity());
    let obj = gtobject { val: gt_bytes };
    println!("rust传出的东西： {:?}", obj.val);
    let b = Box::new(obj);
    return Box::into_raw(b);
}

#[no_mangle]
pub extern "C" fn gt_dispose(ptr: *mut gtobject) {
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn gt_check(ptr: *mut gtobject) {
    let obj = unsafe { ptr.as_mut().expect("invalid ptr") };
    println!("传回rust的东西： {:?}", obj.val)
}

#[no_mangle]
pub extern "C" fn test_generator_pairing(ptr: *mut gtobject) {
    let g1 = G1Affine::generator();
    let g2 = G2Affine::generator();
    let result = pairing(&g1, &g2);
    let result_test_bytes = unsafe { ptr.as_mut().expect("invalid ptr") };
    let result_test = bytes_to_gt(result_test_bytes.val);
    assert_eq!(result, result_test);
}