﻿/* automatically generated by rust-bindgen 0.55.1 */

pub const XXH_VERSION_MAJOR: u32 = 0;
pub const XXH_VERSION_MINOR: u32 = 8;
pub const XXH_VERSION_RELEASE: u32 = 0;
pub const XXH_VERSION_NUMBER: u32 = 800;
pub const XXH3_SECRET_SIZE_MIN: usize = 136;
extern "C" {
    pub fn XXH_versionNumber() -> libc::c_uint;
}
pub const XXH_errorcode_XXH_OK: XXH_errorcode = 0;
pub const XXH_errorcode_XXH_ERROR: XXH_errorcode = 1;
pub type XXH_errorcode = libc::c_int;
pub type XXH32_hash_t = u32;
extern "C" {
    pub fn XXH32(
        input: *const ::core::ffi::c_void,
        length: usize,
        seed: XXH32_hash_t,
    ) -> XXH32_hash_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH32_state_s {
   total_len_32: XXH32_hash_t,
   large_len: XXH32_hash_t,
   v1: XXH32_hash_t,
   v2: XXH32_hash_t,
   v3: XXH32_hash_t,
   v4: XXH32_hash_t,
   mem32: [XXH32_hash_t; 4],
   memsize: XXH32_hash_t,
   reserved: XXH32_hash_t,
}
pub type XXH32_state_t = XXH32_state_s;
extern "C" {
    pub fn XXH32_createState() -> *mut XXH32_state_t;
}
extern "C" {
    pub fn XXH32_freeState(statePtr: *mut XXH32_state_t) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH32_copyState(dst_state: *mut XXH32_state_t, src_state: *const XXH32_state_t);
}
extern "C" {
    pub fn XXH32_reset(statePtr: *mut XXH32_state_t, seed: XXH32_hash_t) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH32_update(
        statePtr: *mut XXH32_state_t,
        input: *const ::core::ffi::c_void,
        length: usize,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH32_digest(statePtr: *const XXH32_state_t) -> XXH32_hash_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH32_canonical_t {
    pub digest: [u8; 4usize],
}
#[test]
fn bindgen_test_layout_XXH32_canonical_t() {
    assert_eq!(
        ::core::mem::size_of::<XXH32_canonical_t>(),
        4usize,
        concat!("Size of: ", stringify!(XXH32_canonical_t))
    );
    assert_eq!(
        ::core::mem::align_of::<XXH32_canonical_t>(),
        1usize,
        concat!("Alignment of ", stringify!(XXH32_canonical_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<XXH32_canonical_t>())).digest as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH32_canonical_t),
            "::",
            stringify!(digest)
        )
    );
}
extern "C" {
    pub fn XXH32_canonicalFromHash(dst: *mut XXH32_canonical_t, hash: XXH32_hash_t);
}
extern "C" {
    pub fn XXH32_hashFromCanonical(src: *const XXH32_canonical_t) -> XXH32_hash_t;
}
pub type XXH64_hash_t = u64;
extern "C" {
    pub fn XXH64(
        input: *const ::core::ffi::c_void,
        length: usize,
        seed: XXH64_hash_t,
    ) -> XXH64_hash_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH64_state_s {
   total_len: XXH64_hash_t,
   v1: XXH64_hash_t,
   v2: XXH64_hash_t,
   v3: XXH64_hash_t,
   v4: XXH64_hash_t,
   mem64: [XXH64_hash_t; 4],
   memsize: XXH32_hash_t,
   reserved32: XXH32_hash_t,
   reserved64: XXH64_hash_t,
}
pub type XXH64_state_t = XXH64_state_s;
extern "C" {
    pub fn XXH64_createState() -> *mut XXH64_state_t;
}
extern "C" {
    pub fn XXH64_freeState(statePtr: *mut XXH64_state_t) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH64_copyState(dst_state: *mut XXH64_state_t, src_state: *const XXH64_state_t);
}
extern "C" {
    pub fn XXH64_reset(statePtr: *mut XXH64_state_t, seed: XXH64_hash_t) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH64_update(
        statePtr: *mut XXH64_state_t,
        input: *const ::core::ffi::c_void,
        length: usize,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH64_canonical_t {
    pub digest: [u8; 8usize],
}
#[test]
fn bindgen_test_layout_XXH64_canonical_t() {
    assert_eq!(
        ::core::mem::size_of::<XXH64_canonical_t>(),
        8usize,
        concat!("Size of: ", stringify!(XXH64_canonical_t))
    );
    assert_eq!(
        ::core::mem::align_of::<XXH64_canonical_t>(),
        1usize,
        concat!("Alignment of ", stringify!(XXH64_canonical_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<XXH64_canonical_t>())).digest as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH64_canonical_t),
            "::",
            stringify!(digest)
        )
    );
}
extern "C" {
    pub fn XXH64_canonicalFromHash(dst: *mut XXH64_canonical_t, hash: XXH64_hash_t);
}
extern "C" {
    pub fn XXH64_hashFromCanonical(src: *const XXH64_canonical_t) -> XXH64_hash_t;
}
extern "C" {
    pub fn XXH3_64bits(data: *const ::core::ffi::c_void, len: usize) -> XXH64_hash_t;
}
extern "C" {
    pub fn XXH3_64bits_withSeed(
        data: *const ::core::ffi::c_void,
        len: usize,
        seed: XXH64_hash_t,
    ) -> XXH64_hash_t;
}
extern "C" {
    pub fn XXH3_64bits_withSecret(
        data: *const ::core::ffi::c_void,
        len: usize,
        secret: *const ::core::ffi::c_void,
        secretSize: usize,
    ) -> XXH64_hash_t;
}
pub const XXH3_INTERNALBUFFER_SIZE: usize = 256;
pub const XXH3_SECRET_DEFAULT_SIZE: usize = 192;
#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Copy, Clone)]
struct Acc([XXH64_hash_t; 8]);
#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Copy, Clone)]
struct CustomSecret([u64; XXH3_SECRET_DEFAULT_SIZE / ::core::mem::size_of::<u64>()]);
#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Copy, Clone)]
struct Buffer([u64; XXH3_INTERNALBUFFER_SIZE / ::core::mem::size_of::<u64>()]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH3_state_s {
    acc: Acc,
    customSecret: CustomSecret,
    buffer: Buffer,
    bufferedSize: XXH32_hash_t,
    reserved32: XXH32_hash_t,
    nbStripesSoFar: usize,
    totalLen: XXH64_hash_t,
    nbStripesPerBlock: usize,
    secretLimit: usize,
    seed: XXH64_hash_t,
    reserved64: XXH64_hash_t,
    extSecret: *const u8,
}
pub type XXH3_state_t = XXH3_state_s;
extern "C" {
    pub fn XXH3_createState() -> *mut XXH3_state_t;
}
extern "C" {
    pub fn XXH3_freeState(statePtr: *mut XXH3_state_t) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_copyState(dst_state: *mut XXH3_state_t, src_state: *const XXH3_state_t);
}
extern "C" {
    pub fn XXH3_64bits_reset(statePtr: *mut XXH3_state_t) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_64bits_reset_withSeed(
        statePtr: *mut XXH3_state_t,
        seed: XXH64_hash_t,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_64bits_reset_withSecret(
        statePtr: *mut XXH3_state_t,
        secret: *const ::core::ffi::c_void,
        secretSize: usize,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_64bits_update(
        statePtr: *mut XXH3_state_t,
        input: *const ::core::ffi::c_void,
        length: usize,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_64bits_digest(statePtr: *const XXH3_state_t) -> XXH64_hash_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH128_hash_t {
    pub low64: XXH64_hash_t,
    pub high64: XXH64_hash_t,
}
#[test]
fn bindgen_test_layout_XXH128_hash_t() {
    assert_eq!(
        ::core::mem::size_of::<XXH128_hash_t>(),
        16usize,
        concat!("Size of: ", stringify!(XXH128_hash_t))
    );
    assert_eq!(
        ::core::mem::align_of::<XXH128_hash_t>(),
        8usize,
        concat!("Alignment of ", stringify!(XXH128_hash_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<XXH128_hash_t>())).low64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH128_hash_t),
            "::",
            stringify!(low64)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<XXH128_hash_t>())).high64 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH128_hash_t),
            "::",
            stringify!(high64)
        )
    );
}
extern "C" {
    pub fn XXH3_128bits(data: *const ::core::ffi::c_void, len: usize) -> XXH128_hash_t;
}
extern "C" {
    pub fn XXH3_128bits_withSeed(
        data: *const ::core::ffi::c_void,
        len: usize,
        seed: XXH64_hash_t,
    ) -> XXH128_hash_t;
}
extern "C" {
    pub fn XXH3_128bits_withSecret(
        data: *const ::core::ffi::c_void,
        len: usize,
        secret: *const ::core::ffi::c_void,
        secretSize: usize,
    ) -> XXH128_hash_t;
}
extern "C" {
    pub fn XXH3_128bits_reset(statePtr: *mut XXH3_state_t) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_128bits_reset_withSeed(
        statePtr: *mut XXH3_state_t,
        seed: XXH64_hash_t,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_128bits_reset_withSecret(
        statePtr: *mut XXH3_state_t,
        secret: *const ::core::ffi::c_void,
        secretSize: usize,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_128bits_update(
        statePtr: *mut XXH3_state_t,
        input: *const ::core::ffi::c_void,
        length: usize,
    ) -> XXH_errorcode;
}
extern "C" {
    pub fn XXH3_128bits_digest(statePtr: *const XXH3_state_t) -> XXH128_hash_t;
}
extern "C" {
    pub fn XXH128_isEqual(h1: XXH128_hash_t, h2: XXH128_hash_t) -> libc::c_int;
}
extern "C" {
    pub fn XXH128_cmp(
        h128_1: *const ::core::ffi::c_void,
        h128_2: *const ::core::ffi::c_void,
    ) -> libc::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XXH128_canonical_t {
    pub digest: [u8; 16usize],
}
#[test]
fn bindgen_test_layout_XXH128_canonical_t() {
    assert_eq!(
        ::core::mem::size_of::<XXH128_canonical_t>(),
        16usize,
        concat!("Size of: ", stringify!(XXH128_canonical_t))
    );
    assert_eq!(
        ::core::mem::align_of::<XXH128_canonical_t>(),
        1usize,
        concat!("Alignment of ", stringify!(XXH128_canonical_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<XXH128_canonical_t>())).digest as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XXH128_canonical_t),
            "::",
            stringify!(digest)
        )
    );
}
extern "C" {
    pub fn XXH128_canonicalFromHash(dst: *mut XXH128_canonical_t, hash: XXH128_hash_t);
}
extern "C" {
    pub fn XXH128_hashFromCanonical(src: *const XXH128_canonical_t) -> XXH128_hash_t;
}