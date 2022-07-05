use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

#[allow(dead_code)]
pub fn current_ts() -> Result<u64, SystemTimeError> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
    Ok(since_the_epoch.as_secs())
}
