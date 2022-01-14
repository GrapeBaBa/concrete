use crate::cmd;
use std::io::Error;

pub fn test_book_boolean() -> Result<(), Error> {
    cmd!("cargo build -p concrete-boolean --release")?;
    cmd!("mdbook test concrete-boolean/book -L target/release/deps")
}
pub fn test_book_core() -> Result<(), Error> {
    cmd!("cargo build -p concrete-core --release")?;
    cmd!("mdbook test concrete-core/book -L target/release/deps")
}
