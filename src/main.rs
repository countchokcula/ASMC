static ASSEMBLY_X86: &'static str = "bits 16
[org 0x7c00]
initialize:
    mov bx, 0x8000 
    mov ss, bx ; move stack segment to 0x8000
    mov bp, sp ; resets base pointer

    mov ax, 0000h ;set video mode
    int 10h

    mov ax, 0500h ;set page
    int 10h

    mov ax, 0b00h ;set backgroun color to green
    mov bx, 0002h
    int 10h
_main:
    mov ax, 1300h ;display message
    mov bx, 000fh
    mov cx, len
    mov dx, 0h
    mov bp, hello_world
    int 10h
    
    mov bp, sp

hello_world db 'Hello, world!',0xa,0
len $-hello_world";

use std::env;
use std::path::Path;
use std::fs::File;
use std::error::Error;
//use std::process;
//use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    // NOTE: there should be 3 arguements when executing the new
    // command. args[0] == name of program
    // args[1] == command
    // args[2] == option
    if args.len() > 1{
        match args[1].as_ref() {
            "new" => {
                if args.len() > 2{
                    new_proj(&args[2]);
                }else{
                    print_usages();
                    panic!("You must supply a name when using the 'new' command!");
                }
            },
            "init" => {
                initialize_proj();
            },
            "run" => {
                run_proj();
            },
            _ => print_usages()
        }
    } else {
        print_usages();
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

fn new_proj(name: &String){
    // create directory
    let fmt = format!("./{}", name);
    let root = Path::new(&fmt);

    if root.is_dir(){
        println!("-- {} Already exists, reinitialize with 'asmc initialize'", name);
    }else{
        match fs::DirBuilder::new().recursive(false).create(root){
           Err(why) => panic!("-- Could not create {}: {}", fmt, why.description()),
           Ok(_) => create_paths(&name)
        }; // Create Root Directory
        create_asmc(&fmt, &name);
    }
    
}
fn create_paths(name: &String){
    for path in ["src", "include", "build"].iter() {
        let fmt = format!("./{}/{}", name, path);
        let new_path = Path::new(&fmt);

        if new_path.is_dir(){
            println!("\t-- {} already exist!", fmt);
        }else{
            if let Err(why) = fs::DirBuilder::new().recursive(false).create(new_path){
                panic!("Could not create Directory: {}", why.description());
            }
            assert!(fs::metadata(new_path).unwrap().is_dir());
            println!("\t== Created {}", fmt);
            
            if "src" == *path {
                create_main_asm(&fmt);
            }
        }
    }
}
fn create_main_asm(fmt: &str){
    let new_fmt = format!("{}/main.asm", fmt);
    let new_path = Path::new(&new_fmt);
    let display = new_path.display();
    
    let mut file = match File::create(&new_path) {
        Err(why) => panic!("Error creating file {}: {}", display, why.description()),
        Ok(file) => {
            println!("\t== Created {}", display);
            file
        }  
    }; // END
    if let Err(why) = file.write_all(ASSEMBLY_X86.as_bytes()) {
        panic!("Error writing to {}: {}", display, why.description());
    } 
    if let Err(why) = file.sync_data() {
        panic!("Error syncing to {}: {}", display, why.description());
    }
}
fn initialize_proj(){
    // go through each file in folder
    // and put files that may not exist
    let check_main = || {
        let main_asm = "./src/main.asm";
        if let Err(_why) = File::open(main_asm){ // if file does not exist
            if let Err(why) = File::create(main_asm){ // create new file
                panic!("Could not write to {}, try changing to the root(the name) directory of the project: {}", main_asm, why.description())
            }
        }
    };
    match File::open("./.asmc"){ 
        Err(_why) => match File::create("./.asmc"){ //if .asmc does not exist create new file
            Err(why) => panic!("Could not initialize, error writing to ./.asmc to {}: ", why.description()),
            Ok(_) => check_main()
        },
        Ok(_) =>  check_main() // if .asmc does exist create main
    }
}
fn create_asmc(fmt: &String, name: &String){
    let new_fmt = format!("{}/.asmc", fmt);
    if let Err(_why) = File::open(&new_fmt){ 
        if let Err(why) = File::create(&new_fmt){ //if .asmc does not exist create new file
            panic!("Could not initialize, error writing to {}/.asmc to {}: ", name, why.description());
        }
    }
}
fn run_proj(){
    // compiles and executes code
}