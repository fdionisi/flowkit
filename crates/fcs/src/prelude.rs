use std::fs::File;

use crate::traits::ByteRead;
pub use crate::traits::{FcsRead, FcsWrite};

impl ByteRead for File {}

impl FcsRead for File {}

impl FcsWrite for File {}
