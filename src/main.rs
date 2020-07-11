// use colored::*;

#[derive(Debug)]
struct ModFile {
    filepath: String,
    compile_to_fcb: bool,
    decompile_to_xml: bool,
}

fn install_patch(modfiles: Vec<ModFile>) {
    println!("installing patch...");
    for modfile in modfiles {
        println!("{:?}", modfile)
    }
}

fn main() {
    println!("{}", "Hello, world!");
    let modfiles = vec![
        ModFile {
            filepath: String::from("./file1"),
            compile_to_fcb: false,
            decompile_to_xml: true,
        },
        ModFile {
            filepath: String::from("./file2"),
            compile_to_fcb: true,
            decompile_to_xml: false,
        },
    ];
    install_patch(modfiles);
}
