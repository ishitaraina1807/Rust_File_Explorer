use std::fs;
// use std::io;

fn main() {
    // let mut current_dir = String::new();
    // std::io::stdin()
    // .read_line(&mut current_dir)
    // .expect("Failed to read line");
    // let mut current_dir = String::new();
    // io::stdin()
    //     .read_line(&mut current_dir)
    //     .expect("Failed to read input");
    let current_dir = "./target";

    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        println!("{}", file_name_str);
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read directory");
    }
}
