use crate::ir::{Ir, IrService};
use crate::presentation::PresentationService;
use crate::request::Request;
use crate::response::Response;
use error::Error;
use serde::Serialize;
use serde_json::json;
use std::sync::atomic::Ordering::Relaxed;
use std::{slice, sync::atomic::AtomicU64};

pub mod error;
pub mod ir;
pub mod mock;
pub mod presentation;
pub mod request;
pub mod response;

mod utils;

#[unsafe(no_mangle)]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(size);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn build(ptr: *mut u8, size: usize) -> *const u8 {
    match try_build(ptr, size) {
        Ok(value) => write_response(value),
        Err(err) => write_response(json!({"error": format!("{err}")})),
    }
}

fn try_build(ptr: *mut u8, size: usize) -> Result<Response, Error> {
    let request = load_request(ptr, size)?;

    let ir = IrService::new(request.clone())?.build(request);

    let presentation_service = PresentationService { ir };

    Ok(Response {
        files: presentation_service.generate()?,
    })
}

static RESPONSE_LENGTH: AtomicU64 = AtomicU64::new(0);

fn write_response<T: Serialize>(response: T) -> *const u8 {
    let buffer = serde_json::to_string(&response).unwrap();

    RESPONSE_LENGTH.store(buffer.len() as _, Relaxed);
    buffer.leak().as_bytes().as_ptr()
}

#[unsafe(no_mangle)]
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

#[test]
fn test_from_catalog() {
    let contents = include_str!("../tests/request.json");

    println!(
        "{:?}",
        try_build(contents.as_ptr() as _, contents.len())
            .unwrap()
            .files
    );
}
