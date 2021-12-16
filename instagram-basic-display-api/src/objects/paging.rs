use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paging {
    pub cursors: PagingCursors,
    pub previous: Option<String>,
    pub next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagingCursors {
    pub before: String,
    pub after: String,
}

impl Paging {
    pub fn next_cursor(&self) -> Option<String> {
        if self.next.is_some() {
            Some(self.cursors.after.clone())
        } else {
            None
        }
    }
}
