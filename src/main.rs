use std::fs::read_dir;

fn main() -> std::io::Result<()> {
    let mut count: u32 = 0;

    for _ in read_dir(".")? {
        count += 1;
    }

    println!("Count: {count}");
    Ok(())
}

// use std::fs;

// fn main() -> std::io::Result<()> {
//     let mut count: u32 = 0;
//     for entry in fs::read_dir(".")? {
//         let dir = entry?;
//         count += 1;
//         println!("{:?}", dir.path());
//     }
//     println!("Count: {count}");
//     Ok(())
// }
