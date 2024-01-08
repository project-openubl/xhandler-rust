use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IndexState {
    A,
    B,
}

impl IndexState {
    pub fn directory(&self, root: &Path) -> PathBuf {
        match self {
            Self::A => root.join("a"),
            Self::B => root.join("b"),
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}
