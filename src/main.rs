use components::storage::AppStorage;

mod components;

fn main() -> std::io::Result<()> {
    let storage_dirs = AppStorage::new()?;
    storage_dirs.destroy()?;
    Ok(())
}

#[cfg(test)]
mod tests;