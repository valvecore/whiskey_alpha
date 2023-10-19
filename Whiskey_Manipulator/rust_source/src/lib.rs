
// the translation layer for the c static library
mod whiskclib{
    #[repr(C)]
    pub struct FromAddress{
        pub error_code:u8,
        pub data:u8,
        pub data32:u32,
        pub data64:u64,
    }
    
    extern "C"{

        pub fn attach_process(pid:usize)->u8;
        pub fn dettach_process(pid:usize)->u8;
        pub fn write_address8(pid:usize,address:usize,data:u8)->u8;
        pub fn read_address8(pid:usize,address:usize)->FromAddress;
        pub fn read_address32little(pid:usize,address:usize)->FromAddress;
        pub fn write_address32little(pid:usize,address:usize,data:u32)->u8;
        pub fn read_address64little(pid:usize,address:usize)->FromAddress;
        pub fn write_address64little(pid:usize,address:usize,data:u64)->u8;
        pub fn force_continue(pid:usize);
        pub fn read_error();
    }
       

}
// end of translation layer
//ERROR_TYPES
pub mod error{
    pub mod attach{
        pub const INVALID_PERMISSIONS:&str="could not attach, invalid permissions.";
        pub const INVALID_PERMISSIONS_ERRORCODE:u8=2;
        pub const NO_SUCH_PROCESS:&str="could not attach, no such process.";
        pub const NO_SUCH_PROCESS_ERRORCODE:u8=3;
        pub const COULD_NOT_ATTACH:&str="could not attach.";
        pub const COULD_NOT_ATTACH_ERRORCODE:u8=1;
        pub const INVALID_BYTEORDER:&str="invalid byte order.";
        pub const SUCCESS_ERRORCODE:u8=0;
    }
    pub mod dettach{
        pub const SUCCESS_ERRORCODE:u8=0;
    }
    pub mod write_address{
        pub const INVALID_ADDRESS:&str="could not write to address,invalid address.";
        pub const INVALID_ADDRESS_ERRORCODE:u8=1;
        pub const SUCCESS_ERRORCODE:u8=0;
    }
    pub mod read_address{
        pub const INVALID_ADDRESS:&str="could not read address,invalid address.";
        pub const INVALID_ADDRESS_ERRORCODE:u8=1;
        pub const SUCCESS_ERRORCODE:u8=0;

    }
    pub const POSSIBLE_CORRUPTION:&str="this program has run improperly, possible corruption.";
}
pub mod byteorder{
    pub const LITTLE_ENDIAN:u8=0;
    pub const BIG_ENDIAN:u8=1;

}



//END OF ERROR_TYPES

//STRUCTS
pub struct WindowsProcess{
    pub pid:usize,
    pub attached:bool,
    pub byteorder:u8
}
//END OF STRUCTS




//Functions
    
    

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



// prints out the read error 
pub fn read_error() {
    
    use whiskclib::*;
    
    unsafe{
        read_error();
    }
}
/*
this function attaches to the process, note: You should only attach one program. This must be run in root.
*/
pub fn attach(set_pid:usize,byte_order:u8)->Result<WindowsProcess,&'static str>{
    use error::attach::*;
    use error::*;
    use whiskclib::*;
    

    match byte_order{
        byteorder::LITTLE_ENDIAN=>{},
        byteorder::BIG_ENDIAN=>{},
        _=>return Err(INVALID_BYTEORDER)

    }
    match unsafe{attach_process(set_pid)}{
        COULD_NOT_ATTACH_ERRORCODE=>return Err(COULD_NOT_ATTACH),
        INVALID_PERMISSIONS_ERRORCODE=>return Err(INVALID_PERMISSIONS),
        NO_SUCH_PROCESS_ERRORCODE=>return Err(NO_SUCH_PROCESS),
        SUCCESS_ERRORCODE=>{},
        _=>panic!("{}",POSSIBLE_CORRUPTION),
    }
    return Ok(WindowsProcess{
        pid:set_pid,
        attached:true,
        byteorder:byte_order
    });

}
/*
this function dettaches a process, you must dettach a process before you attach another process or end the program.
*/
pub fn dettach(windows_process:&WindowsProcess)->Result<WindowsProcess,&'static str>{
    use error::dettach::*;
    use error::*;
    use whiskclib::*;

    match unsafe{dettach_process(windows_process.pid)}{
        SUCCESS_ERRORCODE=>{},
        _=>panic!("{}",POSSIBLE_CORRUPTION)
    }

    return Ok(WindowsProcess{
        pid:windows_process.pid,
        attached:false,
        byteorder:windows_process.byteorder
    });
}
/*
this functions writes a byte to an memory address of the process.
*/
pub fn write_8bit_address(windows_process:&WindowsProcess,address:usize,data:u8)->Result<(),&'static str>{
    use error::write_address::*;
    use error::*;
    use whiskclib::*;

    match unsafe{write_address8(windows_process.pid,address,data)}{
        INVALID_ADDRESS_ERRORCODE=>return Err(INVALID_ADDRESS),
        SUCCESS_ERRORCODE=>return Ok(()),
        _=>panic!("{}",POSSIBLE_CORRUPTION)

    }
}
/*
this functions writes a byte to an memory address of the process.
*/
pub fn write_32bit_address(windows_process:&WindowsProcess,address:usize,data:u32)->Result<(),&'static str>{
    use error::write_address::*;
    use error::*;   
    use whiskclib::*;

    match unsafe{write_address32little(windows_process.pid,address,data)}{
        INVALID_ADDRESS_ERRORCODE=>return Err(INVALID_ADDRESS),
        SUCCESS_ERRORCODE=>return Ok(()),
        _=>panic!("{}",POSSIBLE_CORRUPTION)

    }
}
/*
this functions writes a byte to an memory address of the process.
*/
pub fn write_64bit_address(windows_process:&WindowsProcess,address:usize,data:u64)->Result<(),&'static str>{
    use error::write_address::*;
    use error::*;   
    use whiskclib::*;

    match unsafe{write_address64little(windows_process.pid,address,data)}{
        INVALID_ADDRESS_ERRORCODE=>return Err(INVALID_ADDRESS),
        SUCCESS_ERRORCODE=>return Ok(()),
        _=>panic!("{}",POSSIBLE_CORRUPTION)

    }
}
/*
this functions reads the address and returns it through the result
*/
pub fn read_8bit_address(windows_process:&WindowsProcess,address:usize)->Result<u8,&'static str>{
    use error::read_address::*;
    use error::*;
    use whiskclib::*;

    
    let address_struct:FromAddress=unsafe{ read_address8(windows_process.pid,address) };

    match address_struct.error_code{
        INVALID_ADDRESS_ERRORCODE=>return Err(INVALID_ADDRESS),
        SUCCESS_ERRORCODE=>return Ok(address_struct.data),
        _=>panic!("{}",POSSIBLE_CORRUPTION)

    }
}
/*
this functions reads the address and returns it through the result
*/
pub fn read_32bit_address(windows_process:&WindowsProcess,address:usize)->Result<u32,&'static str>{
    use error::read_address::*;
    use error::*;
    use whiskclib::*;
    
    //println!("{:x}",address);

    let address_struct:FromAddress=unsafe{ 

        read_address32little(windows_process.pid,address) 

    };
    //println!("got {}",address_struct.data32);

    match address_struct.error_code{
        INVALID_ADDRESS_ERRORCODE=>return Err(INVALID_ADDRESS),
        SUCCESS_ERRORCODE=>return Ok(address_struct.data32),
        _=>panic!("{}",POSSIBLE_CORRUPTION)

    }
    
}
/*
this functions reads the address and returns it through the result
*/
pub fn read_64bit_address(windows_process:&WindowsProcess,address:usize)->Result<u64,&'static str>{
    use error::read_address::*;
    use error::*;
    use whiskclib::*;

    let address_struct:FromAddress=unsafe{ 

        read_address64little(windows_process.pid,address) 

    };
    
    println!("### {}",address_struct.data64);   

    match address_struct.error_code{
        INVALID_ADDRESS_ERRORCODE=>return Err(INVALID_ADDRESS),
        SUCCESS_ERRORCODE=>return Ok(address_struct.data64),
        _=>panic!("{}",POSSIBLE_CORRUPTION)

    }
    
}
//forces the process to continue 
pub fn force_continue(windows_process:&WindowsProcess){
    
    use whiskclib::*;
    
    unsafe{
        force_continue(windows_process.pid);
    }
}
//END OF FUNCTIONS






