use chrono::Utc;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn generate_id(len: i8) -> String {
    let timestamp = Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    let combined = timestamp.wrapping_add(counter);

    const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut result = String::with_capacity(len as usize);
    let mut num = combined;

    for _ in 0..len {
        let idx = (num % CHARSET.len() as u64) as usize;
        result.push(CHARSET[idx] as char);
        num /= CHARSET.len() as u64;
    }

    result
}

pub fn gen_range(from: i16, to: i16) -> u64 {
    let range = (to - from + 1) as u64;
    let seed = Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;

    seed % range
}
