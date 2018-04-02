use std::env;
use std::path::Path;
//use std::process;
//use std::collections::HashMap;
use std::fs;
//use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args[1].as_ref() {
        "new" => {
            if args.len() > 2{
                new_proj(&args[2]);
            }else{
                print_usages();
                eprintln!("You must supply a name when using the 'new' command!");
            }
        },
        "initialize" => {
            reinitialize_proj();
        },
        "run" => {
            run_proj();
        },
        _ => print_usages()
    }
}
fn print_usages(){
    let commands = [
        ["new", "creates new project with parameters: [name]"], 
        ["initialize", "recreates files in project dir"],
        ["build", "builds files required executable code"], 
        ["run", "builds & execute the x86 Assembly code"]
    ];
    println!("\nCOMMANDS: [new initialize build run]");
    for i in commands.iter() {
        println!("\t{}: {}", i[0], i[1]);
    }
}
/*fn print_usage() -> HashMap<&str, &str>{
    let mut usages: HashMap<&str, &str> = HashMap::with_capacity(4);
    let commands = [
        ["new", "creates new project with [name]"], 
        ["initialize", "recreates files in project dir"],
        ["build", "builds files required executable code"], 
        ["run", "builds & execute the x86 Assembly code"]
    ];
    for cmd in commands{
        usages.insert(cmd[0], cmd[1]);
    }
    println!("COMMANDS: [new initialize build run]");
    for (key, val) in usages.iter() {
        println!("\t{}: {}", key, val);
    }
    return usages;
}*/
fn new_proj(name: &String){
    // create directory
    let fmt = format!("./{}", name);
    let path = Path::new(&fmt);
    let mut builder = fs::DirBuilder::new()
        .recursive(false)
        .create(path).unwrap();
    assert!(fs::metadata(path).unwrap().is_dir());
    
}
fn reinitialize_proj(){
    // go through each file in folder
    // and put files that may not exist
}
fn run_proj(){
    // compiles and executes code
}