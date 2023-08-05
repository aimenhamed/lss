use std::fs;
use std::fs::DirEntry;
use std::io;
use std::os::unix::fs::PermissionsExt;

pub fn get_file_permissions(entry: &DirEntry) -> io::Result<String> {
    let path = entry.path();
    let metadata = fs::metadata(path)?;
    let permissions = metadata.permissions();
    let mode = permissions.mode();

    // Check various permission properties
    let readable = mode & 0o444 != 0;
    let writable = mode & 0o222 != 0;
    let executable = mode & 0o111 != 0;

    let mut perms = String::new();
    if readable {
        perms.push('r');
    } else {
        perms.push('-');
    }
    if writable {
        perms.push('w');
    } else {
        perms.push('-');
    }
    if executable {
        perms.push('x');
    } else {
        perms.push('-');
    }

    return Ok(perms);
}
