use std::fs::read_dir;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = ".")]
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    // println!("path: {:?}", args.path);

    let mut total_count: u32 = 0;
    let mut dir_count: u32 = 0;
    let mut file_count: u32 = 0;

    for entry in read_dir(args.path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            dir_count += 1;
        }
        if path.is_file() {
            file_count += 1;
        }
        total_count += 1;
    }

    println!("Total Number of Files: {total_count}");
    println!("Number of directories: {dir_count}");
    println!("Number of Files: {file_count}");

    Ok(())
}

