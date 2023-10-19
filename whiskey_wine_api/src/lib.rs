




//GLOBAL VARIABLES



//global variables for the construcing of the whiskey wine data, change if needed






const EXE_PATH_JSON_KEY:&'static str = "EXE_PATH";
const USER_JSON_KEY:&'static str = "USER";
const PROCESS_TIMEOUT_JSON_KEY:&'static str = "PROCESS_TIMEOUT";
const WHISKEY_WINE_DIR_NAME:&'static str="WHISKEY_WINE_DONOTEDIT";
const WHISKEY_JSON_PATHS_FILE_NAME:&'static str = "path_data.json";
const WHISKEY_WINE_SHELL_START_FILE_NAME:&'static str="start_main_exe.sh";
const FIND_PID:&'static str="";
const PID_KEY:[char;4]=['P','I','D','{'];
const BASH_FORMAT:&'static str="#!/bin/bash\n";
const RUN_USER:&'static str = "runuser -u ";
const WINE_RUN_COMMAND_FIRST:&'static str = " -- bash -c 'source /home/";
const WINE_RUN_COMMAND_SECOND:&'static str = "/.bashrc;source /home/tony/.profile printenv; wine start /unix ";
const WINE_RUN_COMMAND_ROOT:&'static str = "VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/intel_icd.x86_64.json\necho $USER\n export DISPLAY=:0\necho $DISPLAY \nwine start /unix ";
const WINE_RUN_COMMAND_END_PART:&'static str = "'";
const FIND_PROCESS_TIMEOUT:usize = 5000 as usize;
//END OF GLOBAL VARIABLES

//STRUCTS
pub struct WindowsProcess{
    pub path:String,
    pub whiskey_files_path:String,
    pub pid:usize,
    pub running:bool,
    pub std_output:Option<std::process::Output>,
    pub user:String,
    pub process_name:Option<String>,
    pub process_timeout:usize

    

}
//END OF STRUCTS
//IMPLS


impl WindowsProcess{
    
    //runs the exe
    pub fn run(&mut self)->Result<(),std::io::Error>{
        use std::io::Error;
        use std::io::ErrorKind::*;
        use general_functions::*;
        
        match self.running{
            true=>return Err(Error::new(AlreadyExists,"process is already running")),
            false=>{}
        }

        let process_name:String;

        if self.process_name == None {
            process_name = get_end_of_path(&self.path);

        }

        else {

           process_name = self.process_name.clone().unwrap();
        
        }

        if check_if_process_running_un(&process_name)  {
            
            return Err(
                Error::new(
                    AlreadyExists,
                    "process is already running"))

        }
        
       

        

        
        
        self.pid=
            run_wine_start_shell_script(&self.whiskey_files_path,
                                        &process_name,
                                        self.process_timeout
                                        )?;

        return Ok(());

            
    }
    //returns true if the process is alive, false if it isnt
    pub fn alive(&mut self)->bool {
        
        use general_functions::*;                                           

    
        self.running = check_if_process_running_up(self.pid);
        
        return self.running;
        


        

    }
    //changes the path to the exe file 
    pub fn change_exe_path(&mut self,path:&str)->Result<(),std::io::Error> {
        
        use general_functions::*;
        
        check_if_exe_exists(path)?;

        self.path = path.to_string();
    
        write_exe_path_data_json_file(
            &self.whiskey_files_path,
            path)
            .unwrap();
        
        write_start_shell_file(&self.whiskey_files_path
                               , &construct_shell_wine_start_file(&self.path,&self.user)).unwrap();
        
        return Ok(());
        
    }


}



//small functions used by the library to make the code more understandable
pub mod general_functions{
    use crate::{WHISKEY_WINE_SHELL_START_FILE_NAME, PID_KEY, WHISKEY_JSON_PATHS_FILE_NAME};

    

    
   

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
    //gets the absolute path of a string 
    pub fn get_absolute_path(path:&str)->Result<String, std::io::Error>{
        
        use std::fs::canonicalize;
        use std::path::PathBuf;
        use std::io::Error;
        use std::io::ErrorKind;

        let current_path: PathBuf = PathBuf::from(path);

        
        let canonicalize_binding = canonicalize(current_path)?;

        let absolute_path:Option<&str> =  canonicalize_binding.to_str();

        if absolute_path == None {
            
            return Err(
                Error::new(
                    ErrorKind::InvalidData, 
                    "the path doesnt exist"
                           )
                );

        }
        
        return Ok(
            absolute_path.unwrap()
            .to_string()
            )

    }
    //adds the whiskey wine directory name to the provided path str
    pub fn add_whiskey_files_dir_to_path(path:&str)->String{
        
        use super::*;

        let current_path:String = 
            check_then_add_slash_to_path(
                path,
                check_for_slash_type(path)) + WHISKEY_WINE_DIR_NAME;

        return current_path;

    }
    
    //adds the paths.json file to path 
    pub fn add_whiskey_json_file_to_path(path:&str)->String{
        
        use super::*;

        let current_path:String = 
            check_then_add_slash_to_path(
                path,
                check_for_slash_type(path)) + WHISKEY_JSON_PATHS_FILE_NAME;

        return current_path;

    }

    //spawns the paths json file
    pub fn spawn_paths_json_file(path:&str)->Result<(),std::io::Error> {
        
        use std::fs::File;
        
        let json_file_path:String = add_whiskey_json_file_to_path(path);
        
        File::create(json_file_path)?;
            
        return Ok(());
    }

    //writes exe path data to paths json file 
    pub fn write_exe_path_data_json_file(path:&str,exe_path:&str)->Result<(),std::io::Error>{
        
        use serde_json::{Map, Value,to_string};
        use std::fs::*;
        use super::*;

        let mut json_data:Map<String,Value>=Map::new();
        
        json_data.insert(EXE_PATH_JSON_KEY.to_string(), exe_path.into());

        let json_file:String = add_whiskey_json_file_to_path(path);

        write(json_file,&to_string(&json_data)?)?;

        return Ok(());
    }

    //writes user data to paths json file 
    pub fn write_user_data_json_file(path:&str,user:&str)->Result<(),std::io::Error>{
        
        use serde_json::{Map, Value,to_string};
        use std::fs::*;
        use super::*;

        let mut json_data:Map<String,Value>=Map::new();
        
        json_data.insert(USER_JSON_KEY.to_string(), user.into());

        let json_file:String = add_whiskey_json_file_to_path(path);

        write(json_file,&to_string(&json_data)?)?;

        return Ok(());
    }

    //writes process timeout data to paths json file 
    pub fn write_process_timeout_data_json_file(path:&str,process_timeout:usize)->Result<(),std::io::Error>{
        
        use serde_json::{Map, Value,to_string};
        use std::fs::*;
        use super::*;

        let mut json_data:Map<String,Value>=Map::new();
        
        json_data.insert(USER_JSON_KEY.to_string(), process_timeout.into());

        let json_file:String = add_whiskey_json_file_to_path(path);

        write(json_file,&to_string(&json_data)?)?;

        return Ok(());
    }

    //creates the json paths file and writes the exe path to it
    pub fn create_then_write_json_paths_file(path:&str,exe_path:&str,user:&str)->Result<(),std::io::Error>{
        
        spawn_paths_json_file(path)?;

        write_exe_path_data_json_file(path, exe_path)?;
        
        write_user_data_json_file(path,user)?;

        return Ok(());
    }

    //gets the pid out of a string
    pub fn get_pid_from_string(string:&str)->Result<usize,std::io::Error>{

        use std::io::Error;
        use std::io::ErrorKind::*;

        if string.contains(PID_KEY) != true{
            return Err(Error::new(
                    NotFound, 
                    "pid key not inside string"));
        }

        let mut buffer: usize = 1;
        let mut pid_string: String = String::new();
        let mut read_key: bool = false;
        let mut read_pid: bool = false;

        for character in string.chars(){
            
            if read_key == false && read_pid == false {
                
                if character == PID_KEY[0] {
                    
                    read_key = true;

                }
    
            }

            if read_key {
                
                if character == PID_KEY[ buffer ] {
                    
                    buffer+=1;



                }
                

                

            }

            if read_pid {
                
                if character.is_numeric() != true {
                    
                    break;
                }
                
                pid_string+=&character.to_string();
            }

            if buffer == PID_KEY.len(){

                read_key = false;
                
                read_pid = true;

            }

        }
        
        return Ok(pid_string.parse().unwrap());
    }
    //constructs the start shell script, with the path parameter to point to the exe
    pub fn construct_shell_wine_start_file(path:&str,user:&str)->String{
        use super::*;

        if user == "root" {
            
                let mut script:String=format!("{}{}{}",BASH_FORMAT,
                                      WINE_RUN_COMMAND_ROOT,
                                      path);
                
                script=script+FIND_PID;

                return script;
                

        }

       let mut script:String=format!("{}{}{}{}{}{}{}{}",BASH_FORMAT,
                                      RUN_USER,
                                      user,
                                      WINE_RUN_COMMAND_FIRST,
                                      user,
                                      WINE_RUN_COMMAND_SECOND,
                                      path,
                                      WINE_RUN_COMMAND_END_PART);
        

        //let mut script:String=format!("{}{}",BASH_FORMAT,path);
        //script=script+FIND_PID;
        return script;

    }

    /*
    adds a slash at the end of the path if there inst, the first parameter is the path and second is a bool to tell the function 
    weither it should be a foward slash or a back slash 
    ||true for forward slash false for backwards slash||

    */
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
    //this function add the directory for whiskey wine files directory to the path supplied into
    //the parameter 
    pub fn add_whiskey_wine_dir_to_path(path:&str)->String{
        return check_then_add_slash_to_path(&path, check_for_slash_type(&path))+super::WHISKEY_WINE_DIR_NAME;
    }
    //this function adds the file for the whiskey wine api into the supplied path
    pub fn add_whiskey_wine_shell_start_file(path:&str)->String{
        return check_then_add_slash_to_path(&path, check_for_slash_type(&path))+super::WHISKEY_WINE_SHELL_START_FILE_NAME;
    }
    //spawns a file, the file path should be in the first parameter, and the second parameter
    //should be the data to write to the file
    pub fn create_then_write_shell_file(path:&str,contents:&str)->Result<(),std::io::Error>{
        
        

        create_shell_file(path)?;
        
        write_shell_file(path, contents)?;

        return Ok(());
    
    }
    //creates the shell file
    pub fn create_shell_file(path:&str)->Result<(),std::io::Error>{
        use std::fs::File;
        

        File::create(path)?;
        return Ok(());
    
    }
    //writes data to the shell file 
    pub fn write_shell_file(path:&str,contents:&str)->Result<(),std::io::Error>{
        use std::io::Write;
        use std::fs::File;
        

        File::create(path)?.write_all(contents.as_bytes())?;
        return Ok(());
    
    }
    //writes data to the start shell file 
    pub fn write_start_shell_file(path:&str,contents:&str)->Result<(),std::io::Error>{
        use std::io::Write;
        use std::fs::File;
        
        let start_shell_file_path:String = add_whiskey_wine_shell_start_file(path);

        File::create(start_shell_file_path)?.write_all(contents.as_bytes())?;
        return Ok(());
    
    }
    //runs the wine start shell script and returns the pid of the process
    pub fn run_wine_start_shell_script(whiskey_wine_files_dir:&str, process_name:&str,process_timeout:usize)->Result<usize,std::io::Error>{
        use std::process::Command;
        use std::process::Output;
        use std::process::Child;
        use std::thread::*;
        use std::process::Stdio;
       
        
        
        

        

        let complete_path: String = check_then_add_slash_to_path(
            whiskey_wine_files_dir,
            check_for_slash_type(
                whiskey_wine_files_dir)
            )+WHISKEY_WINE_SHELL_START_FILE_NAME;

        

      
            
    //Command::new("bash")
        //.arg("-c")
      //  .arg(complete_path)
        //.spawn()
        //.expect("Failed to execute command"); 

    let  mut child:Child = Command::new("bash")
            .arg(complete_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");
            
        



        
       
        

        
        
        return Ok(
            get_pid_from_name(process_name,process_timeout)?
            );

    }
    //check if the whiskey wine files dir exists in the path
    pub fn check_for_whiskey_wine_dir(path:&str)->bool{
        
        use std::path::Path;

        let whiskey_wine_dir_path:String = add_whiskey_wine_dir_to_path(path);

        return Path::new(&whiskey_wine_dir_path).exists();

    }
    //returns the file extension of a file declared in a string
    pub fn get_file_extension(text:&str)->Result<String,std::io::Error>{
        use std::io::Error;
        use std::io::ErrorKind::*;

        let mut buffer:usize = text.len()-1;
        let mut buffer_count:usize = 0; 

        let text_in_chars:Vec<char> = text.chars().collect();

        let mut string_buffer = String::new();
        
        let mut rearranged_string_buffer = String::new();

        let mut extension_found:bool = false; 

        loop{ 

            if text_in_chars[buffer] == '.'{

                extension_found=true;
                break;

            }

            string_buffer.push(text_in_chars[buffer]);
            buffer-=1;
            
        }
       

        if buffer_count == 0 || extension_found == false{
            
            return Err(
                Error::new(InvalidData
                           , "file or path text not properly structured and couldnt find file extension")
                ); 

        }

        return Ok(
            string_buffer.chars()
            .rev()
            .collect()
            );
        
        


    }

    //returns the file extension of a file declared in a string but the string is not flipped back
    //to the right string, and therefore may not return the correct extension if you use this
    // this is mainly there to take an .exe extension out of a path because exe is the same
    // reveresed and it saves some time when not un reversing it
    pub fn get_file_extension_notflipped(text:&str)->Result<String,std::io::Error>{
        use std::io::Error;
        use std::io::ErrorKind::*;

        let mut buffer:usize = text.len()-1;
        let mut buffer_count:usize = 0; 

        let text_in_chars:Vec<char> = text.chars().collect();

        let mut string_buffer = String::new();
        
        let mut extension_found:bool = false; 
        
        loop{ 

            if text_in_chars[buffer] == '.'{

                extension_found=true;
                break;

            }

            string_buffer.push(text_in_chars[buffer]);
            buffer-=1;
            buffer_count+=1;
            
        }
       

        if buffer_count == 0 || extension_found == false{
            
            return Err(
                Error::new(InvalidData
                           , "file or path text not properly structured and couldnt find file extension")
                ); 

        }

        return Ok(
            string_buffer);
        
        


    }
    /*
    returns a true if the json paths file in the whiskey wine dir appointed to in the path parameter exists. false if it doesnt.
    the path parameter must contain the path to the whiskey wine directory you want to check if
    it has the json paths file
    */
    pub fn check_for_json_paths_file(path:&str)->bool{
        
        use std::path::Path;

        let json_paths_file_dir:String = add_whiskey_json_file_to_path(path);

        if Path::new(&json_paths_file_dir).is_file() != true{

            return false;

        }

        return true;
    }

    /*
    returns a true if the start shell file file in the whiskey wine dir appointed to in the path parameter exists. false if it doesnt.
    the path parameter must contain the path to the whiskey wine directory you want to check if
    it has the start shell file
    */
    pub fn check_for_start_shell_file(path:&str)->bool{
        
        use std::path::Path;
        
        let shell_paths_file_dir:String = add_whiskey_wine_shell_start_file(path);

        if Path::new(&shell_paths_file_dir).is_file() != true{

            return false;

        }

        return true;
    }

    /*
    returns true if the key exists, returns false if it doesnt .
    */
    pub fn check_if_key_in_json_file(file_path:&str,key:&str)->Result<bool,std::io::Error>{
        
        use std::fs::File;
        use serde_json::{
            from_reader
            ,Value};
        
        let file: File = File::open(file_path)?;
        let json: Value = from_reader(file)?;

        if json.get(key) == None{ 
            return Ok(false);   
        }

        return Ok(true);
        
                    

    }

    /*
    returns true if the exe path key exists in paths json file, returns false if it doesnt .
    */
    pub fn check_if_exe_path_in_json_paths_file(whiskey_wine_dir_path:&str)->Result<bool,std::io::Error>{
        
        use super::EXE_PATH_JSON_KEY;
        
        let json_file_path=
            add_whiskey_json_file_to_path(
                whiskey_wine_dir_path
                );
        
        
        return Ok(
            check_if_key_in_json_file(
                &json_file_path, 
                EXE_PATH_JSON_KEY)?
            );
        
                    

    }
    /*
    returns true if the process timeout key exists in paths json file, returns false if it doesnt .
    */
    pub fn check_if_process_timeout_in_json_paths_file(whiskey_wine_dir_path:&str)->Result<bool,std::io::Error>{
        
        use super::PROCESS_TIMEOUT_JSON_KEY;
        
        let json_file_path=
            add_whiskey_json_file_to_path(
                whiskey_wine_dir_path
                );
        
        
        return Ok(
            check_if_key_in_json_file(
                &json_file_path, 
                PROCESS_TIMEOUT_JSON_KEY)?
            );
        
                    

    }
    /* 
    returns the exe path from paths json file 
    */
    pub fn get_exe_path_from_paths_json_file(whiskey_wine_dir_path:&str)->Result<String,std::io::Error>{
        
        use std::fs::File;
        use serde_json::{
            from_reader,
            Value
        };
        use std::io::Error;
        use std::io::ErrorKind::*;
        
        let json_file_path=
            add_whiskey_json_file_to_path(
                whiskey_wine_dir_path
                );
            
        use super::EXE_PATH_JSON_KEY;
        let file:File = File::open(json_file_path)?;
        let json_data:Value = from_reader(file)?;

        //this if statement was from chatgpt, im going to keep this in here for the time being
        //until i properly figure out how serde_json works
        if let Some(exe_path) = json_data.get(EXE_PATH_JSON_KEY).and_then(Value::as_str) { 
                                                                                           
            return Ok(exe_path.to_string());
        }

        return Err(
            Error::new(
                InvalidData,
                format!("{} is not a valid string",EXE_PATH_JSON_KEY)
                )
            );
    }

    /* 
    returns the user from paths json file 
    */
    pub fn get_user_from_paths_json_file(whiskey_wine_dir_path:&str)->Result<String,std::io::Error>{
        
        use std::fs::File;
        use serde_json::{
            from_reader,
            Value
        };
        use std::io::Error;
        use std::io::ErrorKind::*;
        
        let json_file_path=
            add_whiskey_json_file_to_path(
                whiskey_wine_dir_path
                );
            
        use super::USER_JSON_KEY;
        let file:File = File::open(json_file_path)?;
        let json_data:Value = from_reader(file)?;

        //this if statement was from chatgpt, im going to keep this in here for the time being
        //until i properly figure out how serde_json works
        if let Some(exe_path) = json_data.get(USER_JSON_KEY).and_then(Value::as_str) { 
                                                                                           
            return Ok(exe_path.to_string());
        }

        return Err(
            Error::new(
                InvalidData,
                format!("{} is not a valid string",USER_JSON_KEY)
                )
            );
    }

    /* 
    returns the process timeout from paths json file 
    */
    pub fn get_process_timeout_from_paths_json_file(whiskey_wine_dir_path:&str)->Result<usize,std::io::Error>{
        
        use std::fs::File;
        use serde_json::{
            from_reader,
            Value
        };
        use std::io::Error;
        use std::io::ErrorKind::*;
        
        let json_file_path=
            add_whiskey_json_file_to_path(
                whiskey_wine_dir_path
                );
            
        use super::PROCESS_TIMEOUT_JSON_KEY;
        let file:File = File::open(json_file_path)?;
        let json_data:Value = from_reader(file)?;

        //this if statement was from chatgpt, im going to keep this in here for the time being
        //until i properly figure out how serde_json works
        if let Some(process_timeout) = json_data.get(PROCESS_TIMEOUT_JSON_KEY).and_then(Value::as_str) { 
                                                                                           
            return Ok(process_timeout.parse().unwrap());
        }

        return Err(
            Error::new(
                InvalidData,
                format!("{} is not a valid string",PROCESS_TIMEOUT_JSON_KEY)
                )
            );
    }

    /* 
    returns true if the process is alive, returns false if it isnt 
    */ 
    pub fn check_if_process_running_up(pid:usize)->bool{
        
        
        use sysinfo::{ProcessExt, System, SystemExt};

        let system = System::new_all();

        for (_,process) in system.processes(){
            if pid == <sysinfo::Pid as Into<usize>>::into(process.pid()) {
                return true;
            }
        }
        return false;

    }
    /* 
    returns true if the process is alive, returns false if it isnt. takes process name instead of pid 
    */ 
    pub fn check_if_process_running_un(process_name:&str)->bool{
        
        use sysinfo::{System, SystemExt};

        let system = System::new_all();
        
        

        for _ in system.processes_by_exact_name(process_name){
            
            return true;
        }

        return false;

    }
    
    /*
    returns what a path is pointing to 
    */
    pub fn get_end_of_path(path: &str)->String{
        
        let mut size_buffer: usize = path.len() - 1;
        let path_bytes: &[u8] = path.as_bytes();
        let mut parsed_string: String = String::new(); 

        while size_buffer != 0 {

            if path_bytes[size_buffer] == '\\' as u8 || path_bytes[size_buffer] == '/' as u8 {
                
                break;

            }

            parsed_string.push(
                path_bytes[size_buffer] as char

                );

            size_buffer -= 1;

        }
        
        return parsed_string.chars().rev().collect();

    }
    /*
     gets the pid of the process by the process name
    */
    pub fn get_pid_from_name(process_name:&str,process_timeout:usize)->Result<usize, std::io::Error> {
        
        use sysinfo::{ProcessExt, System, SystemExt};
        use std::io::Error;
        use std::io::ErrorKind::*;
        use std::time::Duration;
        use std::thread::sleep;
    

        let mut system = System::new_all();
        

        

        for i in 0..process_timeout {
            
            
            system.refresh_all();

            for (process_id,process) in system.processes(){
                
                
                
                if process.name() == process_name {
                    
                    
                    return Ok(
                        process.pid().into()
                        );
                    
                }

            }

            sleep(Duration::from_millis(1))

        }
        
        return Err(
            Error::new(
                Other,
                "the windows process did not start correctly"
                )
        );
            
        

        

    }
    //checks if the exe file exists or not, doesnt return bool but error 
    pub fn check_if_exe_exists(exe_path:&str)->Result<(),std::io::Error> {
        
        use std::io::ErrorKind::*;
        use std::io::Error;
        use std::path::Path;    

        if Path::new(exe_path).exists() != true{       
        
            return Err(
                    Error::new(NotFound,
                           "the path to the exe file is not found"

                        )
                );

        }
    
    
        if Path::new(exe_path).is_dir() == true {
    
            return Err(
                    Error::new(InvalidData,
                           "the path to the exe is not a exe file, but the path to the file is a directory"

                        )
                );
            

        }

        return Ok(());
    }

    //checks if the user exists
    pub fn check_if_user_exists(username: &str) -> bool {
        
        use std::process::Command;

        let output = Command::new("id")
            .arg("-u")
            .arg(username)
            .output()
            .expect("Failed to execute command");

        output.status.success()
    }
    
    
}


//end of modules


//FUNCTIONS
//constructs and builds the windows process and whiskey wine data for it to be run, the path
//parameter should be the path to the exe and spawn_path should be to the path you want the whiskey
//wine files to spawn
pub fn define_process(spawn_path:&str,exe_path:&str,user:&str,process_name:Option<&str>,process_timeout:usize)->Result<WindowsProcess,std::io::Error>{
    use general_functions::*;
    use std::path::*;
    use std::fs::*;
    use std::io::Error;
    use std::io::ErrorKind::*;

    let path:&str = &get_absolute_path(exe_path)?;
    
    //let path:&str = exe_path;

    

    check_if_exe_exists(path)?;
    
    if get_file_extension_notflipped(path)? != "exe"{
        
        
        return Err(
                Error::new(InvalidData,
                           "the provided file does not have the .exe extension, this is to make sure you dont try to run the wrong program. if it is an windows execution file please give it the .exe file extension. "

                    )
            );

    }
    let current_whiskey_wine_dir: String=add_whiskey_wine_dir_to_path(spawn_path);
    
    println!("{}",current_whiskey_wine_dir);

    let current_start_wine_shell_script_dir: String=add_whiskey_wine_shell_start_file(&current_whiskey_wine_dir);


    if Path::new(&current_whiskey_wine_dir).is_dir(){
        remove_dir_all(&current_whiskey_wine_dir)?;
        
    }

    create_dir(&current_whiskey_wine_dir)?;
    
    
    create_then_write_json_paths_file(&current_whiskey_wine_dir,path,user)?;
    

    create_then_write_shell_file(
        &current_start_wine_shell_script_dir, 
        &construct_shell_wine_start_file(&path,&user))?;
    
    if !check_if_user_exists(user){

        return Err(
            Error::new(
                InvalidData,
                "user doesnt exist")
            );

    }

    let process_option:Option<String>;

    
    if process_name == None {
        
        process_option = None;

    }

    else{ 

        process_option = Some(
            process_name.unwrap().to_string()
            );
        
    }
    

    
    return Ok(WindowsProcess { 
        path:path.to_string(), whiskey_files_path:current_whiskey_wine_dir.to_string() , 
        pid:0, running:false, 
        std_output:None,
        user:String::from(user),
        process_name:process_option,
        process_timeout:process_timeout
    });
    

}

//this function reuses the whiskey wine files instead of deleting them and making new files
pub fn define_reusable_process(path:&str,process_name:Option<&str>)->Result<WindowsProcess,std::io::Error>{
    
    use general_functions::*;
    use std::path::*;
    use std::io::Error;
    use std::io::ErrorKind::*;

    let current_whiskey_wine_dir: String=add_whiskey_wine_dir_to_path(path);

    if Path::new(&current_whiskey_wine_dir).exists() == false {
        
        return Err(
                Error::new(InvalidData,
                           "the whiskey wine api compiled directory with its files does not exist"

                    )
            );
        
    }

    if Path::new(&current_whiskey_wine_dir).is_file() {
        
        return Err(
                Error::new(InvalidData,
                           "the whiskey wine api compiled directory is a file."

                    )
            );
    
    }

    let current_start_wine_shell_script_dir: String=
        add_whiskey_wine_shell_start_file(&current_whiskey_wine_dir);
    
    if Path::new(&current_whiskey_wine_dir).is_file() {
        
        return Err(
                Error::new(NotFound,
                           "could not find the start wine shell script"

                    )
            );
    
    }
    
    

    

    if check_for_json_paths_file(&current_whiskey_wine_dir) != true {

        return Err(
                Error::new(NotFound,
                           "could not find the start wine shell script"

                    )
            );

    }

    if check_for_start_shell_file(&current_whiskey_wine_dir) != true {

        return Err(
                Error::new(NotFound,
                           "could not find the start wine shell script"

                    )
            );

    }

    if check_if_exe_path_in_json_paths_file(&current_whiskey_wine_dir)? != true{
        
        return Err(
                Error::new(NotFound,
                           "exe path key not in json file"

                    )
            );

    }

    let exe_path:String = get_exe_path_from_paths_json_file(&current_whiskey_wine_dir)?;

    let user:String = get_user_from_paths_json_file(&current_whiskey_wine_dir)?;

    let process_timeout:usize = get_process_timeout_from_paths_json_file(&current_whiskey_wine_dir)?;
    
    println!("t{}",process_timeout);


    let process_option:Option<String>;

    if process_name == None {
        
        process_option = None;

    }

    else{ 

        process_option = Some(
            process_name.unwrap().to_string()
            );
        
    }



    return Ok(WindowsProcess { 
        path: exe_path,
        whiskey_files_path: current_whiskey_wine_dir.to_string() , 
        pid: 0,
        running: false, 
        std_output: None,
        user:String::from(user),
        process_name:process_option,
        process_timeout
    });

}


//this function returns a true if the whiskey wine files in path exists and can be reused
pub fn check_if_process_can_be_reused(path:&str)->bool{
    
    return general_functions::check_for_whiskey_wine_dir(path);

}



//END OF FUNCTIONS
//EOF
