use error::Error;
use serde::Serialize;
use serde_json::json;
use std::mem::forget;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::Builder;
use std::{
    io::{self, Cursor, Read, Write},
    slice,
    sync::atomic::{AtomicU64, AtomicUsize},
};

use crate::request::Request;

mod builder;
mod error;
mod method;
mod mock;
mod query_namespace;
mod request;
mod response;
mod template_context;
mod r#type;
mod utils;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(size);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}

#[no_mangle]
pub extern "C" fn build(ptr: *mut u8, size: usize) -> *const u8 {
    match try_build(ptr, size) {
        Ok(value) => write_response(value),
        Err(err) => write_response(json!({"error": format!("{err}")})),
    }
}

#[no_mangle]
pub extern "C" fn try_build(ptr: *mut u8, size: usize) -> Result<impl Serialize, Error> {
    let request = load_request(ptr, size)?;

    let builder = builder::Builder::new(request.clone());
    builder.build()
}

static RESPONSE_LENGTH: AtomicU64 = AtomicU64::new(0);

fn write_response<T: Serialize>(response: T) -> *const u8 {
    let buffer = serde_json::to_string(&response).unwrap();

    RESPONSE_LENGTH.store(buffer.len() as _, Relaxed);
    buffer.leak().as_bytes().as_ptr()
}

#[no_mangle]
pub extern "C" fn response_length() -> u64 {
    RESPONSE_LENGTH.load(Relaxed)
}

fn load_request(ptr: *mut u8, size: usize) -> Result<Request, error::Error> {
    let buffer = unsafe { slice::from_raw_parts(ptr, size) };
    let payload = String::from_utf8(buffer.into()).unwrap();
    let request: Request = serde_json::from_str(&payload)?;
    return Ok(request);
}

fn main() {}
