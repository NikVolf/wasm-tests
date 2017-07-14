#![feature(link_args)]
#![no_main]
#![allow(deprecated)] 

mod helpers;

use std::hash::{Hasher, SipHasher};
use helpers::{CallArgs, ext, write_u32, logger};

#[no_mangle]
pub fn call(desc: *mut u8) {   
    let mut ctx = unsafe { CallArgs::from_raw(desc) };

    let addr = [13u8, 19, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let input = [1u8, 2, 3, 5, 7, 11];
    let mut result = vec![0u8; 256];
    match ext::call_code(&addr, &input, &mut result[..]) {
        Ok(_) => {
            logger::debug("Call succeed");
        },
        Err(_) => {
            logger::debug("Call failed");
        }
    }

    let mut hasher = SipHasher::new_with_keys(0, 0);
    hasher.write(&result[..]);

    *ctx.result_mut() = Vec::with_capacity(4);
    write_u32(&mut ctx.result_mut()[..], hasher.finish() as u32);

    unsafe { ctx.save(desc); }
}