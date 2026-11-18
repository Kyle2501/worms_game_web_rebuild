use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::prelude::*;
use ignore::WalkBuilder;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::{Context, Result};

/// Scans an input directory, respects `.worksignore`, compresses assets concurrently using Rayon,
/// and displays a real-time progress bar via indicatif.
pub fn compress_assets_parallel(input_dir: &Path, output_dir: &Path) -> Result<usize> {
    // 1. Ensure the output directory exists
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {:?}", output_dir))?;

    // 2. Configure WalkBuilder to scan and parse `.worksignore` files
    let walker = WalkBuilder::new(input_dir)
        .add_custom_ignore_filename(".worksignore")
        .hidden(false)
        .git_ignore(false)
        .build();

    // 3. Collect non-ignored files into a vector
    let files: Vec<PathBuf> = walker
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .filter(|entry| entry.file_name() != ".worksignore")
        .map(|entry| entry.path().to_owned())
        .collect();

    let total_files = files.len();
    if total_files == 0 {
        println!("No assets found to compress in {:?}.", input_dir);
        return Ok(0);
    }

    // 4. Set up the indicatif progress bar style
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta}) {msg}")
            .expect("Valid progress bar template")
            .progress_chars("#>-"),
    );
    pb.set_message("Compressing assets...");

    // Atomic counter to track completed tasks safely across Rayon threads
    let completed = AtomicUsize::new(0);

    // 5. Process files in parallel using Rayon
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

            // Increment atomic counter and update progress bar
            let current = completed.fetch_add(1, Ordering::Relaxed) + 1;
            pb.set_position(current as u64);

            Ok(())
        })
        .collect();

    pb.finish_with_message("Asset compression complete!");

    let processed = results.map(|v| v.len())?;
    Ok(processed)
}
