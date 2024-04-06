use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let mut total_files = 0;
    let mut total_sizes = 0;

    let path = std::env::args().nth(1).expect("Please provide a directory path");
    let path = PathBuf::from(path);

    for entry in WalkDir::new(path) {
        let entry = entry.expect("Error accessing directory entry");
        if entry.file_type().is_file() {
            // if is_image_file(&entry.path()) {
                total_files += 1;
                total_sizes += entry.metadata().expect("Error getting file metadata").len();
            // }
        }
    }

    println!("Total files: {}", total_files);
    println!("Total size of image files: {} bytes", total_sizes);
}

fn is_image_file(path: &Path) -> bool {
    let ext = path.extension().and_then(|os_str| os_str.to_str()).unwrap_or_default();
    ext.eq_ignore_ascii_case("jpg") || ext.eq_ignore_ascii_case("jpeg") || ext.eq_ignore_ascii_case("png") || ext.eq_ignore_ascii_case("bmp") || ext.eq_ignore_ascii_case("gif")
}
