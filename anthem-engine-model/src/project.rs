use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    id: u64,
}
