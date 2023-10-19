use std::path::PathBuf;


use whiskey_python::{whiskey_python_parsing::{get_external_script, parse_whiskey_commands, get_exe_path, get_byte_order}, interpreter::run_process};

use crate::general_functions::spawn_whiskey_wine_files;





const WHISKEY_PYTHON_SCRIPT_CONTENTS:&'static  str = "

$set_external_script {


attach()

#do stuff here

dettach()

    

}

";


//this holds all the file names for easy changing if needed 
mod file_names {
    
    pub const IDENTIFIER_FILE:&'static str = "identify_whiskey";
    pub const WHISKEY_SCRIPT_FILE:&'static str = "main.wisk";
    
}

mod errors {
    
    

    //prints out red text
    pub fn output_red_text(text:&str) {
        
        use crossterm::style::{Color, StyledContent, style};
        use crossterm::style::Stylize;

        let red_text: StyledContent<&str> = style(text).with(Color::Rgb {
            r: 255,
            g: 0,
            b: 0,
        });

        println!("{}",red_text);
    
    }

    //the rest are self explanatory 
    

    pub fn no_exe_path() {


        output_red_text("no exe path supplied");
    }

    pub fn no_hack_name() {

        output_red_text("no name supplied");

    }

    pub fn hack_already_exists() {
        
        output_red_text("the hack folder already exists");

    }

    pub fn not_a_hack_directory() {

        output_red_text("the supplied directory is not a whiskey hack directory");

    }

    pub fn couldnt_compile() {

        output_red_text("failed to compile");

    }

    pub fn no_external_script() {

        output_red_text("no external script supplied");

    }
    
    pub fn no_byte_order() {

        output_red_text("no byte order supplied");
    }

    pub fn invalid_byte_order() {

        output_red_text("invalid byte order supplied");
    }

    pub fn couldnt_run() {
        
        output_red_text("a problem occured when running the exe");

    }

    pub fn couldnt_run_script() {
        
        output_red_text("a problem occured when running the whiskey script");

    }
    pub fn not_root() {
        
        output_red_text("you need to be root");

    }
    pub fn no_user() {

        output_red_text("no user has been supplied");

    }

    pub fn exe_not_found() {
        
        output_red_text("the exe file was not found");
        
    }

    pub fn process_timeout_not_int() {
        
        output_red_text("the process timeout must be an integer");
        
    }

}


mod general_functions {
    use std::{path::PathBuf, io::Write};

    use whiskey_python::{CompiledScripts, init_whiskey_python::{spawn_whiskey_python_files, check_if_whiskey_python_files_exist, wipe_whiskey_python_files, define_compiled_files_struct}};

    use crate::WHISKEY_PYTHON_SCRIPT_CONTENTS;

    

    


    
    //spawns a file use to identify if the directory is a whiskey directory
    pub fn spawn_identifier_file(path:&PathBuf)->Result<(),std::io::Error> {
        
        use std::fs::File;
        use super::file_names::*;

        let mut identifier_path:PathBuf = PathBuf::from(path);

        identifier_path.push(IDENTIFIER_FILE);

        File::create(identifier_path)?;

        return Ok(());

    }

    //checks if a the identifier file exists 
    pub fn check_for_identifier_file(path:&PathBuf)->bool{
        
        use std::path::Path;
        use crate::file_names::IDENTIFIER_FILE;

        let mut identifier_path:PathBuf = PathBuf::from(path);
        

        identifier_path.push(IDENTIFIER_FILE);
        
        

        return Path::new(&identifier_path).is_file();

    }

    //spawns the whiskey script and then writes to it
    pub fn spawn_write_whiskey_script(path:&PathBuf,contents:&str)->Result<(),std::io::Error> {

        
        use std::fs::*;        
        use super::file_names::WHISKEY_SCRIPT_FILE;

        let mut path_to_whiskey_script:PathBuf = PathBuf::from(path);
        
        path_to_whiskey_script.push(WHISKEY_SCRIPT_FILE);

        File::create(path_to_whiskey_script)?.write_all(contents.as_bytes())?;

        return Ok(());

    }

    //checks for whiskey script file 
    pub fn check_for_whiskey_script_file(path:&PathBuf)->bool {

        use std::path::PathBuf;
        use std::path::Path;
        use super::file_names::*;

        let mut path_to_whiskey_script:PathBuf = PathBuf::from(path);
            
        
        path_to_whiskey_script.push(WHISKEY_SCRIPT_FILE);

        return Path::new(&path_to_whiskey_script).is_file();

    }

    //reads the whiskey script file 
    pub fn read_whiskey_script(path:&PathBuf)->Result<String, std::io::Error>{

        use std::fs::*;
        use super::file_names::*;
        
        let mut file_path:PathBuf = PathBuf::from(path);

        file_path.push(WHISKEY_SCRIPT_FILE);

        

        return Ok( read_to_string(&file_path)? );
        
    }

    //compiles the whiskey script, returns an array of strings 
    // 1. external_script 
    // 2. exe_path 
    // 3. byte_order
    // this may change in a later version
    pub fn compile_whiskey_script(path:&PathBuf)->Result<[String;3],std::io::Error> {
        
        use whiskey_python::whiskey_python_parsing::*;
        use std::fs::*;

        let mut array:[String;3] = [

            String::new(),
            String::new(),
            String::new()

        ];

        let whiskey_script:String = read_whiskey_script(path)?;

        let whiskey_commands:Vec<String> = parse_whiskey_commands(&whiskey_script)?;

        array[0] = get_external_script(&whiskey_commands)?;
        
        array[1] = get_exe_path(&whiskey_commands)?;
        
        array[2] = get_byte_order(&whiskey_commands)?;
        
        return Ok( array );

    }
    
    //define compiled whiskey python code
    pub fn define_compiled_whiskey_python_code(path:&str,internal_script:&str)->Result<CompiledScripts, std::io::Error>{
    
        return Ok( define_compiled_files_struct(path,internal_script));


    }
    
    //creates the directory with the compiled whiskey python files 
    pub fn spawn_compiled_whiskey_file(compiled_scripts:&CompiledScripts) -> Result<(),std::io::Error> {
        
        spawn_whiskey_python_files(compiled_scripts)?;

        return Ok(());


    }
    //checks if the files already exists then wipes then spawns the whiskey python files
    pub fn wipe_spawn_whiskey_python_files(compiled_scripts:&CompiledScripts)->Result<(),std::io::Error> {
        
        if check_if_whiskey_python_files_exist(&compiled_scripts) {
            
            wipe_whiskey_python_files(&compiled_scripts)?;

        }
        spawn_whiskey_python_files(compiled_scripts)?;

        return Ok(());
    }

    //spawns the whiskey wine api files 
    pub fn spawn_whiskey_wine_files(exe_path:&PathBuf,
                                    spawn_path:&PathBuf,
                                    user:&str,
                                    process_name:Option<&str>,
                                    process_timeout:usize) -> Result<(),std::io::Error> {

        use whiskey_wine_api::*;

        define_process(
            &exe_path.clone().into_os_string().into_string().unwrap(),
            &spawn_path.clone().into_os_string().into_string().unwrap(),
            &user,
            process_name,
            process_timeout

                       )?;

        return Ok(());



    }

    //combines byte order and exe path to whiskey script 
    pub fn combine_whiskey_script(exe_path:&str,byte_order:&str,user:&str)->String {
            
        return format!("$set_exe_path {}\n$set_byte_order {}\n$set_user {}\n\n",exe_path,byte_order,user) + WHISKEY_PYTHON_SCRIPT_CONTENTS;
        
    }

    //check if root 
    pub fn is_root()->bool {
        
        use nix::unistd::Uid;
        
        return Uid::effective().is_root();

    }

    
    



}

//creates all of the whiskey files needed for the cli, not for the wine api or whiskey python 
fn create_whiskey_files(hack_name:&str,path:&str,exe_path:&str,user:&str)->Result<(),std::io::Error> {

    use std::fs::*;
    use std::path::PathBuf;
    use general_functions::*;

    let mut current_path:PathBuf = PathBuf::from(path);

    current_path.push(hack_name);
    
    output_green_text("creating files");

    create_dir(&current_path)?;
    
    spawn_identifier_file(&current_path)?;
    
    spawn_write_whiskey_script(&current_path, 
                               &combine_whiskey_script(exe_path, 
                                                       "little_endian",
                                                       user))?;
    
    output_green_text("done making files");
    
    return Ok(());
}

//outputs green text to the terminal 
fn output_green_text(text:&str) {

    use crossterm::style::{Color, StyledContent, style};
    use crossterm::style::Stylize;

    let green_text: StyledContent<&str> = style(text).with(Color::Rgb {
        r: 0,
        g: 200,
        b: 45,
    });

    println!("{}",green_text);

}


//check if directory with the hack_name already exists 
fn does_hack_already_exist(hack_name:&str,path:&str)->bool {
    
    use std::path::PathBuf;
    use std::path::Path;


    let mut hack_path:PathBuf = PathBuf::from(path);
    hack_path.push(hack_name);

    return Path::new(
        &hack_path.into_os_string().into_string().unwrap()
        ).is_dir();

}

//checks the directory if its a whiskey hack directory 
fn is_directory_whiskey_hack_directory(spawn_path:&str)->bool {


    use general_functions::*;

    let path:PathBuf = PathBuf::from(spawn_path);

    

    if check_for_identifier_file(&path) != true {
        
        return false; 

    }
    
    if check_for_whiskey_script_file(&path) != true {
        
        return false;

    }

    return true;



}


//this function runs and maneges the hack
fn run_hack(spawn_path:&PathBuf) {


    use whiskey_python::init_whiskey_python::*;
    use whiskey_python::init_whiskey_python::spawn_whiskey_wine_files;
    use whiskey_python::whiskey_python_parsing::*;
    use whiskey_python::interpreter::*;
    use whiskey_python::CompiledScripts;
    use whiskey_wine_api::WindowsProcess;
    use crate::general_functions::*;
    use errors::*;
    use std::path::Path;

    output_green_text("compiling...");


    let whiskey_script:String;

    match read_whiskey_script(spawn_path) {

        Err(error)=> {
            couldnt_compile();
            output_red_text(&error.to_string());
            return;
            },
        Ok(value)=> whiskey_script = value,

    }

    

    let whiskey_commands:Vec<String>; 

    match parse_whiskey_commands(&whiskey_script) {
        
        Err(error)=> {
            couldnt_compile();
            output_red_text(&error.to_string());
            return;
        },
        Ok(value)=> whiskey_commands = value,

    }

    




    let external_script:String;

    match get_external_script(&whiskey_commands) {
        
        Err(error)=>{
            no_external_script();
            output_red_text(&error.to_string());
            return;
        },
        Ok(value)=> external_script = value

    }



    

    let compiled:CompiledScripts;

    match define_compiled_whiskey_python_code(
        &spawn_path.clone().into_os_string().into_string().unwrap(),
        &external_script){


        Err(error)=> {
            couldnt_compile();
            output_red_text(&error.to_string());
            return;
        },

        Ok(value)=> compiled = value

    }

    
    

    check_wipe_whiskey_python_files(&compiled);
        
        
    

    match spawn_whiskey_python_files(&compiled) {
        
        Err(error)=> {
            couldnt_compile();
            output_red_text(&error.to_string());
            return;
        }, 
        Ok(_)=>{}

    }
    
    let process_name:Option<String>; 

    match get_process_name(&whiskey_commands) {
        
        Err(_)=> process_name = None,
        Ok(value)=> process_name = Some(value)

    }

    println!("{:?}",process_name);
    

    let exe_path:String; 

    match get_exe_path(&whiskey_commands) {
        
        Err(_)=>{
            no_exe_path();
            return;
        },
        Ok(value)=> exe_path = value

    }
    
    if !Path::new(&exe_path).is_file() {
        
        exe_not_found();
        return;

    }

    let byte_order:String; 

    match get_byte_order(&whiskey_commands) {
        
        Err(_)=>{
            no_byte_order();
            return;

        },
        Ok(value)=> byte_order = value

    }

    let user:String; 

    match get_user(&whiskey_commands) {
        
        Err(_)=>{
            no_user();
            return;

        },
        Ok(value)=> user = value

    }
    let process_timeout_str:String; 

    match get_process_timeout(&whiskey_commands) {
        
        Err(_)=>process_timeout_str = String::from("60"),
        Ok(value)=> process_timeout_str = value

    }

    let process_timeout:usize; 

    match process_timeout_str.parse() {
        
        Err(_)=>{
            process_timeout_not_int();
            return;
        }
        Ok(value)=>process_timeout=value

    }

    let byte_order_converted:u8;

    match convert_byte_order(&byte_order) {

        Err(_)=>{
            invalid_byte_order();
            return;
        },
        Ok(value)=> byte_order_converted = value

    }
    
    let windows_process:WindowsProcess;

    match spawn_whiskey_wine_files(&exe_path, 
                                   &spawn_path.clone().into_os_string().into_string().unwrap(),
                                   &user,
                                   process_name.as_deref(),
                                   process_timeout
                                   ) {
        
        Err(error)=> {
            couldnt_compile();
            output_red_text(&error.to_string());
            return;
        },

        Ok(value)=> windows_process = value
        
    }



    set_windows_process(windows_process);

    set_byte_order(byte_order_converted);

    output_green_text("finished compiling");

    output_green_text("running exe");
    
    match run_process() {

        Err(error)=>{
            
            couldnt_run();
            output_red_text(&error.to_string());
            return;
        },
        Ok(_)=>{}

    }

    output_green_text(&(String::from("success running exe! pid is ")+&get_pid().to_string()));

    output_green_text("running whiskey python scripts, hit ctrl+c to end it");

    match run_internal_python_script(&compiled) {
        

        Err(error)=>{
            
            couldnt_run_script();
            output_red_text(&error.to_string());
            return;
        },
        Ok(_)=>{}


    }
    
    output_green_text("done running python scripts");
    
    

}

fn main() {
    
    use std::*; 
    use env::*;
    use args;
    use std::str::Chars;
    use errors::*;
    use std::path::Path;
    use std::path::PathBuf;
    use general_functions::*;

    let arguments:Vec<String> = args().collect();  
    let mut exe_path:String = String::new();
    let mut spawn_path:String = String::from("./");
    let mut user:String = String::new();
    let mut new_hack:bool = false;
    let mut final_argument:bool = false; //finish making this
    let mut hack_name:String = String::new();
    let mut run:bool = false;


    for i in 0..arguments.len() {

        
        
        let argument_chars:Vec<u8> = Vec::from( arguments[i].as_bytes() );
        
        if i == arguments.len()-1{

            final_argument = true;
            
        }

        if arguments[i] == "new_hack" {
            
            new_hack = true;
            
        }

        if arguments[i] == "run" {

            run = true;

        }

        else if argument_chars[0] != '-' as u8 && argument_chars[1] != '-' as u8 {
            
            continue;

        }
        
        if final_argument {

            continue;

        }
        
        else if arguments[i] == "--exe_path" {
            
            exe_path = arguments[i+1].clone();
            
        }
        else if arguments[i] == "--spawn_path" {
            
            spawn_path = arguments[i+1].clone();
            
        }
        
        else if arguments[i] == "--name" {

            hack_name = arguments[i+1].clone();

        }

        else if arguments[i] == "--user" {

            user = arguments[i+1].clone();

        }

    

        



    }

    if new_hack && exe_path != String::new() && hack_name != String::new() && user != String::new(){
        
        
        if does_hack_already_exist(&hack_name, &spawn_path) {
            
            hack_already_exists();
            
            return;

        }
        
    
        match create_whiskey_files(&hack_name,&spawn_path,&exe_path,&user) {
            
            Ok(_) => return,
            Err(error) => output_red_text(
                &format!("Error when creating files: {}",error)
                )
        

        }
            
    }

    if new_hack && exe_path == String::new() {
        
        no_exe_path();
        
        return ;

    }

    if new_hack && hack_name == String::new() {
        
        no_hack_name();

        return;

    }

    if new_hack && user == String::new() {
        
        no_user();

        return;

    }

    if run {
        
        if !is_root() {

            not_root();
            
            return;

        }

        if is_directory_whiskey_hack_directory(&spawn_path) != true {
            
            println!("{}",spawn_path);

            not_a_hack_directory();

            return;


        }

        run_hack(&PathBuf::from(spawn_path));
        
        

        return; 

        
    }

    

}
