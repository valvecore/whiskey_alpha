


//start of use commands 


//end of use commands 


//start of structs 

pub struct CompiledScripts{

    pub internal_script:String,
    pub path:String,
    

}


//end of structs


//start of modules

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

    //the rest is self explanatory

    pub fn may_be_uninitialized(){

        output_red_text("WARNING! recently read address may be uninitialized");

    }

}



pub mod python_code {
    
    pub const STARTING_PYTHON_CODE:&'static str =
"
global handle_possible_error
global hex_to_int 
global is_alive
global read_32
global read_64 
global write_32 
global read_8
global write_64
global write_8

def handle_possible_error(error_code):
    
    if error_code == 1 :

        raise Exception('unknown error')

    if error_code == 2 :

        raise Exception('attempted write or read to invalid address')
        
def hex_to_int(hex_value): # this function is from phind ai
    # Remove '0x' prefix if it exists
    
    hex_value = hex_value.replace('\\n','')

    if hex_value.startswith('0x'):
        hex_value = hex_value[2:]

    # Map of hexadecimal characters to decimal values
    hex_map = {
        '0': 0, '1': 1, '2': 2, '3': 3, '4': 4, '5': 5, '6': 6, '7': 7, '8': 8, '9': 9,
        'a': 10, 'b': 11, 'c': 12, 'd': 13, 'e': 14, 'f': 15
    }

    # Convert the hexadecimal number to an integer
    int_value = 0
    for i, digit in enumerate(reversed(hex_value)):
        int_value += hex_map[digit] * (16 ** i)
    
    return int_value

def is_alive():

    return from_rust_check_alive()

def read_32(address):
    
    return_array = from_rust_read_32bit_value(address)

    handle_possible_error(return_array[1])

    return return_array[0]

def read_64(address):
    
    return_array = from_rust_read_64bit_value(address)

    handle_possible_error(return_array[1])

    return return_array[0]

def read_8(address):

    return_array = from_rust_read_8bit_value(address)

    handle_possible_error(return_array[1])

    return return_array[0]

def write_32(address,data):

    handle_possible_error(from_rust_write_32bit_value(address,data))

def write_64(address,data):

    handle_possible_error(from_rust_write_64bit_value(address,data))
def write_8(address,data):

    handle_possible_error(from_rust_write_8bit_value(address,data))
    
";



}


pub mod whiskey_python_file_paths {

    pub const WHISKEY_PYTHON_FOLDER_NAME:&'static str="WHISKEY_PYTHON_DONOTEDIT";
    pub const WHISKEY_PYTHON_EXTERNAL_SCRIPT_NAME:&'static str="external_whiskey_python_script.py";
    pub const WHISKEY_PYTHON_INTERNAL_SCRIPT_NAME:&'static str="internal_whiskey_python_script.py";
    

}

pub mod whiskey_command_types { 
    
    pub const SET_EXE_PATH:&'static str = "set_exe_path" ;

    pub const SET_EXTERNAL_SCRIPT:&'static str = "set_external_script" ;
    
    pub const SET_BYTE_ORDER:&'static str = "set_byte_order" ;

    pub const SET_USER:&'static str = "set_user" ;

    pub const SET_PROCESS_NAME:&'static str = "set_process_name";

    pub const SET_PROCESS_TIMEOUT:&'static str = "set_process_timeout";
}

pub mod general_functions {
    use crate::whiskey_python_parsing::get_last_command_used;

    
    macro_rules!  add_forward_slash_to_path{
        ($a:expr) => {
            {
                String::from($a)+"/"
            }
            
        };
    }
    macro_rules!  add_backward_slash_to_path{
        ($a:expr) => {
            {
                String::from($a)+"\\"
            }
            
        };
    }

    pub fn check_then_add_slash_to_path(path:&str,forward_slash:bool)->String{
        let mut new_path:String=String::from(path);
        match new_path.as_bytes()[new_path.len()-1] as char{
            '/'=>return new_path,
            '\\'=>return new_path,
            _=>{}
        }
        new_path=match forward_slash{
            true=>add_forward_slash_to_path!(new_path),
            false=>add_backward_slash_to_path!(new_path)
        };
        
        return new_path

    }
    /*
    this function checks what kind of slashes a string that contains a path uses, returns true for a forward slash
    returns false for a back slash
    */
    pub fn check_for_slash_type(path:&str)->bool{
        return path.contains("/");
    }
    
    //spawns a file, the file path should be in the first parameter, and the second parameter
    //should be the data to write to the file
    pub fn create_then_write_file(path:&str,contents:&str)->Result<(),std::io::Error>{
        use std::io::Write;
        use std::fs::File;
        

        File::create(path)?.write_all(contents.as_bytes())?;
        return Ok(());
    
    }
    
    // this function creates the directory meant to contain the compiled whiskey python 
    pub fn create_whiskey_python_directory(path:&str)->Result<(),std::io::Error>{
        use super::whiskey_python_file_paths::*;
        let whiskey_python_directory:String=check_then_add_slash_to_path(
            path, 
            check_for_slash_type(path))+WHISKEY_PYTHON_FOLDER_NAME;

        

        
        std::fs::create_dir(whiskey_python_directory)?;
        
        return Ok(());    

    }
    //this function creates the compiled internal python file inside the main whiskey_python_files
    //directory then writes the contents to it 
    pub fn create_write_whiskey_python_internal_file(path:&str,contents:&str)->Result<(),std::io::Error>{
        
        use super::whiskey_python_file_paths::*;

        let mut current_path:String = add_whiskey_files_dir_to_path(path);
            
        
        

       current_path = add_whiskey_internal_script_name_to_path(&current_path);       

        create_then_write_file(
            &current_path                              
            , 
            contents)?;
        return Ok(());

    }
    
    //creates and writes the necessary compiled files for whiskey python 
    pub fn create_write_whiskey_python_files(path:&str,internal_script_contents:&str)->Result<(),std::io::Error>{
        
        create_whiskey_python_directory(path)?;

        create_write_whiskey_python_internal_file(path, internal_script_contents)?;

        return Ok(());

    }
    //adds the whiskey python directory name to the provided path str
    pub fn add_whiskey_files_dir_to_path(path:&str)->String{
        
        use super::whiskey_python_file_paths::*;

        let current_path:String = 
            check_then_add_slash_to_path(
                path,
                check_for_slash_type(path)) + WHISKEY_PYTHON_FOLDER_NAME;

        return current_path;

    }

    //adds the whiskey internal script name to the provided path str
    pub fn add_whiskey_internal_script_name_to_path(path:&str)->String{

        use super::whiskey_python_file_paths::*;

        let current_path:String =
           check_then_add_slash_to_path(
            path,
            check_for_slash_type(path))
            +WHISKEY_PYTHON_INTERNAL_SCRIPT_NAME;

        return current_path;
    }
    
    
    


}

pub mod whiskey_python_parsing {
    use crate::whiskey_command_types::SET_USER;

    
    const WHISKEY_COMMAND_IDENTIFIER:char='$';


  
    //returns the byte order from whiskey commands 
    pub fn get_byte_order(whiskey_commands:&Vec<String>)->Result<String,std::io::Error> {
        
        use super::whiskey_command_types::SET_BYTE_ORDER;

        return Ok(
            get_last_command_used(whiskey_commands, SET_BYTE_ORDER)?[0].clone()
            );

    }

    //gets the command name and parameter out of whiskey parsed commands by command type

    pub fn get_whiskey_commands(whiskey_commands:&Vec<String>,command_type:&str)->Result<Vec<String>,std::io::Error> {
        
        use std::io::Error;
        use std::io::ErrorKind::*;

    
        let mut parsed_whiskey_commands: Vec<String> = Vec::new();
        let mut scan_stage:u8 = 0;
        let mut count: usize = 0;
        let mut scan: bool = false;
        
        

        for string in whiskey_commands {
            
            if scan_stage == 0 {

                scan = false;

                if string == command_type {
                    
                    scan = true;
                }
    
            }
            else if scan_stage <= 2 {
                
                if scan{
                    parsed_whiskey_commands.push(string.to_string());
                    count+=1;
                }
            }

            

            scan_stage+=1;

            if scan_stage == 3 {
                
                scan = false;
                scan_stage = 0;
                
            }
            

        }
        
        if count == 0{
            
            return Err(
                    Error::new(
                        InvalidData,
                        "couldnt find any matching commands"
                        )    
                );

        }

        return Ok(parsed_whiskey_commands);


    }
    //gets the last command called with the command type 

    pub fn get_last_command_used(whiskey_commands:&Vec<String>,command_type:&str)->Result<[String;2],std::io::Error>{
        
        use std::io::Error; 
        use std::io::ErrorKind::*;

        let parsed_commands:Vec<String> = get_whiskey_commands(&whiskey_commands,command_type)?;

        let buffer_point:usize = parsed_commands.len()-2;

        if buffer_point >= parsed_commands.len() {
            
            return Err(
                    Error::new(
                        InvalidInput,
                        "the pointer to the command was larger than the commands len"
                        )
                );

        }

        let whiskey_command:[String;2] = [
        parsed_commands[buffer_point].clone(),
        parsed_commands[buffer_point+1].clone()
        ];

        return Ok(
            whiskey_command
        );


        

        
    }
    //gets the whiskey command by its command type and pointer 
    pub fn get_whiskey_command(whiskey_commands:&Vec<String>,command_type:&str,command_pointer:usize)->Result<[String;2],std::io::Error>{
        
        use std::io::Error; 
        use std::io::ErrorKind::*;

        let parsed_commands:Vec<String> = get_whiskey_commands(&whiskey_commands,command_type)?;

        let buffer_point:usize = command_pointer*3;

        if buffer_point >= parsed_commands.len() {
            
            return Err(
                    Error::new(
                        InvalidInput,
                        "the pointer to the command was larger than the commands len"
                        )
                );

        }

        let whiskey_command:[String;2] = [
        parsed_commands[buffer_point].clone(),
        parsed_commands[buffer_point+1].clone()
        ];

        return Ok(whiskey_command
        );



    }

    //returns the exe path from the whiskey commands

    pub fn get_exe_path(whiskey_commands:&Vec<String>)->Result<String,std::io::Error> {
        
        use super::whiskey_command_types::SET_EXE_PATH;

        return Ok(

                (get_last_command_used(whiskey_commands, SET_EXE_PATH)?)[0].clone() 
            );
        

    }
    //returns the process timeout from the whiskey commands

    pub fn get_process_timeout(whiskey_commands:&Vec<String>)->Result<String,std::io::Error> {
        
        use super::whiskey_command_types::SET_PROCESS_TIMEOUT;

        return Ok(

                (get_last_command_used(whiskey_commands, SET_PROCESS_TIMEOUT)?)[0].clone() 
            );
        

    }
    //returns the user from the whiskey commands

    pub fn get_user(whiskey_commands:&Vec<String>)->Result<String,std::io::Error> {
        
        use super::whiskey_command_types::SET_USER;

        return Ok(

                (get_last_command_used(whiskey_commands, SET_USER)?)[0].clone() 
            );
        

    }

    //returns the process name from the whiskey commands

    pub fn get_process_name(whiskey_commands:&Vec<String>)->Result<String,std::io::Error> {
        
        use super::whiskey_command_types::SET_PROCESS_NAME;

        return Ok(

                (get_last_command_used(whiskey_commands, SET_PROCESS_NAME)?)[0].clone() 
            );
        

    }

    //returns the external script from the whiskey commands 
    

    pub fn get_external_script(whiskey_commands:&Vec<String>)->Result<String,std::io::Error> {

        use super::whiskey_command_types::SET_EXTERNAL_SCRIPT;
        
        
        return Ok(

                get_last_command_used(whiskey_commands, SET_EXTERNAL_SCRIPT)?[1].clone() 
            );

    }

    //this function parses a single wiskey command, returns a list of 3 objects instead of a box.
    //refer to the commets before the parse_whiskey_commands function for more details
    pub fn parse_single_whiskey_command(text:&str)->Result<[String;3],std::io::Error>{

        use std::io::ErrorKind::InvalidData;
        use std::io::Error;

        let mut parsed:[String;3]=[String::new(),String::new(),String::new()];

        let mut command_type:String=String::new();
        let mut command_name:String=String::new();
        let mut command_parameters:String=String::new();

        let mut read_command_type:bool=true;
        let mut read_command_name:bool=false;
        let mut read_command_parameters:bool=false;
        
        let mut command_type_done:bool=false;
        let mut command_name_done:bool=false;

        for character in text.chars(){
            
            if read_command_type{
                
                if character != ' '{ 

                    command_type.push(character);
                    
                }

                else{
                    read_command_type=false;
                    command_type_done=true;
                    read_command_name=true;
                }
            }

            else if read_command_name{
                
                if character == '$' {
                    
        
                    return Err(
                        Error::new(InvalidData, 
                                   "invalid symbol in whiskey command")
                        );
                
                }

                if character != ' '{ 

                    command_name.push(character);
                    
                }

                else{
                    read_command_name=false;
                    read_command_type=false;
                    command_name_done=true;
                    read_command_parameters=true;
                }

            }
            else if read_command_parameters {
                
                command_parameters.push(character);
    
            }
        }

        if command_parameters == String::from("") || command_parameters == String::new() {
            command_name_done=true;

        }

        

        parsed[0] = command_type;
        parsed[1] = command_name;
        parsed[2] = command_parameters;
        
        

        if command_name_done != true{
            return Err(
                std::io::Error::new(
                    InvalidData,
                    "whiskey command not properly structured (no command type)"
                    )
                );
        }

        if command_type_done != true{
            return Err(
                std::io::Error::new(
                    InvalidData,
                    "whiskey command not properly structured (no command name)"
                    )
                );
        }
        
        return Ok(parsed);
    }

    //this function parses the whiskey commands out of the given text and returns it in the vector
    /*
    the format at which the whiskey commands are returned to is of the following 

    a parsed whiskey command is arranged in three objects inside the vector, first object is the command type, the second object
    is the command name. The third is the parameters passed into the whiskey command.
    
    
    */
    pub fn parse_whiskey_commands(text:&str)->Result<Vec<String>,std::io::Error>{

        let mut parsed:Vec<String> = Vec::new();
        
        let mut current_whiskey_command:String = String::new();
        let mut script:String = String::new();

        let mut read_until_new_line:bool = false;
        let mut finished_with_new_line:bool = false;
        let mut read_script:bool = false;
        let mut read_for_end_bracket = true;


        for character in text.chars(){
            
            if read_until_new_line == false && read_script == false {

                if character == WHISKEY_COMMAND_IDENTIFIER{
                    read_until_new_line=true;
                }
            }
            
            else if read_until_new_line && read_script == false{

                
                if character == '{'{ 
                    read_until_new_line=false;
                    read_script=true;
                    

                }
                else if character != '\n' {
                    finished_with_new_line=false;
                    current_whiskey_command.push(character);
                }
                else {

                    read_until_new_line=false;
                    
                    let single_command_buffer=parse_single_whiskey_command(&current_whiskey_command)?;
                    parsed.push(single_command_buffer[0].clone());
                    parsed.push(single_command_buffer[1].clone());
                    parsed.push(single_command_buffer[2].clone());
                    finished_with_new_line=true;
                    current_whiskey_command=String::new();
                }

            }

            else if read_script {
            
                if character == '}' && read_for_end_bracket{
                    
                    read_script = false;

                    let single_command_buffer=parse_single_whiskey_command(&current_whiskey_command)?;
                    parsed.push(single_command_buffer[0].clone());
                    parsed.push(single_command_buffer[1].clone());
                    parsed.push(script.clone());
                    finished_with_new_line = true;
                }
                else{
                    
                    script.push(character);
                    if character == '"' || character == '\'' {
                        
                        read_for_end_bracket = !read_for_end_bracket; //flips the bool value
                        
                    }

                }
    
            }



        }
        
        if finished_with_new_line != true{
            
            let single_command_buffer=parse_single_whiskey_command(&current_whiskey_command)?;
            parsed.push(single_command_buffer[0].clone());
            parsed.push(single_command_buffer[1].clone());
            parsed.push(single_command_buffer[2].clone());

        }
        

        return Ok(parsed);

    }
    
    //combines the starting python code with the python script 
    pub fn combine_starting_script_with(script:&str)->String {
        
        use super::python_code::STARTING_PYTHON_CODE;
        
        return format!("{}\n#-----\n{}",STARTING_PYTHON_CODE,script);
    }
    //converts the byte order str to a u8 
    pub fn convert_byte_order(byte_order:&str)->Result<u8,std::io::Error>{
        
        use whiskey_manipulator::byteorder::*;
        use std::io::Error;
        use std::io::ErrorKind::*;

        if byte_order == "little_endian" {
            
            return Ok(LITTLE_ENDIAN);

        }

        if byte_order == "big_endian" {

            return Ok(BIG_ENDIAN);
            
        }

        return Err(
            Error::new(
                InvalidData, 
                "invalid byte order"
                )
            );


    }
    
}

pub mod init_whiskey_python{
    
    use whiskey_wine_api::define_reusable_process;

    use super::*;

    //constructs the compiled files struct from the function parameters 
    pub fn define_compiled_files_struct(path:&str,internal_script_contents:&str)->CompiledScripts{
        
        use whiskey_python_parsing::*;

        let internal_script_combined:String = combine_starting_script_with(internal_script_contents);

        
        
        return CompiledScripts { internal_script:internal_script_combined.to_string(), path:path.to_string()};

    }

    //creates all the neccessary files for whiskey_python
    pub fn spawn_whiskey_python_files(compiled_scripts:&CompiledScripts)->Result<(),std::io::Error>{
        
        
        use std::io::Error;
        use std::io::ErrorKind::*;
        use std::path::Path;    
        use general_functions::*;

        if Path::new(&compiled_scripts.path).exists() == false {
            
            return Err(
                Error::new(
                    NotFound,
                    "directory does not exist"
                    )

                );

        }
        
        create_write_whiskey_python_files(&compiled_scripts.path, &compiled_scripts.internal_script)?;


        return Ok(());
    }
    //this function removes all of the whiskey python compiled files
    pub fn wipe_whiskey_python_files(compiled_scripts:&CompiledScripts)->Result<(),std::io::Error> {
        
        use general_functions::*;

        std::fs::remove_dir_all(
            add_whiskey_files_dir_to_path(&compiled_scripts.path)
                                )?;
        return Ok(());
        
    }

    //this function returns a bool if the whiskey python compiled files exist 
    pub fn check_if_whiskey_python_files_exist(compiled_scripts:&CompiledScripts)->bool{
        
        use super::general_functions::*;
        


        return std::path::Path::new(
            &add_whiskey_files_dir_to_path(
                &compiled_scripts.path)).exists();
        
    }
    
    //this functions wipes the whiskey python compiled files if they exist
    pub fn check_wipe_whiskey_python_files(compiled_scripts:&CompiledScripts){
        
        if check_if_whiskey_python_files_exist(&compiled_scripts) {
            
            wipe_whiskey_python_files(&compiled_scripts).unwrap();
        
        }
    }

    //spawns the whiskey wine files,doesnt reuse the files
    pub fn spawn_whiskey_wine_files(exe_path:&str,
                                    spawn_path:&str,
                                    user:&str,
                                    process_name:Option<&str>,
                                    process_timeout:usize)->
                                    Result<whiskey_wine_api::WindowsProcess,std::io::Error>{
        
        use whiskey_wine_api::*;
    
        
        return Ok(
            define_process(spawn_path,exe_path,user,process_name,process_timeout)?

            );

    }

    //spawns the whiskey wine files, does reuse the files
    pub fn spawn_and_reuse_whiskey_wine_files(path:&str,process_name:Option<&str>)->Result<whiskey_wine_api::WindowsProcess,std::io::Error> {
        
        use whiskey_wine_api::*;

        return define_reusable_process(path,process_name);
        
    }
    //sets the global variable inside of interpreter module for the windows process 
    pub fn set_windows_process(windows_process:whiskey_wine_api::WindowsProcess){
        
        use super::interpreter::CURRENT_WINDOWS_PROCESS;
        
        let mut global_windows_process = CURRENT_WINDOWS_PROCESS.lock().unwrap();
        *global_windows_process = windows_process;
        
    }
    //sets the byte order
    pub fn set_byte_order(byte_order:u8) {
        
        use super::interpreter::BYTE_ORDER;
        
        let mut global_byte_order = BYTE_ORDER.lock().unwrap();
        *global_byte_order = byte_order;

    }
   
}




pub mod interpreter {
    use crate::CompiledScripts;
    
    use lazy_static::lazy_static;
    use std::sync::Mutex;
   
    //gets the pid of the process
    pub fn get_pid()->usize {
        
        
        
        use std::sync::{mpsc, Arc, Mutex};
        
        


        let global_windows_process = CURRENT_WINDOWS_PROCESS.lock().unwrap();

        return (*global_windows_process).pid;

    }


    //detaches the process 

    pub fn detach_from_the_process() {
        
        use whiskey_manipulator::*;

        let global_manipulator_process = CURRENT_MANIPULATOR_WINDOWS_PROCESS.lock().unwrap();
                
        dettach(
            &*global_manipulator_process
            ).unwrap();

    }

    //injects into the process 
    pub fn inject_into_process() {
            
        
        use whiskey_manipulator::*;

        let mut global_manipulator_process = CURRENT_MANIPULATOR_WINDOWS_PROCESS.lock().unwrap();
        let global_windows_process = CURRENT_WINDOWS_PROCESS.lock().unwrap();
        let byte_order = BYTE_ORDER.lock().unwrap();
        
        *global_manipulator_process = attach(
            (*global_windows_process).pid,
            *byte_order
            ).unwrap();

        
    }

    //runs the process
    pub fn run_process()->Result<(),std::io::Error>{

        let mut global_windows_process = CURRENT_WINDOWS_PROCESS.lock().unwrap();
            
        
        (*global_windows_process).run()?;
        
            

        return Ok(());


    }

    lazy_static! {
        pub static ref CURRENT_WINDOWS_PROCESS: Mutex<whiskey_wine_api::WindowsProcess> = Mutex::new(
            whiskey_wine_api::WindowsProcess
            { 
                path: String::new(),
                whiskey_files_path: String::new(), 
                pid: 0, 
                running: false, 
                std_output: None,
                user: String::new(),
                process_name:None,
                process_timeout:60
            }
                );
    }

    lazy_static! {
        pub static ref BYTE_ORDER: Mutex<u8> = Mutex::new(0);
            
    }

    lazy_static! {
        pub static ref CURRENT_MANIPULATOR_WINDOWS_PROCESS: Mutex<whiskey_manipulator::WindowsProcess> = Mutex::new(
            whiskey_manipulator::WindowsProcess
            { 
                pid:0,
                attached:false,
                byteorder:0,
            }
            );
    }


    pub mod for_python_functions{
        use crate::errors::may_be_uninitialized;

        

        #[pyo3::pyfunction]
        pub fn from_rust_write_32bit_value(address:usize,data:u32)->usize {
            
            use super::*;
            use whiskey_manipulator::*;
            use whiskey_manipulator::error::write_address::INVALID_ADDRESS;
            use error::*;
                
            let global_manipulator_process = CURRENT_MANIPULATOR_WINDOWS_PROCESS.lock().unwrap();

            match write_32bit_address(&*global_manipulator_process,address,data) {
                
                Ok(_)=>return 0,
                Err(error)=>{
                    println!("ERRRR! {}",error);
                    if error == INVALID_ADDRESS {
                        return 2;
                    }
                    
                },
            

            }
            
            return 1;

        }
        #[pyo3::pyfunction]
        pub fn from_rust_write_64bit_value(address:usize,data:u64)->usize {
            
            use super::*;
            use whiskey_manipulator::*;
            use whiskey_manipulator::error::write_address::INVALID_ADDRESS;
            use error::*;
                
            let global_manipulator_process = CURRENT_MANIPULATOR_WINDOWS_PROCESS.lock().unwrap();

            match write_64bit_address(&*global_manipulator_process,address,data) {
                
                Ok(_)=>return 0,
                Err(error)=>{
                    println!("ERRRR! {}",error);
                    if error == INVALID_ADDRESS {
                        return 2;
                    }
                    
                },
            

            }
            
            return 1;

        }
        #[pyo3::pyfunction]
        pub fn from_rust_write_8bit_value(address:usize,data:u8)->usize {
            
            use super::*;
            use whiskey_manipulator::*;
            use whiskey_manipulator::error::write_address::INVALID_ADDRESS;
            use error::*;
                
            let global_manipulator_process = CURRENT_MANIPULATOR_WINDOWS_PROCESS.lock().unwrap();

            match write_8bit_address(&*global_manipulator_process,address,data) {
                
                Ok(_)=>return 0,
                Err(error)=>{
                    println!("ERRRR! {}",error);
                    if error == INVALID_ADDRESS {
                        return 2;
                    }
                    
                },
            

            }
            
            return 1;

        }

        #[pyo3::pyfunction]
        pub fn from_rust_check_alive()-> bool {
            
            use super::*;

            let mut global_windows_process = CURRENT_WINDOWS_PROCESS.lock().unwrap();

            return (*global_windows_process).alive();

        }
        #[pyo3::pyfunction]
        pub fn from_rust_read_8bit_value(address:usize)->[u8;2] {
            
            use super::*;
            use whiskey_manipulator::*;
            use whiskey_manipulator::error::read_address::INVALID_ADDRESS;
            use error::*;
                
            let global_manipulator_process = CURRENT_MANIPULATOR_WINDOWS_PROCESS.lock().unwrap();
            
            match read_8bit_address(&*global_manipulator_process,address) {
                
                Err(error)=>{
                    if error == INVALID_ADDRESS {
                        return [0,2];
                    }
                }
                
                Ok(value)=>{
                    
                    if value == 255 {
                        
                        may_be_uninitialized()
                    }

                    return [value,0]

                }

            };
            
            return [0,1];
            
        }
        #[pyo3::pyfunction]
        pub fn sleep(time:usize){ 
            
            use std::thread;
            use std::time::Duration;

            thread::sleep(Duration::from_millis(time.try_into().unwrap()));

        }
        #[pyo3::pyfunction]
        pub fn input(before_string:&str)->String {
            
            use std::io::Write;
            use std::io::stdout;
            use std::io::stdin;

            let mut string = String::new();
            
            print!("{}",before_string);
            stdout().flush();
            stdin().read_line(&mut string).unwrap();
            

            return string;


        }
        #[pyo3::pyfunction]
        pub fn inject() {
            
            use super::*;
            use whiskey_manipulator::*;

            let mut global_manipulator_process = CURRENT_MANIPULATOR_WINDOWS_PROCESS.lock().unwrap();
            let global_windows_process = CURRENT_WINDOWS_PROCESS.lock().unwrap();
            let byte_order = BYTE_ORDER.lock().unwrap();
            
            *global_manipulator_process = attach(
                (*global_windows_process).pid,
                *byte_order
            ).unwrap();
        }

        #[pyo3::pyfunction]
        pub fn un_inject() {
            
            use super::*;
            use whiskey_manipulator::*;

            let mut global_manipulator_process = CURRENT_MANIPULATOR_WINDOWS_PROCESS.lock().unwrap();
                
            *global_manipulator_process = dettach(
                &*global_manipulator_process
            ).unwrap();

        
        }
        #[pyo3::pyfunction] 
        pub fn force_continue() {
            
            use super::*;
            use whiskey_manipulator::*;

            let global_manipulator_process = CURRENT_MANIPULATOR_WINDOWS_PROCESS.lock().unwrap();
                
            force_continue(&*global_manipulator_process);      

        }
        
        #[pyo3::pyfunction]
        pub fn from_rust_read_32bit_value(address:usize)->[u32;2] {

            use super::*;
            use whiskey_manipulator::*;
            use whiskey_manipulator::error::*;
            
            let global_manipulator_process = CURRENT_MANIPULATOR_WINDOWS_PROCESS.lock().unwrap();
            
            

            match read_32bit_address(&*global_manipulator_process, address) {

                Ok(value)=> return [value,0],
                Err(error)=>{
                    
                    if error == read_address::INVALID_ADDRESS {

                        return [0,2];

                    }

                    else{
                        
                        return [0,1];

                    }


                }

            }

        }
        #[pyo3::pyfunction]
        pub fn from_rust_read_64bit_value(address:usize)->[u64;2] {

            use super::*;
            use whiskey_manipulator::*;
            use whiskey_manipulator::error::*;
            
            let global_manipulator_process = CURRENT_MANIPULATOR_WINDOWS_PROCESS.lock().unwrap();
            
            

            match read_64bit_address(&*global_manipulator_process, address) {

                Ok(value)=> return [value,0],
                Err(error)=>{
                    
                    if error == read_address::INVALID_ADDRESS {

                        return [0,2];

                    }

                    else{
                        
                        return [0,1];

                    }


                }

            }

        }

        #[pyo3::pyfunction]
        pub fn print_read_error() {
            
            use whiskey_manipulator::*;

            read_error();

        }

    }
    //runs the internal script
    pub fn run_internal_python_script(compiled_scripts:&CompiledScripts)->pyo3::PyResult<()> {
        
        use pyo3::prelude::*;
        use pyo3::types::IntoPyDict;
        use pyo3::types::PyString;
        use pyo3::wrap_pyfunction;
        use pyo3::types::PyDict;
        use for_python_functions::*;
        use super::python_code::*;
        



        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let globals = PyDict::new(py);
            globals.set_item("from_rust_check_alive", wrap_pyfunction!(from_rust_check_alive, py)?)?;
            globals.set_item("from_rust_read_8bit_value", wrap_pyfunction!(from_rust_read_8bit_value, py)?)?;
            globals.set_item("from_rust_read_32bit_value", wrap_pyfunction!(from_rust_read_32bit_value, py)?)?;
            globals.set_item("from_rust_write_32bit_value", wrap_pyfunction!(from_rust_write_32bit_value, py)?)?;
            globals.set_item("from_rust_read_64bit_value", wrap_pyfunction!(from_rust_read_64bit_value, py)?)?;
            globals.set_item("from_rust_write_64bit_value", wrap_pyfunction!(from_rust_write_64bit_value, py)?)?;
            globals.set_item("from_rust_write_8bit_value", wrap_pyfunction!(from_rust_write_8bit_value, py)?)?;
            globals.set_item("sleep", wrap_pyfunction!(sleep, py)?)?;
            globals.set_item("input", wrap_pyfunction!(input, py)?)?;
            globals.set_item("attach",wrap_pyfunction!(inject,py)?)?;
            globals.set_item("dettach", wrap_pyfunction!(un_inject,py)?)?;
            globals.set_item("force_continue",wrap_pyfunction!(force_continue,py)?)?;
            globals.set_item("print_read_error",wrap_pyfunction!(print_read_error,py)?)?;
            py.run(&compiled_scripts.internal_script,
                Some(globals),
                None
            )?;
            Ok(())
        })
        
    }





}

//end of modules
//EOF
