mod format;
mod permissions;

use clap::Parser;
use std::fs;
use std::io::Result;

#[derive(Debug, Parser)]
struct Args {
    #[clap(default_value = ".")]
    dir: String,

    #[clap(short, long, default_value = "false")]
    a: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let entries = fs::read_dir(args.dir).expect("Failed to read directory");

    println!("{}", format::get_header());
    println!("-----------------------------------------------------------------------------------");

    let mut count = 0;
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if !args.a && name.starts_with('.') {
                    continue; // Skip hidden files when the -a flag is not provided
                }
                let metadata = entry.metadata().expect("Failed to get metadata");
                let file_type = if metadata.is_dir() { "dir" } else { "file" };
                let size = format::format_size(metadata.len());
                let modified = format::format_modified_time(&metadata);
                let mut perms = match permissions::get_file_permissions(&entry) {
                    Ok(p) => p,
                    Err(_) => String::from("N/A"), // Handle the error case here
                };
                perms.push(' ');
                perms.push(' ');
                let is_exec = perms.contains("x");
                let is_dir = file_type == "dir";
                let formatted_name = &format::format_name(name, is_dir, is_exec);
                count += 1;
                println!(
                    "| {:2} | {} | {:<4} | {:>8} | {:<15} | {} |",
                    count, formatted_name, file_type, size, modified, perms
                );
            }
        }
    }

    return Ok(());
}
