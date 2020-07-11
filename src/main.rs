// use colored::*;

fn install_patch(modfiles: Vec<&str>) {
    println!("installing patch...");
    for modfile in modfiles {
        println!("{}",modfile)
    }
}

fn main() {
    println!("{}", "Hello, world!");
    let modfiles = vec!["file1", "file2", "file3"];
    install_patch(modfiles)
}
