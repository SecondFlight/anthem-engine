use serde::{Deserialize, Serialize};

use crate::{song::Song, util::id::get_id};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: u64,
    pub song: Song,
}

impl Default for Project {
    fn default() -> Self {
        Self { id: get_id(), song: Song::default() }
    }
}
