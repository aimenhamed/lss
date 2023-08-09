use std::fs::Metadata;

pub fn format_size(size: u64) -> String {
    const SIZES: [&str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    if size == 0 {
        return "0B".to_string();
    }
    let i = (size as f64).log(1024.0).floor() as usize;
    let s: f64 = 1024.0;
    let size_str = (size as f64 / s.powi(i as i32)).round().to_string();
    let mut size_formatted = size_str.trim_end_matches("0").trim_end_matches(".");
    if size_formatted == "0" {
        size_formatted = "0";
    }
    format!("{}{}", size_formatted, SIZES[i])
}

pub fn format_modified_time(metadata: &Metadata) -> String {
    let modified_time = metadata.modified().unwrap().elapsed().unwrap().as_secs();

    if modified_time < 60 {
        format!("{} seconds ago", modified_time)
    } else if modified_time < 3600 {
        format!("{} minutes ago", modified_time / 60)
    } else if modified_time < 86_400 {
        format!("{} hours ago", modified_time / 3600)
    } else if modified_time < 2_592_000 {
        format!("{} days ago", modified_time / 86_400)
    } else if modified_time < 31_536_000 {
        format!("{} months ago", modified_time / 2_592_000)
    } else {
        format!("{} years ago", modified_time / 31_536_000)
    }
}

pub fn format_name(name: &str, is_directory: bool, is_exec: bool) -> String {
    if is_directory {
        let bold = format!("\x1B[1;32m{}\x1B[0m", name);
        return format!("{:<41}", bold);
    } else {
        if is_exec {
            let italics = format!("\x1B[3m{}\x1B[0m", name);
            return format!("{:<38}", italics);
        }
        return format!("{:<30}", name);
    }
}

pub fn get_header(small: bool) -> String {
    if small {
        return format!("| {:2} | {:<30} |", "#", "name");
    }

    return format!(
        "| {:2} | {:<30} | {:<4} | {:>8} | {:<15} | {} |",
        "#", "name", "type", "size", "modified", "perms"
    );
}

pub fn get_delimiter(small: bool) -> String {
    if small {
        return String::from("---------------------------------------");
    }

    return String::from(
        "-----------------------------------------------------------------------------------",
    );
}
