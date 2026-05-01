use anyhow::Result;
use rayon::prelude::*;
use std::fs;
// use std::path::Path;

use crate::filters::Filter;

pub fn process_single<F: Filter>(input: &str, output: &str, filter: F) -> Result<()> {
    let img = image::open(input)?;
    let processed = filter.apply(img);
    processed.save(output)?;
    println!("Saved: {}", output);
    Ok(())
}

pub fn process_batch<F>(folder: String, filter: F) -> Result<()>
where
    // F can be any type that implements Filter trait
    // Sync is required because the filter will be shared across multiple threads
    F: Filter + Sync,
{

    // Read all entries inside the folder
    // returns an iterator of Result<DirEntry, Error>
    let entries: Vec<_> = fs::read_dir(folder)?
        .filter_map(Result::ok)
        // Collect all directory entries into a vector
        .collect();

    // Process entries in parallel using Rayon
    entries.par_iter().for_each(|entry| {
        // Convert DirEntry into PathBuf
        let path = entry.path();

        // Process only files, ignore directories
        if path.is_file() {
            // Try opening image file
            if let Ok(img) = image::open(&path) {

                let processed = filter.apply(img);

                // Create output file name
                let output = format!(
                    "processed_{}",
                    path.file_name().unwrap().to_string_lossy()
                );

                // Save processed image
                // '_' ignores save errors silently
                let _ = processed.save(output);
            }
        }
    });

    println!("Batch processing completed.");
    // Return success
    Ok(())
}





