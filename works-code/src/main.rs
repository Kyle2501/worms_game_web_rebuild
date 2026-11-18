mod assets;
use std::path::Path;
use anyhow::Result;

fn main() -> Result<()> {
    let input = Path::new("./assets");
    let output = Path::new("./dist/compressed_assets");

    let start = std::time::Instant::now();
    let count = assets::compress_assets_parallel(input, output)?;
    let duration = start.elapsed();

    println!("Successfully processed {} assets in {:?}.", count, duration);
    Ok(())
}
