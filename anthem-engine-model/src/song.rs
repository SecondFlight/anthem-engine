use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{pattern::Pattern, util::id::get_id};

#[derive(Serialize, Deserialize)]
pub struct Song {
    id: u64,
    patterns: HashMap<u64, Pattern>,
}

impl Default for Song {
    fn default() -> Self {
        Self {
            id: get_id(),
            patterns: HashMap::new(),
        }
    }
}
