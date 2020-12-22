use core::time::Duration;
use std::vec::Vec;
use std::collections::HashMap;

pub struct PageInfo {
    headers: HashMap<String, String>,
    content: Vec<u8>,
    download_time: Duration,
}