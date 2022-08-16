use std::io::Error;

use crate::components::storage::AppStorage;

mod components;

fn main() -> Result<(), Error> {
    let storage_dirs = AppStorage::build()?;
    Ok(())
}

#[cfg(test)]
mod tests;
