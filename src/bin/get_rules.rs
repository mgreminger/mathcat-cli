use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("Fetching crates via cargo vendor...");
    let status = Command::new("cargo")
        .arg("vendor")
        .status()
        .expect("Failed to execute cargo vendor");

    if !status.success() {
        eprintln!("cargo vendor failed.");
        std::process::exit(1);
    }

    // 1. Locate the vendored Rules directory
    let vendor_dir = Path::new("vendor");
    let mut rules_src = None;

    if let Ok(entries) = fs::read_dir(vendor_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // Look for a folder like 'mathcat-0.7.5'
                if name.starts_with("mathcat") && path.is_dir() {
                    let candidate = path.join("Rules");
                    if candidate.exists() {
                        rules_src = Some(candidate);
                        break;
                    }
                }
            }
        }
    }

    let src_path = match rules_src {
        Some(path) => path,
        None => {
            eprintln!("Could not find mathcat*/Rules in vendor/. Did cargo vendor succeed?");
            std::process::exit(1);
        }
    };

    let dest_path = Path::new("Rules");

    // 2. Recursively copy the directory
    println!("Copying Rules from {:?} to {:?}", src_path, dest_path);
    if let Err(e) = copy_dir_all(&src_path, &dest_path) {
        eprintln!("Failed to copy Rules directory: {}", e);
        std::process::exit(1);
    }

    println!("Successfully extracted Rules!");
}

// Helper to recursively copy directories since std::fs doesn't have it built-in
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}