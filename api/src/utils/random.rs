use chrono::Utc;

pub fn gen_range(from: i16, to: i16) -> u64 {
    let range = (to - from + 1) as u64;
    let seed = Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;

    seed % range
}
