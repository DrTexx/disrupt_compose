// use colored::*;
// use std::env;
// use std::path::Path;
// use std::fs::File;
// use std::process::Command;

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

// fn test() {
//     let mut file = File::open("project.json").unwrap();
//     let json: serde_json::Value =
//         serde_json::from_reader(file).expect("JSON was not well-formatted");

//     println!("{:?}", json);
// println!("{}", DISRUPT_COMPOSE_FCB_DECOMPILATION_BINARY);
// let example = ModFile {
//     filepath: String::from("./watersplines.fcb"),
//     compile_to_fcb: false,
//     decompile_to_xml: true,
// };
// println!("{:?}", example);

// if example.decompile_to_xml {
//     let output = Command::new(DISRUPT_COMPOSE_FCB_DECOMPILATION_BINARY)
//         .arg("--xml")
//         .arg(example.filepath)
//         .output()
//         .expect("failed to execute process");

//     println!("status: {}", output.status);
//     println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
//     println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

//     assert!(output.status.success());
// }
// }
