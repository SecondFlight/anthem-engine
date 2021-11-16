use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::pattern::Pattern;

#[derive(Serialize, Deserialize)]
pub struct Song {
    id: u64,
    patterns: HashMap<u64, Pattern>,
}
