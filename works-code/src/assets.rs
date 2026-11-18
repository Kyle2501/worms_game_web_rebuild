use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use rayon::prelude::*;
use walkdir::WalkDir;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use anyhow::{Context, Result};

/// Scans an input directory recursively and compresses all assets concurrently 
/// into the output directory using Rayon.
pub fn compress_assets_parallel(input_dir: &Path, output_dir: &Path) -> Result<usize> {
    // 1. Ensure the output directory exists
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {:?}", output_dir))?;

    // 2. Collect all files synchronously via walkdir (filtering out directories)
    let files: Vec<PathBuf> = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_owned())
        .collect();

    println!("Found {} assets in {:?}. Compressing concurrently...", files.len(), input_dir);

    // 3. Process files in parallel using Rayon's parallel iterator
    let results: Result<Vec<()>, io::Error> = files
        .par_iter()
        .map(|src_path| {
            // Compute the relative path to preserve directory structure in output
            let relative_path = src_path
                .strip_prefix(input_dir)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            
            let mut dest_path = output_dir.join(relative_path);
            
            // Append a compressed extension (e.g., .worksz)
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

            // Compress payload using Zlib (Default compression level)
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&buffer)?;
            let compressed_data = encoder.finish()?;

            // Write compressed output
            let mut output_file = File::create(dest_path)?;
            output_file.write_all(&compressed_data)?;

            Ok(())
        })
        .collect();

    // 4. Handle results and return processed file count
    let processed = results.map(|v| v.len())?;
    Ok(processed)
}
