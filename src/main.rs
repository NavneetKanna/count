use std::fs::read_dir;
use clap::Parser;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = ".")]
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let mut total_count: u32 = 0;
    let mut dir_count: u32 = 0;
    let mut file_count: u32 = 0;
    let mut hidden_count: u32 = 0;

    for entry in read_dir(args.path)? {
        let entry = entry?;
        let file = entry.file_type()?;
        let file_name = entry.file_name();
        let os_str = OsStr::as_bytes(&file_name);

        if os_str[0] == 46 {
            hidden_count += 1;
        }
        else {
            if file.is_dir() {
                dir_count += 1;
            }
            if file.is_file() {
                file_count += 1;
            }
        }

        total_count += 1;
    }

    println!("Total Number of Files: {total_count}");
    println!("Number of Hidden Files and Folders: {hidden_count}");
    println!("Number of directories: {dir_count}");
    println!("Number of Files: {file_count}");

    Ok(())
}

