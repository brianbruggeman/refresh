use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Repo {
    pub name: String,
    pub ssh_url: String,
}
