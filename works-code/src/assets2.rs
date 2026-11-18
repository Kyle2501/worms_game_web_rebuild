use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use rayon::prelude::*;
use ignore::WalkBuilder;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use anyhow::{Context, Result};

/// Scans an input directory recursively, respects `.worksignore` rules, 
/// and compresses all matching assets concurrently using Rayon.
pub fn compress_assets_parallel(input_dir: &Path, output_dir: &Path) -> Result<usize> {
    // 1. Ensure the output directory exists
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {:?}", output_dir))?;

    // 2. Configure WalkBuilder to scan the directory and parse `.worksignore` files
    let walker = WalkBuilder::new(input_dir)
        .add_custom_ignore_filename(".worksignore")
        .hidden(false) // Set to true if you want to skip dotfiles (e.g. .DS_Store) automatically
        .git_ignore(false) // Disable standard .gitignore if you only want strict .worksignore control
        .build();

    // 3. Collect all non-ignored files synchronously into a vector
    let files: Vec<PathBuf> = walker
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        // Exclude the ignore file itself from being bundled
        .filter(|entry| entry.file_name() != ".worksignore")
        .map(|entry| entry.path().to_owned())
        .collect();

    println!("Found {} assets in {:?} (respecting .worksignore). Compressing concurrently...", files.len(), input_dir);

    // 4. Process files in parallel using Rayon
    let results: Result<Vec<()>, io::Error> = files
        .par_iter()
        .map(|src_path| {
            let relative_path = src_path
                .strip_prefix(input_dir)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            
            let mut dest_path = output_dir.join(relative_path);
            
            // Append .worksz extension
            if let Some(ext) = dest_path.extension() {
                let mut new_ext = ext.to_os_string();
                new_ext.push(".worksz");
                dest_path.set_extension(new_ext);
            } else {
                dest_path.set_extension("worksz");
            }

            // Ensure parent directory exists for nested assets
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Read source file bytes
            let mut input_file = File::open(src_path)?;
            let mut buffer = Vec::new();
            input_file.read_to_end(&mut buffer)?;

            // Compress payload using Zlib
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&buffer)?;
            let compressed_data = encoder.finish()?;

            // Write compressed output
            let mut output_file = File::create(dest_path)?;
            output_file.write_all(&compressed_data)?;

            Ok(())
        })
        .collect();

    let processed = results.map(|v| v.len())?;
    Ok(processed)
}
