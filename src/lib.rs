mod utils;

use std::sync::Mutex;
use std::{collections::HashMap, convert::TryFrom};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use ssimulacra2::{compute_frame_ssimulacra2, ColorPrimaries, TransferCharacteristic, Xyb};
use yuvxyb::Rgb;

mod ssimulacra2;

#[macro_use]
extern crate lazy_static;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = window)]
    pub fn __random() -> u8;
}

pub struct BufferStorage {
    pub buffer_map: HashMap<String, Vec<u8>>,
}

impl BufferStorage {
    fn new() -> Self {
        BufferStorage {
            buffer_map: HashMap::new(),
        }
    }
}

macro_rules! log {
  ($($t:tt)*) => (crate::log(&("[C]".to_string() + &format_args!($($t)*).to_string())))
}

lazy_static! {
    pub static ref GLOBAL_BUFFER_STORAGE: Mutex<BufferStorage> = {
        let buffer_storage = BufferStorage::new();
        Mutex::new(buffer_storage)
    };
}

#[wasm_bindgen]
pub fn set_wasm_panic_hook() {
    // can be continued
    set_panic_hook();
}

#[wasm_bindgen]
pub fn new_buffer(key: String, len: usize) -> *const u8 {
    log!("new_buffer, key: {:?}, len: {:?}", key, len);
    let mut global_buffer_storage = GLOBAL_BUFFER_STORAGE.lock().unwrap();
    let buffer = vec![255 as u8; len];
    let ptr = buffer.as_ptr();
    global_buffer_storage.buffer_map.insert(key, buffer);
    ptr
}

#[wasm_bindgen]
pub fn get_buffer(key: String) -> *const u8 {
    let global_buffer_storage = GLOBAL_BUFFER_STORAGE.lock().unwrap();
    if let Some(buffer) = global_buffer_storage.buffer_map.get(&key) {
        return buffer.as_ptr();
    } else {
        return Vec::new().as_ptr();
    }
}

#[wasm_bindgen]
pub fn print_buffer(key: String) {
    log!("call print buffer");
    let global_buffer_storage = GLOBAL_BUFFER_STORAGE.lock().unwrap();
    if let Some(buffer) = global_buffer_storage.buffer_map.get(&key) {
        log!("[render-wasm]print buffer: {:?}", buffer);
    }
}

#[wasm_bindgen]
pub fn remove_buffer(key: String) {
    let mut global_buffer_storage = GLOBAL_BUFFER_STORAGE.lock().unwrap();
    if let Some(_buffer) = global_buffer_storage.buffer_map.remove(&key) {
        log!("remove buffer success");
    } else {
        log!("remove buffer error");
    }
}

#[wasm_bindgen]
pub fn calc_s2(img1_key: String, img2_key: String, width: usize, height: usize) -> f64 {
    let global_buffer_storage = GLOBAL_BUFFER_STORAGE.lock().unwrap();
    let _buffer1 = global_buffer_storage
        .buffer_map
        .get(&img1_key)
        .expect("NoSuchKey");
    let chunked_vec1 = _buffer1
        .chunks(3)
        .map(|chunk| [chunk[0] as f32, chunk[1] as f32, chunk[2] as f32])
        .collect::<Vec<_>>();

    let source_data = Xyb::try_from(
        Rgb::new(
            chunked_vec1,
            width,
            height,
            TransferCharacteristic::SRGB,
            ColorPrimaries::BT709,
        )
        .expect("Failed to process source_data into RGB"),
    )
    .expect("Failed to process source_data into XYB");

    let _buffer2 = global_buffer_storage
        .buffer_map
        .get(&img2_key)
        .expect("NoSuchKey");
    let chunked_vec2 = _buffer2
        .chunks(3)
        .map(|chunk| [chunk[0] as f32, chunk[1] as f32, chunk[2] as f32])
        .collect::<Vec<_>>();
    let distorted_data = Xyb::try_from(
        Rgb::new(
            chunked_vec2,
            width,
            height,
            TransferCharacteristic::SRGB,
            ColorPrimaries::BT709,
        )
        .expect("Failed to process source_data into RGB"),
    )
    .expect("Failed to process source_data into XYB");

    let res = compute_frame_ssimulacra2(source_data, distorted_data)
        .expect("Failed to calculate ssimulacra2");
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_test() {
        let chunked_vec = vec![255 as u8; 20 * 20 * 3]
            .chunks(3)
            .map(|chunk| [chunk[0] as f32, chunk[1] as f32, chunk[2] as f32])
            .collect::<Vec<_>>();

        let source_data = Xyb::try_from(
            Rgb::new(
                chunked_vec,
                20 as usize,
                20 as usize,
                TransferCharacteristic::SRGB,
                ColorPrimaries::BT709,
            )
            .expect("Failed to process source_data into RGB"),
        )
        .expect("Failed to process source_data into XYB");

        let chunked_vec2 = vec![254 as u8; 20 * 20 * 3]
            .chunks(3)
            .map(|chunk| [chunk[0] as f32, chunk[1] as f32, chunk[2] as f32])
            .collect::<Vec<_>>();

        let distorted_data = Xyb::try_from(
            Rgb::new(
                chunked_vec2,
                20 as usize,
                20 as usize,
                TransferCharacteristic::SRGB,
                ColorPrimaries::BT709,
            )
            .expect("Failed to process source_data into RGB"),
        )
        .expect("Failed to process source_data into XYB");

        let res = compute_frame_ssimulacra2(source_data, distorted_data)
            .expect("Failed to calculate ssimulacra2");

        println!("{:?}", res);

        assert_eq!(100.0, 100.0);
    }
}
